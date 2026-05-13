// PlanTier is defined in common-rs and re-exported here.
// App-level convenience: default to Free for unknown strings from the DB.
pub(crate) use platform_rs::cache::PlanTier;

/// Parse a tier string from the DB, defaulting to Free for unknown values.
/// Use this instead of `.parse::<PlanTier>()` when you want a safe fallback.
pub(crate) fn parse_tier_or_free(value: &str) -> PlanTier {
    value.parse::<PlanTier>().unwrap_or(PlanTier::Free)
}

#[cfg(all(test, feature = "unit"))]
mod tests {
    use super::*;

    #[test]
    fn should_parse_all_tiers() {
        assert_eq!("free".parse::<PlanTier>().unwrap(), PlanTier::Free);
        assert_eq!("pro".parse::<PlanTier>().unwrap(), PlanTier::Pro);
        assert_eq!("plus".parse::<PlanTier>().unwrap(), PlanTier::Plus);
        assert_eq!("enterprise".parse::<PlanTier>().unwrap(), PlanTier::Enterprise);
    }

    #[test]
    fn should_default_invalid_to_free() {
        assert_eq!(parse_tier_or_free("invalid"), PlanTier::Free);
        assert_eq!(parse_tier_or_free(""), PlanTier::Free);
    }

    #[test]
    fn should_serialize_to_kebab_case() {
        assert_eq!(PlanTier::Free.to_string(), "free");
        assert_eq!(PlanTier::Pro.to_string(), "pro");
        assert_eq!(PlanTier::Plus.to_string(), "plus");
        assert_eq!(PlanTier::Enterprise.to_string(), "enterprise");
    }

    #[test]
    fn should_default_to_free() {
        assert_eq!(PlanTier::default(), PlanTier::Free);
    }
}
