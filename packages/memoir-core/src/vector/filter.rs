//! Caller-supplied metadata filter applied alongside scope at search time.
//!
//! [`MemoryFilter`] mirrors the shape of Qdrant's payload `Filter` —
//! `must` / `must_not` / `should` over a list of [`FilterCondition`]s — but
//! keeps the public type owned by memoir-core. The internal translation to
//! `qdrant_client::qdrant::Filter` happens at the [`crate::vector::qdrant`]
//! boundary; consumers never see the Qdrant types directly.
//!
//! Conditions translate one-to-one to Qdrant's `FieldCondition` primitives:
//! `Equals` becomes a `Match::Keyword`/`Match::Integer`, `In` becomes a
//! `Match::Keywords`/`Match::Integers`, and `Range` maps to Qdrant's
//! numeric `Range`.

use qdrant_client::qdrant as qd;

/// Caller-supplied metadata filter applied during [`crate::client::Client::search`].
///
/// Conditions in `must` are AND-joined; conditions in `must_not` reject any
/// match; conditions in `should` are OR-joined. Empty sections are inert.
/// The scope tuple (`agent_id` / `org_id` / `user_id`) is always enforced
/// independently by the index — caller-supplied conditions cannot widen scope.
///
/// # Examples
///
/// ```
/// use memoir_core::vector::{FilterCondition, MemoryFilter, MatchValue};
///
/// // Exclude memories whose `conversation_id` metadata field equals 42.
/// let filter = MemoryFilter {
///     must_not: vec![FilterCondition::Equals {
///         field: "conversation_id".to_string(),
///         value: MatchValue::Integer(42),
///     }],
///     ..MemoryFilter::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq)]
pub struct MemoryFilter {
    /// All conditions must match.
    pub must: Vec<FilterCondition>,
    /// No condition may match (AND-NOT).
    pub must_not: Vec<FilterCondition>,
    /// At least one condition must match (OR).
    pub should: Vec<FilterCondition>,
}

impl MemoryFilter {
    /// Returns `true` when every section is empty — the filter is inert.
    pub fn is_empty(&self) -> bool {
        self.must.is_empty() && self.must_not.is_empty() && self.should.is_empty()
    }
}

/// One field-targeted condition inside a [`MemoryFilter`].
#[derive(Debug, Clone, PartialEq)]
pub enum FilterCondition {
    /// Field equals a single keyword, integer, or boolean value.
    Equals { field: String, value: MatchValue },
    /// Field equals any value in the list — `IN (...)` semantics.
    In { field: String, values: MatchValues },
    /// Field is a number within the half-open or closed range.
    Range { field: String, range: NumericRange },
}

/// Concrete value compared against a payload field.
///
/// Qdrant accepts string keywords, integers, and booleans for equality
/// matching. Text-search variants (`matches_text`, `matches_phrase`) are
/// not exposed in v0.1 — only exact match.
#[derive(Debug, Clone, PartialEq)]
pub enum MatchValue {
    /// String value compared with exact equality.
    Keyword(String),
    /// 64-bit integer.
    Integer(i64),
    /// Boolean value.
    Bool(bool),
}

/// Value list backing `IN (...)` semantics for [`FilterCondition::In`].
#[derive(Debug, Clone, PartialEq)]
pub enum MatchValues {
    /// Match any of the provided keywords.
    Keywords(Vec<String>),
    /// Match any of the provided integers.
    Integers(Vec<i64>),
}

/// Half-open or closed numeric range for [`FilterCondition::Range`].
///
/// All bounds are optional; an entirely-unbounded range matches every numeric
/// value (and is semantically a no-op — prefer omitting the condition). At
/// most one of `lt`/`lte` and one of `gt`/`gte` should be set; supplying both
/// `lt` and `lte` (or both `gt` and `gte`) is accepted by Qdrant but the
/// stricter bound wins.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct NumericRange {
    /// Strict upper bound — `< value`.
    pub lt: Option<f64>,
    /// Inclusive upper bound — `<= value`.
    pub lte: Option<f64>,
    /// Strict lower bound — `> value`.
    pub gt: Option<f64>,
    /// Inclusive lower bound — `>= value`.
    pub gte: Option<f64>,
}

// ─── Translation to Qdrant types ────────────────────────────────────────────

impl From<MemoryFilter> for qd::Filter {
    fn from(value: MemoryFilter) -> Self {
        Self {
            must: value.must.into_iter().map(qd::Condition::from).collect(),
            must_not: value.must_not.into_iter().map(qd::Condition::from).collect(),
            should: value.should.into_iter().map(qd::Condition::from).collect(),
            min_should: None,
        }
    }
}

impl From<FilterCondition> for qd::Condition {
    fn from(value: FilterCondition) -> Self {
        match value {
            FilterCondition::Equals { field, value } => {
                let qdv: qd::r#match::MatchValue = value.into();
                qd::Condition::matches(field, qdv)
            }
            FilterCondition::In { field, values } => {
                let qdv: qd::r#match::MatchValue = values.into();
                qd::Condition::matches(field, qdv)
            }
            FilterCondition::Range { field, range } => qd::Condition::range(field, range.into()),
        }
    }
}

impl From<MatchValue> for qd::r#match::MatchValue {
    fn from(value: MatchValue) -> Self {
        match value {
            MatchValue::Keyword(s) => qd::r#match::MatchValue::Keyword(s),
            MatchValue::Integer(i) => qd::r#match::MatchValue::Integer(i),
            MatchValue::Bool(b) => qd::r#match::MatchValue::Boolean(b),
        }
    }
}

