use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Copy,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    strum::EnumString,
    strum::Display,
    strum::AsRefStr,
    strum::EnumIter,
    strum::IntoStaticStr,
)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum PlanTier {
    #[default]
    Free,
    Pro,
    Plus,
    Enterprise,
}

impl From<PlanTier> for String {
    fn from(tier: PlanTier) -> Self {
        tier.to_string()
    }
}

#[cfg(test)]
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
    fn should_display_as_kebab_case() {
        assert_eq!(PlanTier::Free.to_string(), "free");
        assert_eq!(PlanTier::Pro.to_string(), "pro");
        assert_eq!(PlanTier::Plus.to_string(), "plus");
        assert_eq!(PlanTier::Enterprise.to_string(), "enterprise");
    }

    #[test]
    fn should_default_to_free() {
        assert_eq!(PlanTier::default(), PlanTier::Free);
    }

    #[test]
    fn should_reject_invalid_tier() {
        assert!("invalid".parse::<PlanTier>().is_err());
        assert!("".parse::<PlanTier>().is_err());
    }

    #[test]
    fn should_serialize_to_kebab_case_json() {
        assert_eq!(serde_json::to_string(&PlanTier::Pro).unwrap(), r#""pro""#);
        assert_eq!(serde_json::to_string(&PlanTier::Enterprise).unwrap(), r#""enterprise""#);
    }

    #[test]
    fn should_deserialize_from_kebab_case_json() {
        assert_eq!(serde_json::from_str::<PlanTier>(r#""pro""#).unwrap(), PlanTier::Pro);
        assert_eq!(serde_json::from_str::<PlanTier>(r#""free""#).unwrap(), PlanTier::Free);
    }

    #[test]
    fn should_convert_to_string_via_into() {
        let s: String = PlanTier::Enterprise.into();
        assert_eq!(s, "enterprise");
    }

    #[test]
    fn should_convert_via_as_ref() {
        assert_eq!(PlanTier::Pro.as_ref(), "pro");
    }
}
