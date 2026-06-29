//! Integration tests for optional-scope read/write/forget semantics (epic 0017).
//!
//! These prove the asymmetry that the unit tests (which use the in-memory store)
//! cannot prove against real backends: on a **read**, an unset `org`/`agent`
//! widens (the dimension is unconstrained, matching any value); on a **write** or
//! **forget**, an unset dimension is a concrete unscoped address. The behaviors
//! span both the Qdrant filter path (`search`) and the Postgres filter path
//! (`timeline`), which are separate idioms and could diverge.

#![cfg(feature = "integration")]

use std::time::Duration;

use memoir_core::memory::Scope;

mod common;

const INDEX_TIMEOUT: Duration = Duration::from_secs(15);

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_return_memories_across_all_orgs_when_read_org_is_none() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let user = common::fresh_user_id();

    let acme = Scope::builder().user_id(&user).org("acme").agent("a").build()?;
    let globex = Scope::builder().user_id(&user).org("globex").agent("a").build()?;
    let unscoped = Scope::builder().user_id(&user).build()?;

    let m_acme = client.remember("the deploy uses kubernetes at acme", acme.clone()).await?;
    let m_globex = client.remember("the deploy uses kubernetes at globex", globex.clone()).await?;
    let m_unscoped = client.remember("the deploy uses kubernetes generally", unscoped.clone()).await?;

    common::wait_until_indexed(&client, &m_acme.pid, &acme, "deploy kubernetes", INDEX_TIMEOUT).await?;
    common::wait_until_indexed(&client, &m_globex.pid, &globex, "deploy kubernetes", INDEX_TIMEOUT).await?;
    common::wait_until_indexed(&client, &m_unscoped.pid, &unscoped, "deploy kubernetes", INDEX_TIMEOUT).await?;

    let widened = Scope::builder().user_id(&user).build()?;
    let hits = client.search("deploy kubernetes", widened).limit(50).await?;

    let pids: Vec<&str> = hits.list().iter().map(|m| m.pid.as_str()).collect();
    assert!(pids.contains(&m_acme.pid.as_str()), "acme row must be present; got {pids:?}");
    assert!(pids.contains(&m_globex.pid.as_str()), "globex row must be present; got {pids:?}");
    assert!(pids.contains(&m_unscoped.pid.as_str()), "unscoped row must be present; got {pids:?}");

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_return_only_matching_org_when_read_org_is_some() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let user = common::fresh_user_id();

    let acme = Scope::builder().user_id(&user).org("acme").agent("a").build()?;
    let globex = Scope::builder().user_id(&user).org("globex").agent("a").build()?;

    let m_acme = client.remember("the deploy uses kubernetes at acme", acme.clone()).await?;
    let m_globex = client.remember("the deploy uses kubernetes at globex", globex.clone()).await?;

    common::wait_until_indexed(&client, &m_acme.pid, &acme, "deploy kubernetes", INDEX_TIMEOUT).await?;
    common::wait_until_indexed(&client, &m_globex.pid, &globex, "deploy kubernetes", INDEX_TIMEOUT).await?;

    let hits = client.search("deploy kubernetes", acme.clone()).limit(50).await?;

    let pids: Vec<&str> = hits.list().iter().map(|m| m.pid.as_str()).collect();
    assert!(pids.contains(&m_acme.pid.as_str()), "acme row must be present; got {pids:?}");
    assert!(
        !pids.contains(&m_globex.pid.as_str()),
        "globex row must NOT appear under an acme-scoped search; got {pids:?}"
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_not_return_other_users_memories_when_read_org_is_none() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let mine = common::fresh_user_id();
    let theirs = common::fresh_user_id();

    let my_scope = Scope::builder().user_id(&mine).build()?;
    let their_scope = Scope::builder().user_id(&theirs).org("acme").build()?;

    let m_mine = client.remember("the deploy uses kubernetes mine", my_scope.clone()).await?;
    let m_theirs = client.remember("the deploy uses kubernetes theirs", their_scope.clone()).await?;

    common::wait_until_indexed(&client, &m_mine.pid, &my_scope, "deploy kubernetes", INDEX_TIMEOUT).await?;
    common::wait_until_indexed(&client, &m_theirs.pid, &their_scope, "deploy kubernetes", INDEX_TIMEOUT).await?;

    let hits = client.search("deploy kubernetes", my_scope.clone()).limit(50).await?;

    let pids: Vec<&str> = hits.list().iter().map(|m| m.pid.as_str()).collect();
    assert!(pids.contains(&m_mine.pid.as_str()), "my row must be present; got {pids:?}");
    assert!(
        !pids.contains(&m_theirs.pid.as_str()),
        "another user's row must never appear, even at the widest read; got {pids:?}"
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_widen_timeline_across_orgs_when_read_org_is_none() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let user = common::fresh_user_id();

    let acme = Scope::builder().user_id(&user).org("acme").build()?;
    let globex = Scope::builder().user_id(&user).org("globex").build()?;

    let m_acme = client.remember("acme deploy note", acme.clone()).await?;
    let m_globex = client.remember("globex deploy note", globex.clone()).await?;

    // `timeline` is a Postgres read of the episodic rows and does not wait on the
    // async embed substrate, so unlike the search tests above it needs no
    // `wait_until_indexed` — the rows are visible immediately after `remember`.
    let widened = Scope::builder().user_id(&user).build()?;
    let events = client.timeline(widened).await?;

    let pids: Vec<&str> = events.iter().map(|m| m.pid.as_str()).collect();
    assert!(pids.contains(&m_acme.pid.as_str()), "acme row must be in widened timeline; got {pids:?}");
    assert!(pids.contains(&m_globex.pid.as_str()), "globex row must be in widened timeline; got {pids:?}");

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_forget_only_unscoped_memories_when_forget_scope_org_is_none() -> anyhow::Result<()> {
    use memoir_core::memory::ForgetTarget;

    let client = common::fresh_client().await?;
    let user = common::fresh_user_id();

    let unscoped = Scope::builder().user_id(&user).build()?;
    let scoped = Scope::builder().user_id(&user).org("acme").build()?;

    let m_unscoped = client.remember("the deploy uses kubernetes generally", unscoped.clone()).await?;
    let m_scoped = client.remember("the deploy uses kubernetes at acme", scoped.clone()).await?;

    common::wait_until_indexed(&client, &m_unscoped.pid, &unscoped, "deploy kubernetes", INDEX_TIMEOUT).await?;
    common::wait_until_indexed(&client, &m_scoped.pid, &scoped, "deploy kubernetes", INDEX_TIMEOUT).await?;

    let deleted = client.forget(ForgetTarget::Scope(unscoped.clone())).await?;
    assert!(
        deleted.contains(&m_unscoped.pid),
        "forgetting the unscoped scope must delete the unscoped row; deleted {deleted:?}"
    );
    assert!(
        !deleted.contains(&m_scoped.pid),
        "forgetting the unscoped scope must NOT delete the org-scoped row; deleted {deleted:?}"
    );

    let survivors = client.search("deploy kubernetes", scoped.clone()).limit(50).await?;
    let survivor_pids: Vec<&str> = survivors.list().iter().map(|m| m.pid.as_str()).collect();
    assert!(
        survivor_pids.contains(&m_scoped.pid.as_str()),
        "the org-scoped row must survive the unscoped forget; got {survivor_pids:?}"
    );

    Ok(())
}