impl From<MatchValues> for qd::r#match::MatchValue {
    fn from(value: MatchValues) -> Self {
        match value {
            MatchValues::Keywords(items) => qd::r#match::MatchValue::Keywords(qd::RepeatedStrings { strings: items }),
            MatchValues::Integers(items) => qd::r#match::MatchValue::Integers(qd::RepeatedIntegers { integers: items }),
        }
    }
}

impl From<NumericRange> for qd::Range {
    fn from(value: NumericRange) -> Self {
        Self {
            lt: value.lt,
            lte: value.lte,
            gt: value.gt,
            gte: value.gte,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use qdrant_client::qdrant as qd;

    fn condition_one_of(condition: qd::Condition) -> qd::condition::ConditionOneOf {
        condition.condition_one_of.expect("condition_one_of populated")
    }

    fn field_condition(condition: qd::Condition) -> qd::FieldCondition {
        match condition_one_of(condition) {
            qd::condition::ConditionOneOf::Field(f) => f,
            other => panic!("expected Field condition, got {other:?}"),
        }
    }

    #[test]
    fn should_translate_empty_filter_to_empty_qdrant_filter() {
        let filter = MemoryFilter::default();
        assert!(filter.is_empty());
        let q: qd::Filter = filter.into();
        assert!(q.must.is_empty());
        assert!(q.must_not.is_empty());
        assert!(q.should.is_empty());
        assert!(q.min_should.is_none());
    }

    #[test]
    fn should_translate_equals_keyword_condition() {
        let cond = FilterCondition::Equals {
            field: "role".into(),
            value: MatchValue::Keyword("user".into()),
        };
        let q = field_condition(cond.into());
        assert_eq!(q.key, "role");
        let match_value = q.r#match.expect("match populated").match_value.expect("inner populated");
        assert!(matches!(match_value, qd::r#match::MatchValue::Keyword(s) if s == "user"));
    }

    #[test]
    fn should_translate_equals_integer_condition() {
        let cond = FilterCondition::Equals {
            field: "conversation_id".into(),
            value: MatchValue::Integer(42),
        };
        let q = field_condition(cond.into());
        assert_eq!(q.key, "conversation_id");
        let match_value = q.r#match.unwrap().match_value.unwrap();
        assert!(matches!(match_value, qd::r#match::MatchValue::Integer(42)));
    }

    #[test]
    fn should_translate_equals_bool_condition() {
        let cond = FilterCondition::Equals {
            field: "active".into(),
            value: MatchValue::Bool(true),
        };
        let q = field_condition(cond.into());
        let match_value = q.r#match.unwrap().match_value.unwrap();
        assert!(matches!(match_value, qd::r#match::MatchValue::Boolean(true)));
    }

    #[test]
    fn should_translate_in_keywords_condition() {
        let cond = FilterCondition::In {
            field: "tag".into(),
            values: MatchValues::Keywords(vec!["a".into(), "b".into()]),
        };
        let q = field_condition(cond.into());
        let match_value = q.r#match.unwrap().match_value.unwrap();
        match match_value {
            qd::r#match::MatchValue::Keywords(rs) => assert_eq!(rs.strings, vec!["a", "b"]),
            other => panic!("expected Keywords, got {other:?}"),
        }
    }

    #[test]
    fn should_translate_in_integers_condition() {
        let cond = FilterCondition::In {
            field: "id".into(),
            values: MatchValues::Integers(vec![1, 2, 3]),
        };
        let q = field_condition(cond.into());
        let match_value = q.r#match.unwrap().match_value.unwrap();
        match match_value {
            qd::r#match::MatchValue::Integers(ri) => assert_eq!(ri.integers, vec![1, 2, 3]),
            other => panic!("expected Integers, got {other:?}"),
        }
    }

    #[test]
    fn should_translate_range_condition_with_all_bounds() {
        let cond = FilterCondition::Range {
            field: "score".into(),
            range: NumericRange {
                lt: Some(0.9),
                lte: None,
                gt: None,
                gte: Some(0.1),
            },
        };
        let q = field_condition(cond.into());
        let range = q.range.expect("range populated");
        assert_eq!(range.lt, Some(0.9));
        assert_eq!(range.gte, Some(0.1));
        assert_eq!(range.lte, None);
        assert_eq!(range.gt, None);
    }

    #[test]
    fn should_translate_full_filter_with_all_three_sections() {
        let filter = MemoryFilter {
            must: vec![FilterCondition::Equals {
                field: "role".into(),
                value: MatchValue::Keyword("user".into()),
            }],
            must_not: vec![FilterCondition::Equals {
                field: "conversation_id".into(),
                value: MatchValue::Integer(42),
            }],
            should: vec![FilterCondition::In {
                field: "tag".into(),
                values: MatchValues::Keywords(vec!["x".into()]),
            }],
        };
        let q: qd::Filter = filter.into();
        assert_eq!(q.must.len(), 1);
        assert_eq!(q.must_not.len(), 1);
        assert_eq!(q.should.len(), 1);
    }

    #[test]
    fn should_report_empty_for_default_filter() {
        assert!(MemoryFilter::default().is_empty());
    }

    #[test]
    fn should_report_non_empty_when_any_section_populated() {
        let filter = MemoryFilter {
            must: vec![FilterCondition::Equals {
                field: "x".into(),
                value: MatchValue::Bool(false),
            }],
            ..MemoryFilter::default()
        };
        assert!(!filter.is_empty());
    }
}
