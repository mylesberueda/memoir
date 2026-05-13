use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Copy,
    Clone,
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
pub enum OrgRole {
    Owner,
    Admin,
    Member,
    Guest,
}

impl From<OrgRole> for String {
    fn from(role: OrgRole) -> Self {
        role.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_all_roles() {
        assert_eq!("owner".parse::<OrgRole>().unwrap(), OrgRole::Owner);
        assert_eq!("admin".parse::<OrgRole>().unwrap(), OrgRole::Admin);
        assert_eq!("member".parse::<OrgRole>().unwrap(), OrgRole::Member);
        assert_eq!("guest".parse::<OrgRole>().unwrap(), OrgRole::Guest);
    }

    #[test]
    fn should_display_as_kebab_case() {
        assert_eq!(OrgRole::Owner.to_string(), "owner");
        assert_eq!(OrgRole::Admin.to_string(), "admin");
        assert_eq!(OrgRole::Member.to_string(), "member");
        assert_eq!(OrgRole::Guest.to_string(), "guest");
    }

    #[test]
    fn should_reject_invalid_role() {
        assert!("invalid".parse::<OrgRole>().is_err());
        assert!("Owner".parse::<OrgRole>().is_err()); // PascalCase no longer valid
    }

    #[test]
    fn should_serialize_json() {
        assert_eq!(serde_json::to_string(&OrgRole::Owner).unwrap(), r#""owner""#);
    }

    #[test]
    fn should_deserialize_json() {
        assert_eq!(serde_json::from_str::<OrgRole>(r#""admin""#).unwrap(), OrgRole::Admin);
    }
}
