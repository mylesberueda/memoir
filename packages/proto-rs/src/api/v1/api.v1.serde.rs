// @generated
impl serde::Serialize for AddMemberRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.organization_pid.is_empty() {
            len += 1;
        }
        if !self.user_id.is_empty() {
            len += 1;
        }
        if !self.role.is_empty() {
            len += 1;
        }
        if self.email.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.AddMemberRequest", len)?;
        if !self.organization_pid.is_empty() {
            struct_ser.serialize_field("organizationPid", &self.organization_pid)?;
        }
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if !self.role.is_empty() {
            struct_ser.serialize_field("role", &self.role)?;
        }
        if let Some(v) = self.email.as_ref() {
            struct_ser.serialize_field("email", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddMemberRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "organization_pid",
            "organizationPid",
            "user_id",
            "userId",
            "role",
            "email",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OrganizationPid,
            UserId,
            Role,
            Email,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "organizationPid" | "organization_pid" => Ok(GeneratedField::OrganizationPid),
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
                            "role" => Ok(GeneratedField::Role),
                            "email" => Ok(GeneratedField::Email),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AddMemberRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.AddMemberRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddMemberRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut organization_pid__ = None;
                let mut user_id__ = None;
                let mut role__ = None;
                let mut email__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OrganizationPid => {
                            if organization_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationPid"));
                            }
                            organization_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Role => {
                            if role__.is_some() {
                                return Err(serde::de::Error::duplicate_field("role"));
                            }
                            role__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Email => {
                            if email__.is_some() {
                                return Err(serde::de::Error::duplicate_field("email"));
                            }
                            email__ = map_.next_value()?;
                        }
                    }
                }
                Ok(AddMemberRequest {
                    organization_pid: organization_pid__.unwrap_or_default(),
                    user_id: user_id__.unwrap_or_default(),
                    role: role__.unwrap_or_default(),
                    email: email__,
                })
            }
        }
        deserializer.deserialize_struct("api.v1.AddMemberRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AddMemberResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.member.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.AddMemberResponse", len)?;
        if let Some(v) = self.member.as_ref() {
            struct_ser.serialize_field("member", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddMemberResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "member",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Member,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "member" => Ok(GeneratedField::Member),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AddMemberResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.AddMemberResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddMemberResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut member__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Member => {
                            if member__.is_some() {
                                return Err(serde::de::Error::duplicate_field("member"));
                            }
                            member__ = map_.next_value()?;
                        }
                    }
                }
                Ok(AddMemberResponse {
                    member: member__,
                })
            }
        }
        deserializer.deserialize_struct("api.v1.AddMemberResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BillingInterval {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "BILLING_INTERVAL_UNSPECIFIED",
            Self::Monthly => "BILLING_INTERVAL_MONTHLY",
            Self::Annual => "BILLING_INTERVAL_ANNUAL",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for BillingInterval {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "BILLING_INTERVAL_UNSPECIFIED",
            "BILLING_INTERVAL_MONTHLY",
            "BILLING_INTERVAL_ANNUAL",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BillingInterval;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "BILLING_INTERVAL_UNSPECIFIED" => Ok(BillingInterval::Unspecified),
                    "BILLING_INTERVAL_MONTHLY" => Ok(BillingInterval::Monthly),
                    "BILLING_INTERVAL_ANNUAL" => Ok(BillingInterval::Annual),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for BrandingSettings {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.logo_url.is_some() {
            len += 1;
        }
        if self.primary_color.is_some() {
            len += 1;
        }
        if self.secondary_color.is_some() {
            len += 1;
        }
        if self.custom_domain.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.BrandingSettings", len)?;
        if let Some(v) = self.logo_url.as_ref() {
            struct_ser.serialize_field("logoUrl", v)?;
        }
        if let Some(v) = self.primary_color.as_ref() {
            struct_ser.serialize_field("primaryColor", v)?;
        }
        if let Some(v) = self.secondary_color.as_ref() {
            struct_ser.serialize_field("secondaryColor", v)?;
        }
        if let Some(v) = self.custom_domain.as_ref() {
            struct_ser.serialize_field("customDomain", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BrandingSettings {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "logo_url",
            "logoUrl",
            "primary_color",
            "primaryColor",
            "secondary_color",
            "secondaryColor",
            "custom_domain",
            "customDomain",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LogoUrl,
            PrimaryColor,
            SecondaryColor,
            CustomDomain,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "logoUrl" | "logo_url" => Ok(GeneratedField::LogoUrl),
                            "primaryColor" | "primary_color" => Ok(GeneratedField::PrimaryColor),
                            "secondaryColor" | "secondary_color" => Ok(GeneratedField::SecondaryColor),
                            "customDomain" | "custom_domain" => Ok(GeneratedField::CustomDomain),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BrandingSettings;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.BrandingSettings")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BrandingSettings, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut logo_url__ = None;
                let mut primary_color__ = None;
                let mut secondary_color__ = None;
                let mut custom_domain__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LogoUrl => {
                            if logo_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("logoUrl"));
                            }
                            logo_url__ = map_.next_value()?;
                        }
                        GeneratedField::PrimaryColor => {
                            if primary_color__.is_some() {
                                return Err(serde::de::Error::duplicate_field("primaryColor"));
                            }
                            primary_color__ = map_.next_value()?;
                        }
                        GeneratedField::SecondaryColor => {
                            if secondary_color__.is_some() {
                                return Err(serde::de::Error::duplicate_field("secondaryColor"));
                            }
                            secondary_color__ = map_.next_value()?;
                        }
                        GeneratedField::CustomDomain => {
                            if custom_domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("customDomain"));
                            }
                            custom_domain__ = map_.next_value()?;
                        }
                    }
                }
                Ok(BrandingSettings {
                    logo_url: logo_url__,
                    primary_color: primary_color__,
                    secondary_color: secondary_color__,
                    custom_domain: custom_domain__,
                })
            }
        }
        deserializer.deserialize_struct("api.v1.BrandingSettings", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateCheckoutSessionRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.tier != 0 {
            len += 1;
        }
        if self.interval != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.CreateCheckoutSessionRequest", len)?;
        if self.tier != 0 {
            let v = Tier::try_from(self.tier)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.tier)))?;
            struct_ser.serialize_field("tier", &v)?;
        }
        if self.interval != 0 {
            let v = BillingInterval::try_from(self.interval)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.interval)))?;
            struct_ser.serialize_field("interval", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateCheckoutSessionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tier",
            "interval",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Tier,
            Interval,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "tier" => Ok(GeneratedField::Tier),
                            "interval" => Ok(GeneratedField::Interval),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateCheckoutSessionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.CreateCheckoutSessionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateCheckoutSessionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tier__ = None;
                let mut interval__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Tier => {
                            if tier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tier"));
                            }
                            tier__ = Some(map_.next_value::<Tier>()? as i32);
                        }
                        GeneratedField::Interval => {
                            if interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("interval"));
                            }
                            interval__ = Some(map_.next_value::<BillingInterval>()? as i32);
                        }
                    }
                }
                Ok(CreateCheckoutSessionRequest {
                    tier: tier__.unwrap_or_default(),
                    interval: interval__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("api.v1.CreateCheckoutSessionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateCheckoutSessionResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.redirect_url.is_empty() {
            len += 1;
        }
        if self.redirect_type != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.CreateCheckoutSessionResponse", len)?;
        if !self.redirect_url.is_empty() {
            struct_ser.serialize_field("redirectUrl", &self.redirect_url)?;
        }
        if self.redirect_type != 0 {
            let v = RedirectType::try_from(self.redirect_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.redirect_type)))?;
            struct_ser.serialize_field("redirectType", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateCheckoutSessionResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "redirect_url",
            "redirectUrl",
            "redirect_type",
            "redirectType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RedirectUrl,
            RedirectType,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "redirectUrl" | "redirect_url" => Ok(GeneratedField::RedirectUrl),
                            "redirectType" | "redirect_type" => Ok(GeneratedField::RedirectType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateCheckoutSessionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.CreateCheckoutSessionResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateCheckoutSessionResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut redirect_url__ = None;
                let mut redirect_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RedirectUrl => {
                            if redirect_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("redirectUrl"));
                            }
                            redirect_url__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RedirectType => {
                            if redirect_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("redirectType"));
                            }
                            redirect_type__ = Some(map_.next_value::<RedirectType>()? as i32);
                        }
                    }
                }
                Ok(CreateCheckoutSessionResponse {
                    redirect_url: redirect_url__.unwrap_or_default(),
                    redirect_type: redirect_type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("api.v1.CreateCheckoutSessionResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateOrganizationRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.slug.is_empty() {
            len += 1;
        }
        if self.settings.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.CreateOrganizationRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.slug.is_empty() {
            struct_ser.serialize_field("slug", &self.slug)?;
        }
        if let Some(v) = self.settings.as_ref() {
            struct_ser.serialize_field("settings", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateOrganizationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "slug",
            "settings",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Slug,
            Settings,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "slug" => Ok(GeneratedField::Slug),
                            "settings" => Ok(GeneratedField::Settings),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateOrganizationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.CreateOrganizationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateOrganizationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut slug__ = None;
                let mut settings__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Slug => {
                            if slug__.is_some() {
                                return Err(serde::de::Error::duplicate_field("slug"));
                            }
                            slug__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Settings => {
                            if settings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("settings"));
                            }
                            settings__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateOrganizationRequest {
                    name: name__.unwrap_or_default(),
                    slug: slug__.unwrap_or_default(),
                    settings: settings__,
                })
            }
        }
        deserializer.deserialize_struct("api.v1.CreateOrganizationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateOrganizationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.organization.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.CreateOrganizationResponse", len)?;
        if let Some(v) = self.organization.as_ref() {
            struct_ser.serialize_field("organization", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateOrganizationResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "organization",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Organization,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "organization" => Ok(GeneratedField::Organization),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateOrganizationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.CreateOrganizationResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateOrganizationResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut organization__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Organization => {
                            if organization__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organization"));
                            }
                            organization__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateOrganizationResponse {
                    organization: organization__,
                })
            }
        }
        deserializer.deserialize_struct("api.v1.CreateOrganizationResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreatePortalSessionRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("api.v1.CreatePortalSessionRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreatePortalSessionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreatePortalSessionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.CreatePortalSessionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreatePortalSessionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(CreatePortalSessionRequest {
                })
            }
        }
        deserializer.deserialize_struct("api.v1.CreatePortalSessionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreatePortalSessionResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.portal_url.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.CreatePortalSessionResponse", len)?;
        if !self.portal_url.is_empty() {
            struct_ser.serialize_field("portalUrl", &self.portal_url)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreatePortalSessionResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "portal_url",
            "portalUrl",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PortalUrl,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "portalUrl" | "portal_url" => Ok(GeneratedField::PortalUrl),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreatePortalSessionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.CreatePortalSessionResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreatePortalSessionResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut portal_url__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PortalUrl => {
                            if portal_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("portalUrl"));
                            }
                            portal_url__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreatePortalSessionResponse {
                    portal_url: portal_url__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("api.v1.CreatePortalSessionResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteAccountRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("api.v1.DeleteAccountRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteAccountRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteAccountRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.DeleteAccountRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteAccountRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(DeleteAccountRequest {
                })
            }
        }
        deserializer.deserialize_struct("api.v1.DeleteAccountRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteAccountResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("api.v1.DeleteAccountResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteAccountResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteAccountResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.DeleteAccountResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteAccountResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(DeleteAccountResponse {
                })
            }
        }
        deserializer.deserialize_struct("api.v1.DeleteAccountResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteOrganizationRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.organization_pid.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.DeleteOrganizationRequest", len)?;
        if !self.organization_pid.is_empty() {
            struct_ser.serialize_field("organizationPid", &self.organization_pid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteOrganizationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "organization_pid",
            "organizationPid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OrganizationPid,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "organizationPid" | "organization_pid" => Ok(GeneratedField::OrganizationPid),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteOrganizationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.DeleteOrganizationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteOrganizationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut organization_pid__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OrganizationPid => {
                            if organization_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationPid"));
                            }
                            organization_pid__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeleteOrganizationRequest {
                    organization_pid: organization_pid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("api.v1.DeleteOrganizationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteOrganizationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("api.v1.DeleteOrganizationResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteOrganizationResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteOrganizationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.DeleteOrganizationResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteOrganizationResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(DeleteOrganizationResponse {
                })
            }
        }
        deserializer.deserialize_struct("api.v1.DeleteOrganizationResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetCurrentPlanRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("api.v1.GetCurrentPlanRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetCurrentPlanRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetCurrentPlanRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.GetCurrentPlanRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetCurrentPlanRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(GetCurrentPlanRequest {
                })
            }
        }
        deserializer.deserialize_struct("api.v1.GetCurrentPlanRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetCurrentPlanResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.tier != 0 {
            len += 1;
        }
        if self.expires_at.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.GetCurrentPlanResponse", len)?;
        if self.tier != 0 {
            let v = Tier::try_from(self.tier)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.tier)))?;
            struct_ser.serialize_field("tier", &v)?;
        }
        if let Some(v) = self.expires_at.as_ref() {
            struct_ser.serialize_field("expiresAt", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetCurrentPlanResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tier",
            "expires_at",
            "expiresAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Tier,
            ExpiresAt,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "tier" => Ok(GeneratedField::Tier),
                            "expiresAt" | "expires_at" => Ok(GeneratedField::ExpiresAt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetCurrentPlanResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.GetCurrentPlanResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetCurrentPlanResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tier__ = None;
                let mut expires_at__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Tier => {
                            if tier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tier"));
                            }
                            tier__ = Some(map_.next_value::<Tier>()? as i32);
                        }
                        GeneratedField::ExpiresAt => {
                            if expires_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expiresAt"));
                            }
                            expires_at__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetCurrentPlanResponse {
                    tier: tier__.unwrap_or_default(),
                    expires_at: expires_at__,
                })
            }
        }
        deserializer.deserialize_struct("api.v1.GetCurrentPlanResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetOrganizationPlanRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.organization_pid.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.GetOrganizationPlanRequest", len)?;
        if !self.organization_pid.is_empty() {
            struct_ser.serialize_field("organizationPid", &self.organization_pid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetOrganizationPlanRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "organization_pid",
            "organizationPid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OrganizationPid,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "organizationPid" | "organization_pid" => Ok(GeneratedField::OrganizationPid),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetOrganizationPlanRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.GetOrganizationPlanRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetOrganizationPlanRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut organization_pid__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OrganizationPid => {
                            if organization_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationPid"));
                            }
                            organization_pid__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetOrganizationPlanRequest {
                    organization_pid: organization_pid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("api.v1.GetOrganizationPlanRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetOrganizationPlanResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.plan.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.GetOrganizationPlanResponse", len)?;
        if let Some(v) = self.plan.as_ref() {
            struct_ser.serialize_field("plan", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetOrganizationPlanResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "plan",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Plan,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "plan" => Ok(GeneratedField::Plan),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetOrganizationPlanResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.GetOrganizationPlanResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetOrganizationPlanResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut plan__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Plan => {
                            if plan__.is_some() {
                                return Err(serde::de::Error::duplicate_field("plan"));
                            }
                            plan__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetOrganizationPlanResponse {
                    plan: plan__,
                })
            }
        }
        deserializer.deserialize_struct("api.v1.GetOrganizationPlanResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetOrganizationRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.organization_pid.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.GetOrganizationRequest", len)?;
        if !self.organization_pid.is_empty() {
            struct_ser.serialize_field("organizationPid", &self.organization_pid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetOrganizationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "organization_pid",
            "organizationPid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OrganizationPid,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "organizationPid" | "organization_pid" => Ok(GeneratedField::OrganizationPid),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetOrganizationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.GetOrganizationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetOrganizationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut organization_pid__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OrganizationPid => {
                            if organization_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationPid"));
                            }
                            organization_pid__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetOrganizationRequest {
                    organization_pid: organization_pid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("api.v1.GetOrganizationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetOrganizationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.organization.is_some() {
            len += 1;
        }
        if !self.user_role.is_empty() {
            len += 1;
        }
        if !self.permissions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.GetOrganizationResponse", len)?;
        if let Some(v) = self.organization.as_ref() {
            struct_ser.serialize_field("organization", v)?;
        }
        if !self.user_role.is_empty() {
            struct_ser.serialize_field("userRole", &self.user_role)?;
        }
        if !self.permissions.is_empty() {
            struct_ser.serialize_field("permissions", &self.permissions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetOrganizationResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "organization",
            "user_role",
            "userRole",
            "permissions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Organization,
            UserRole,
            Permissions,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "organization" => Ok(GeneratedField::Organization),
                            "userRole" | "user_role" => Ok(GeneratedField::UserRole),
                            "permissions" => Ok(GeneratedField::Permissions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetOrganizationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.GetOrganizationResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetOrganizationResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut organization__ = None;
                let mut user_role__ = None;
                let mut permissions__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Organization => {
                            if organization__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organization"));
                            }
                            organization__ = map_.next_value()?;
                        }
                        GeneratedField::UserRole => {
                            if user_role__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userRole"));
                            }
                            user_role__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Permissions => {
                            if permissions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("permissions"));
                            }
                            permissions__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(GetOrganizationResponse {
                    organization: organization__,
                    user_role: user_role__.unwrap_or_default(),
                    permissions: permissions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("api.v1.GetOrganizationResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetPricingRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("api.v1.GetPricingRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetPricingRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetPricingRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.GetPricingRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetPricingRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(GetPricingRequest {
                })
            }
        }
        deserializer.deserialize_struct("api.v1.GetPricingRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetPricingResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tiers.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.GetPricingResponse", len)?;
        if !self.tiers.is_empty() {
            struct_ser.serialize_field("tiers", &self.tiers)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetPricingResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tiers",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Tiers,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "tiers" => Ok(GeneratedField::Tiers),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetPricingResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.GetPricingResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetPricingResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tiers__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Tiers => {
                            if tiers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tiers"));
                            }
                            tiers__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetPricingResponse {
                    tiers: tiers__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("api.v1.GetPricingResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetUsersRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.GetUsersRequest", len)?;
        if !self.user_ids.is_empty() {
            struct_ser.serialize_field("userIds", &self.user_ids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetUsersRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_ids",
            "userIds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserIds,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "userIds" | "user_ids" => Ok(GeneratedField::UserIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetUsersRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.GetUsersRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetUsersRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserIds => {
                            if user_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userIds"));
                            }
                            user_ids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetUsersRequest {
                    user_ids: user_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("api.v1.GetUsersRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetUsersResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.users.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.GetUsersResponse", len)?;
        if !self.users.is_empty() {
            struct_ser.serialize_field("users", &self.users)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetUsersResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "users",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Users,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "users" => Ok(GeneratedField::Users),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetUsersResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.GetUsersResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetUsersResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut users__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Users => {
                            if users__.is_some() {
                                return Err(serde::de::Error::duplicate_field("users"));
                            }
                            users__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetUsersResponse {
                    users: users__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("api.v1.GetUsersResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListMembersRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.organization_pid.is_empty() {
            len += 1;
        }
        if self.page_size != 0 {
            len += 1;
        }
        if self.cursor.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.ListMembersRequest", len)?;
        if !self.organization_pid.is_empty() {
            struct_ser.serialize_field("organizationPid", &self.organization_pid)?;
        }
        if self.page_size != 0 {
            struct_ser.serialize_field("pageSize", &self.page_size)?;
        }
        if let Some(v) = self.cursor.as_ref() {
            struct_ser.serialize_field("cursor", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListMembersRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "organization_pid",
            "organizationPid",
            "page_size",
            "pageSize",
            "cursor",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OrganizationPid,
            PageSize,
            Cursor,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "organizationPid" | "organization_pid" => Ok(GeneratedField::OrganizationPid),
                            "pageSize" | "page_size" => Ok(GeneratedField::PageSize),
                            "cursor" => Ok(GeneratedField::Cursor),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListMembersRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.ListMembersRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListMembersRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut organization_pid__ = None;
                let mut page_size__ = None;
                let mut cursor__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OrganizationPid => {
                            if organization_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationPid"));
                            }
                            organization_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PageSize => {
                            if page_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageSize"));
                            }
                            page_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Cursor => {
                            if cursor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cursor"));
                            }
                            cursor__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListMembersRequest {
                    organization_pid: organization_pid__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                    cursor: cursor__,
                })
            }
        }
        deserializer.deserialize_struct("api.v1.ListMembersRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListMembersResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.members.is_empty() {
            len += 1;
        }
        if self.next_cursor.is_some() {
            len += 1;
        }
        if self.total != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.ListMembersResponse", len)?;
        if !self.members.is_empty() {
            struct_ser.serialize_field("members", &self.members)?;
        }
        if let Some(v) = self.next_cursor.as_ref() {
            struct_ser.serialize_field("nextCursor", v)?;
        }
        if self.total != 0 {
            struct_ser.serialize_field("total", &self.total)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListMembersResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "members",
            "next_cursor",
            "nextCursor",
            "total",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Members,
            NextCursor,
            Total,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "members" => Ok(GeneratedField::Members),
                            "nextCursor" | "next_cursor" => Ok(GeneratedField::NextCursor),
                            "total" => Ok(GeneratedField::Total),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListMembersResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.ListMembersResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListMembersResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut members__ = None;
                let mut next_cursor__ = None;
                let mut total__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Members => {
                            if members__.is_some() {
                                return Err(serde::de::Error::duplicate_field("members"));
                            }
                            members__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextCursor => {
                            if next_cursor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextCursor"));
                            }
                            next_cursor__ = map_.next_value()?;
                        }
                        GeneratedField::Total => {
                            if total__.is_some() {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            total__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ListMembersResponse {
                    members: members__.unwrap_or_default(),
                    next_cursor: next_cursor__,
                    total: total__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("api.v1.ListMembersResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListOrganizationsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("api.v1.ListOrganizationsRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListOrganizationsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListOrganizationsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.ListOrganizationsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListOrganizationsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ListOrganizationsRequest {
                })
            }
        }
        deserializer.deserialize_struct("api.v1.ListOrganizationsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListOrganizationsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.organizations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.ListOrganizationsResponse", len)?;
        if !self.organizations.is_empty() {
            struct_ser.serialize_field("organizations", &self.organizations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListOrganizationsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "organizations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Organizations,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "organizations" => Ok(GeneratedField::Organizations),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListOrganizationsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.ListOrganizationsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListOrganizationsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut organizations__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Organizations => {
                            if organizations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizations"));
                            }
                            organizations__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListOrganizationsResponse {
                    organizations: organizations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("api.v1.ListOrganizationsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MeRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("api.v1.MeRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MeRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MeRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.MeRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MeRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MeRequest {
                })
            }
        }
        deserializer.deserialize_struct("api.v1.MeRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MeResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.user.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.MeResponse", len)?;
        if let Some(v) = self.user.as_ref() {
            struct_ser.serialize_field("user", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MeResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            User,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "user" => Ok(GeneratedField::User),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.MeResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MeResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = map_.next_value()?;
                        }
                    }
                }
                Ok(MeResponse {
                    user: user__,
                })
            }
        }
        deserializer.deserialize_struct("api.v1.MeResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NotificationSettings {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.email_on_new_member.is_some() {
            len += 1;
        }
        if self.email_on_billing_change.is_some() {
            len += 1;
        }
        if self.slack_notifications.is_some() {
            len += 1;
        }
        if self.slack_webhook_url.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.NotificationSettings", len)?;
        if let Some(v) = self.email_on_new_member.as_ref() {
            struct_ser.serialize_field("emailOnNewMember", v)?;
        }
        if let Some(v) = self.email_on_billing_change.as_ref() {
            struct_ser.serialize_field("emailOnBillingChange", v)?;
        }
        if let Some(v) = self.slack_notifications.as_ref() {
            struct_ser.serialize_field("slackNotifications", v)?;
        }
        if let Some(v) = self.slack_webhook_url.as_ref() {
            struct_ser.serialize_field("slackWebhookUrl", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NotificationSettings {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "email_on_new_member",
            "emailOnNewMember",
            "email_on_billing_change",
            "emailOnBillingChange",
            "slack_notifications",
            "slackNotifications",
            "slack_webhook_url",
            "slackWebhookUrl",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EmailOnNewMember,
            EmailOnBillingChange,
            SlackNotifications,
            SlackWebhookUrl,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "emailOnNewMember" | "email_on_new_member" => Ok(GeneratedField::EmailOnNewMember),
                            "emailOnBillingChange" | "email_on_billing_change" => Ok(GeneratedField::EmailOnBillingChange),
                            "slackNotifications" | "slack_notifications" => Ok(GeneratedField::SlackNotifications),
                            "slackWebhookUrl" | "slack_webhook_url" => Ok(GeneratedField::SlackWebhookUrl),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NotificationSettings;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.NotificationSettings")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<NotificationSettings, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut email_on_new_member__ = None;
                let mut email_on_billing_change__ = None;
                let mut slack_notifications__ = None;
                let mut slack_webhook_url__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EmailOnNewMember => {
                            if email_on_new_member__.is_some() {
                                return Err(serde::de::Error::duplicate_field("emailOnNewMember"));
                            }
                            email_on_new_member__ = map_.next_value()?;
                        }
                        GeneratedField::EmailOnBillingChange => {
                            if email_on_billing_change__.is_some() {
                                return Err(serde::de::Error::duplicate_field("emailOnBillingChange"));
                            }
                            email_on_billing_change__ = map_.next_value()?;
                        }
                        GeneratedField::SlackNotifications => {
                            if slack_notifications__.is_some() {
                                return Err(serde::de::Error::duplicate_field("slackNotifications"));
                            }
                            slack_notifications__ = map_.next_value()?;
                        }
                        GeneratedField::SlackWebhookUrl => {
                            if slack_webhook_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("slackWebhookUrl"));
                            }
                            slack_webhook_url__ = map_.next_value()?;
                        }
                    }
                }
                Ok(NotificationSettings {
                    email_on_new_member: email_on_new_member__,
                    email_on_billing_change: email_on_billing_change__,
                    slack_notifications: slack_notifications__,
                    slack_webhook_url: slack_webhook_url__,
                })
            }
        }
        deserializer.deserialize_struct("api.v1.NotificationSettings", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Organization {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.pid.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.slug.is_empty() {
            len += 1;
        }
        if self.settings.is_some() {
            len += 1;
        }
        if !self.created_at.is_empty() {
            len += 1;
        }
        if !self.updated_at.is_empty() {
            len += 1;
        }
        if self.member_count != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.Organization", len)?;
        if !self.pid.is_empty() {
            struct_ser.serialize_field("pid", &self.pid)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.slug.is_empty() {
            struct_ser.serialize_field("slug", &self.slug)?;
        }
        if let Some(v) = self.settings.as_ref() {
            struct_ser.serialize_field("settings", v)?;
        }
        if !self.created_at.is_empty() {
            struct_ser.serialize_field("createdAt", &self.created_at)?;
        }
        if !self.updated_at.is_empty() {
            struct_ser.serialize_field("updatedAt", &self.updated_at)?;
        }
        if self.member_count != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("memberCount", ToString::to_string(&self.member_count).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Organization {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pid",
            "name",
            "slug",
            "settings",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
            "member_count",
            "memberCount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pid,
            Name,
            Slug,
            Settings,
            CreatedAt,
            UpdatedAt,
            MemberCount,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "pid" => Ok(GeneratedField::Pid),
                            "name" => Ok(GeneratedField::Name),
                            "slug" => Ok(GeneratedField::Slug),
                            "settings" => Ok(GeneratedField::Settings),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            "memberCount" | "member_count" => Ok(GeneratedField::MemberCount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Organization;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.Organization")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Organization, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pid__ = None;
                let mut name__ = None;
                let mut slug__ = None;
                let mut settings__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                let mut member_count__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Pid => {
                            if pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pid"));
                            }
                            pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Slug => {
                            if slug__.is_some() {
                                return Err(serde::de::Error::duplicate_field("slug"));
                            }
                            slug__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Settings => {
                            if settings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("settings"));
                            }
                            settings__ = map_.next_value()?;
                        }
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdAt"));
                            }
                            created_at__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UpdatedAt => {
                            if updated_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updatedAt"));
                            }
                            updated_at__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MemberCount => {
                            if member_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("memberCount"));
                            }
                            member_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Organization {
                    pid: pid__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    slug: slug__.unwrap_or_default(),
                    settings: settings__,
                    created_at: created_at__.unwrap_or_default(),
                    updated_at: updated_at__.unwrap_or_default(),
                    member_count: member_count__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("api.v1.Organization", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OrganizationMember {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.pid.is_empty() {
            len += 1;
        }
        if !self.organization_id.is_empty() {
            len += 1;
        }
        if !self.user_id.is_empty() {
            len += 1;
        }
        if !self.role.is_empty() {
            len += 1;
        }
        if !self.created_at.is_empty() {
            len += 1;
        }
        if self.display_name.is_some() {
            len += 1;
        }
        if !self.email.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.OrganizationMember", len)?;
        if !self.pid.is_empty() {
            struct_ser.serialize_field("pid", &self.pid)?;
        }
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if !self.role.is_empty() {
            struct_ser.serialize_field("role", &self.role)?;
        }
        if !self.created_at.is_empty() {
            struct_ser.serialize_field("createdAt", &self.created_at)?;
        }
        if let Some(v) = self.display_name.as_ref() {
            struct_ser.serialize_field("displayName", v)?;
        }
        if !self.email.is_empty() {
            struct_ser.serialize_field("email", &self.email)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OrganizationMember {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pid",
            "organization_id",
            "organizationId",
            "user_id",
            "userId",
            "role",
            "created_at",
            "createdAt",
            "display_name",
            "displayName",
            "email",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pid,
            OrganizationId,
            UserId,
            Role,
            CreatedAt,
            DisplayName,
            Email,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "pid" => Ok(GeneratedField::Pid),
                            "organizationId" | "organization_id" => Ok(GeneratedField::OrganizationId),
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
                            "role" => Ok(GeneratedField::Role),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "displayName" | "display_name" => Ok(GeneratedField::DisplayName),
                            "email" => Ok(GeneratedField::Email),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OrganizationMember;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.OrganizationMember")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OrganizationMember, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pid__ = None;
                let mut organization_id__ = None;
                let mut user_id__ = None;
                let mut role__ = None;
                let mut created_at__ = None;
                let mut display_name__ = None;
                let mut email__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Pid => {
                            if pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pid"));
                            }
                            pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrganizationId => {
                            if organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            organization_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Role => {
                            if role__.is_some() {
                                return Err(serde::de::Error::duplicate_field("role"));
                            }
                            role__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdAt"));
                            }
                            created_at__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DisplayName => {
                            if display_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayName"));
                            }
                            display_name__ = map_.next_value()?;
                        }
                        GeneratedField::Email => {
                            if email__.is_some() {
                                return Err(serde::de::Error::duplicate_field("email"));
                            }
                            email__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(OrganizationMember {
                    pid: pid__.unwrap_or_default(),
                    organization_id: organization_id__.unwrap_or_default(),
                    user_id: user_id__.unwrap_or_default(),
                    role: role__.unwrap_or_default(),
                    created_at: created_at__.unwrap_or_default(),
                    display_name: display_name__,
                    email: email__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("api.v1.OrganizationMember", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OrganizationPlan {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.organization_pid.is_empty() {
            len += 1;
        }
        if !self.tier.is_empty() {
            len += 1;
        }
        if self.expires_at.is_some() {
            len += 1;
        }
        if !self.created_at.is_empty() {
            len += 1;
        }
        if !self.updated_at.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.OrganizationPlan", len)?;
        if !self.organization_pid.is_empty() {
            struct_ser.serialize_field("organizationPid", &self.organization_pid)?;
        }
        if !self.tier.is_empty() {
            struct_ser.serialize_field("tier", &self.tier)?;
        }
        if let Some(v) = self.expires_at.as_ref() {
            struct_ser.serialize_field("expiresAt", v)?;
        }
        if !self.created_at.is_empty() {
            struct_ser.serialize_field("createdAt", &self.created_at)?;
        }
        if !self.updated_at.is_empty() {
            struct_ser.serialize_field("updatedAt", &self.updated_at)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OrganizationPlan {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "organization_pid",
            "organizationPid",
            "tier",
            "expires_at",
            "expiresAt",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OrganizationPid,
            Tier,
            ExpiresAt,
            CreatedAt,
            UpdatedAt,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "organizationPid" | "organization_pid" => Ok(GeneratedField::OrganizationPid),
                            "tier" => Ok(GeneratedField::Tier),
                            "expiresAt" | "expires_at" => Ok(GeneratedField::ExpiresAt),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OrganizationPlan;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.OrganizationPlan")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OrganizationPlan, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut organization_pid__ = None;
                let mut tier__ = None;
                let mut expires_at__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OrganizationPid => {
                            if organization_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationPid"));
                            }
                            organization_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Tier => {
                            if tier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tier"));
                            }
                            tier__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ExpiresAt => {
                            if expires_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expiresAt"));
                            }
                            expires_at__ = map_.next_value()?;
                        }
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdAt"));
                            }
                            created_at__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UpdatedAt => {
                            if updated_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updatedAt"));
                            }
                            updated_at__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(OrganizationPlan {
                    organization_pid: organization_pid__.unwrap_or_default(),
                    tier: tier__.unwrap_or_default(),
                    expires_at: expires_at__,
                    created_at: created_at__.unwrap_or_default(),
                    updated_at: updated_at__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("api.v1.OrganizationPlan", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OrganizationSettings {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.timezone.is_some() {
            len += 1;
        }
        if self.locale.is_some() {
            len += 1;
        }
        if self.branding.is_some() {
            len += 1;
        }
        if self.security.is_some() {
            len += 1;
        }
        if self.notifications.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.OrganizationSettings", len)?;
        if let Some(v) = self.timezone.as_ref() {
            struct_ser.serialize_field("timezone", v)?;
        }
        if let Some(v) = self.locale.as_ref() {
            struct_ser.serialize_field("locale", v)?;
        }
        if let Some(v) = self.branding.as_ref() {
            struct_ser.serialize_field("branding", v)?;
        }
        if let Some(v) = self.security.as_ref() {
            struct_ser.serialize_field("security", v)?;
        }
        if let Some(v) = self.notifications.as_ref() {
            struct_ser.serialize_field("notifications", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OrganizationSettings {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "timezone",
            "locale",
            "branding",
            "security",
            "notifications",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Timezone,
            Locale,
            Branding,
            Security,
            Notifications,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "timezone" => Ok(GeneratedField::Timezone),
                            "locale" => Ok(GeneratedField::Locale),
                            "branding" => Ok(GeneratedField::Branding),
                            "security" => Ok(GeneratedField::Security),
                            "notifications" => Ok(GeneratedField::Notifications),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OrganizationSettings;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.OrganizationSettings")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OrganizationSettings, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut timezone__ = None;
                let mut locale__ = None;
                let mut branding__ = None;
                let mut security__ = None;
                let mut notifications__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Timezone => {
                            if timezone__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timezone"));
                            }
                            timezone__ = map_.next_value()?;
                        }
                        GeneratedField::Locale => {
                            if locale__.is_some() {
                                return Err(serde::de::Error::duplicate_field("locale"));
                            }
                            locale__ = map_.next_value()?;
                        }
                        GeneratedField::Branding => {
                            if branding__.is_some() {
                                return Err(serde::de::Error::duplicate_field("branding"));
                            }
                            branding__ = map_.next_value()?;
                        }
                        GeneratedField::Security => {
                            if security__.is_some() {
                                return Err(serde::de::Error::duplicate_field("security"));
                            }
                            security__ = map_.next_value()?;
                        }
                        GeneratedField::Notifications => {
                            if notifications__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notifications"));
                            }
                            notifications__ = map_.next_value()?;
                        }
                    }
                }
                Ok(OrganizationSettings {
                    timezone: timezone__,
                    locale: locale__,
                    branding: branding__,
                    security: security__,
                    notifications: notifications__,
                })
            }
        }
        deserializer.deserialize_struct("api.v1.OrganizationSettings", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PricingTier {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.tier != 0 {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if self.price_cents != 0 {
            len += 1;
        }
        if self.annual_price_cents != 0 {
            len += 1;
        }
        if !self.currency.is_empty() {
            len += 1;
        }
        if !self.features.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.PricingTier", len)?;
        if self.tier != 0 {
            let v = Tier::try_from(self.tier)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.tier)))?;
            struct_ser.serialize_field("tier", &v)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.price_cents != 0 {
            struct_ser.serialize_field("priceCents", &self.price_cents)?;
        }
        if self.annual_price_cents != 0 {
            struct_ser.serialize_field("annualPriceCents", &self.annual_price_cents)?;
        }
        if !self.currency.is_empty() {
            struct_ser.serialize_field("currency", &self.currency)?;
        }
        if !self.features.is_empty() {
            struct_ser.serialize_field("features", &self.features)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PricingTier {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tier",
            "name",
            "price_cents",
            "priceCents",
            "annual_price_cents",
            "annualPriceCents",
            "currency",
            "features",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Tier,
            Name,
            PriceCents,
            AnnualPriceCents,
            Currency,
            Features,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "tier" => Ok(GeneratedField::Tier),
                            "name" => Ok(GeneratedField::Name),
                            "priceCents" | "price_cents" => Ok(GeneratedField::PriceCents),
                            "annualPriceCents" | "annual_price_cents" => Ok(GeneratedField::AnnualPriceCents),
                            "currency" => Ok(GeneratedField::Currency),
                            "features" => Ok(GeneratedField::Features),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PricingTier;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.PricingTier")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PricingTier, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tier__ = None;
                let mut name__ = None;
                let mut price_cents__ = None;
                let mut annual_price_cents__ = None;
                let mut currency__ = None;
                let mut features__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Tier => {
                            if tier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tier"));
                            }
                            tier__ = Some(map_.next_value::<Tier>()? as i32);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PriceCents => {
                            if price_cents__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priceCents"));
                            }
                            price_cents__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::AnnualPriceCents => {
                            if annual_price_cents__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annualPriceCents"));
                            }
                            annual_price_cents__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Currency => {
                            if currency__.is_some() {
                                return Err(serde::de::Error::duplicate_field("currency"));
                            }
                            currency__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Features => {
                            if features__.is_some() {
                                return Err(serde::de::Error::duplicate_field("features"));
                            }
                            features__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PricingTier {
                    tier: tier__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    price_cents: price_cents__.unwrap_or_default(),
                    annual_price_cents: annual_price_cents__.unwrap_or_default(),
                    currency: currency__.unwrap_or_default(),
                    features: features__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("api.v1.PricingTier", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RedirectType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "REDIRECT_TYPE_UNSPECIFIED",
            Self::Checkout => "REDIRECT_TYPE_CHECKOUT",
            Self::Portal => "REDIRECT_TYPE_PORTAL",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for RedirectType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "REDIRECT_TYPE_UNSPECIFIED",
            "REDIRECT_TYPE_CHECKOUT",
            "REDIRECT_TYPE_PORTAL",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RedirectType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "REDIRECT_TYPE_UNSPECIFIED" => Ok(RedirectType::Unspecified),
                    "REDIRECT_TYPE_CHECKOUT" => Ok(RedirectType::Checkout),
                    "REDIRECT_TYPE_PORTAL" => Ok(RedirectType::Portal),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for RemoveMemberRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.organization_pid.is_empty() {
            len += 1;
        }
        if !self.user_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.RemoveMemberRequest", len)?;
        if !self.organization_pid.is_empty() {
            struct_ser.serialize_field("organizationPid", &self.organization_pid)?;
        }
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoveMemberRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "organization_pid",
            "organizationPid",
            "user_id",
            "userId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OrganizationPid,
            UserId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "organizationPid" | "organization_pid" => Ok(GeneratedField::OrganizationPid),
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RemoveMemberRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.RemoveMemberRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemoveMemberRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut organization_pid__ = None;
                let mut user_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OrganizationPid => {
                            if organization_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationPid"));
                            }
                            organization_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RemoveMemberRequest {
                    organization_pid: organization_pid__.unwrap_or_default(),
                    user_id: user_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("api.v1.RemoveMemberRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemoveMemberResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("api.v1.RemoveMemberResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoveMemberResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RemoveMemberResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.RemoveMemberResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemoveMemberResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(RemoveMemberResponse {
                })
            }
        }
        deserializer.deserialize_struct("api.v1.RemoveMemberResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResourcePermission {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.read {
            len += 1;
        }
        if self.write {
            len += 1;
        }
        if self.execute {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.ResourcePermission", len)?;
        if self.read {
            struct_ser.serialize_field("read", &self.read)?;
        }
        if self.write {
            struct_ser.serialize_field("write", &self.write)?;
        }
        if self.execute {
            struct_ser.serialize_field("execute", &self.execute)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResourcePermission {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "read",
            "write",
            "execute",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Read,
            Write,
            Execute,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "read" => Ok(GeneratedField::Read),
                            "write" => Ok(GeneratedField::Write),
                            "execute" => Ok(GeneratedField::Execute),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResourcePermission;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.ResourcePermission")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResourcePermission, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut read__ = None;
                let mut write__ = None;
                let mut execute__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Read => {
                            if read__.is_some() {
                                return Err(serde::de::Error::duplicate_field("read"));
                            }
                            read__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Write => {
                            if write__.is_some() {
                                return Err(serde::de::Error::duplicate_field("write"));
                            }
                            write__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Execute => {
                            if execute__.is_some() {
                                return Err(serde::de::Error::duplicate_field("execute"));
                            }
                            execute__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ResourcePermission {
                    read: read__.unwrap_or_default(),
                    write: write__.unwrap_or_default(),
                    execute: execute__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("api.v1.ResourcePermission", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SecuritySettings {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.two_factor_required.is_some() {
            len += 1;
        }
        if self.session_timeout_minutes.is_some() {
            len += 1;
        }
        if self.password_min_length.is_some() {
            len += 1;
        }
        if self.require_password_symbols.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.SecuritySettings", len)?;
        if let Some(v) = self.two_factor_required.as_ref() {
            struct_ser.serialize_field("twoFactorRequired", v)?;
        }
        if let Some(v) = self.session_timeout_minutes.as_ref() {
            struct_ser.serialize_field("sessionTimeoutMinutes", v)?;
        }
        if let Some(v) = self.password_min_length.as_ref() {
            struct_ser.serialize_field("passwordMinLength", v)?;
        }
        if let Some(v) = self.require_password_symbols.as_ref() {
            struct_ser.serialize_field("requirePasswordSymbols", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SecuritySettings {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "two_factor_required",
            "twoFactorRequired",
            "session_timeout_minutes",
            "sessionTimeoutMinutes",
            "password_min_length",
            "passwordMinLength",
            "require_password_symbols",
            "requirePasswordSymbols",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TwoFactorRequired,
            SessionTimeoutMinutes,
            PasswordMinLength,
            RequirePasswordSymbols,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "twoFactorRequired" | "two_factor_required" => Ok(GeneratedField::TwoFactorRequired),
                            "sessionTimeoutMinutes" | "session_timeout_minutes" => Ok(GeneratedField::SessionTimeoutMinutes),
                            "passwordMinLength" | "password_min_length" => Ok(GeneratedField::PasswordMinLength),
                            "requirePasswordSymbols" | "require_password_symbols" => Ok(GeneratedField::RequirePasswordSymbols),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SecuritySettings;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.SecuritySettings")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SecuritySettings, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut two_factor_required__ = None;
                let mut session_timeout_minutes__ = None;
                let mut password_min_length__ = None;
                let mut require_password_symbols__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TwoFactorRequired => {
                            if two_factor_required__.is_some() {
                                return Err(serde::de::Error::duplicate_field("twoFactorRequired"));
                            }
                            two_factor_required__ = map_.next_value()?;
                        }
                        GeneratedField::SessionTimeoutMinutes => {
                            if session_timeout_minutes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sessionTimeoutMinutes"));
                            }
                            session_timeout_minutes__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::PasswordMinLength => {
                            if password_min_length__.is_some() {
                                return Err(serde::de::Error::duplicate_field("passwordMinLength"));
                            }
                            password_min_length__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::RequirePasswordSymbols => {
                            if require_password_symbols__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requirePasswordSymbols"));
                            }
                            require_password_symbols__ = map_.next_value()?;
                        }
                    }
                }
                Ok(SecuritySettings {
                    two_factor_required: two_factor_required__,
                    session_timeout_minutes: session_timeout_minutes__,
                    password_min_length: password_min_length__,
                    require_password_symbols: require_password_symbols__,
                })
            }
        }
        deserializer.deserialize_struct("api.v1.SecuritySettings", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Tier {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "TIER_UNSPECIFIED",
            Self::Free => "TIER_FREE",
            Self::Plus => "TIER_PLUS",
            Self::Pro => "TIER_PRO",
            Self::Enterprise => "TIER_ENTERPRISE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for Tier {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "TIER_UNSPECIFIED",
            "TIER_FREE",
            "TIER_PLUS",
            "TIER_PRO",
            "TIER_ENTERPRISE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Tier;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "TIER_UNSPECIFIED" => Ok(Tier::Unspecified),
                    "TIER_FREE" => Ok(Tier::Free),
                    "TIER_PLUS" => Ok(Tier::Plus),
                    "TIER_PRO" => Ok(Tier::Pro),
                    "TIER_ENTERPRISE" => Ok(Tier::Enterprise),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateMemberRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.organization_pid.is_empty() {
            len += 1;
        }
        if !self.user_id.is_empty() {
            len += 1;
        }
        if self.role.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.UpdateMemberRequest", len)?;
        if !self.organization_pid.is_empty() {
            struct_ser.serialize_field("organizationPid", &self.organization_pid)?;
        }
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if let Some(v) = self.role.as_ref() {
            struct_ser.serialize_field("role", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateMemberRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "organization_pid",
            "organizationPid",
            "user_id",
            "userId",
            "role",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OrganizationPid,
            UserId,
            Role,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "organizationPid" | "organization_pid" => Ok(GeneratedField::OrganizationPid),
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
                            "role" => Ok(GeneratedField::Role),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateMemberRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.UpdateMemberRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateMemberRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut organization_pid__ = None;
                let mut user_id__ = None;
                let mut role__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OrganizationPid => {
                            if organization_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationPid"));
                            }
                            organization_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Role => {
                            if role__.is_some() {
                                return Err(serde::de::Error::duplicate_field("role"));
                            }
                            role__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateMemberRequest {
                    organization_pid: organization_pid__.unwrap_or_default(),
                    user_id: user_id__.unwrap_or_default(),
                    role: role__,
                })
            }
        }
        deserializer.deserialize_struct("api.v1.UpdateMemberRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateMemberResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.member.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.UpdateMemberResponse", len)?;
        if let Some(v) = self.member.as_ref() {
            struct_ser.serialize_field("member", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateMemberResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "member",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Member,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "member" => Ok(GeneratedField::Member),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateMemberResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.UpdateMemberResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateMemberResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut member__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Member => {
                            if member__.is_some() {
                                return Err(serde::de::Error::duplicate_field("member"));
                            }
                            member__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateMemberResponse {
                    member: member__,
                })
            }
        }
        deserializer.deserialize_struct("api.v1.UpdateMemberResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateOrganizationPlanRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.organization_pid.is_empty() {
            len += 1;
        }
        if self.tier.is_some() {
            len += 1;
        }
        if self.expires_at.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.UpdateOrganizationPlanRequest", len)?;
        if !self.organization_pid.is_empty() {
            struct_ser.serialize_field("organizationPid", &self.organization_pid)?;
        }
        if let Some(v) = self.tier.as_ref() {
            struct_ser.serialize_field("tier", v)?;
        }
        if let Some(v) = self.expires_at.as_ref() {
            struct_ser.serialize_field("expiresAt", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateOrganizationPlanRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "organization_pid",
            "organizationPid",
            "tier",
            "expires_at",
            "expiresAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OrganizationPid,
            Tier,
            ExpiresAt,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "organizationPid" | "organization_pid" => Ok(GeneratedField::OrganizationPid),
                            "tier" => Ok(GeneratedField::Tier),
                            "expiresAt" | "expires_at" => Ok(GeneratedField::ExpiresAt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateOrganizationPlanRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.UpdateOrganizationPlanRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateOrganizationPlanRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut organization_pid__ = None;
                let mut tier__ = None;
                let mut expires_at__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OrganizationPid => {
                            if organization_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationPid"));
                            }
                            organization_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Tier => {
                            if tier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tier"));
                            }
                            tier__ = map_.next_value()?;
                        }
                        GeneratedField::ExpiresAt => {
                            if expires_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expiresAt"));
                            }
                            expires_at__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateOrganizationPlanRequest {
                    organization_pid: organization_pid__.unwrap_or_default(),
                    tier: tier__,
                    expires_at: expires_at__,
                })
            }
        }
        deserializer.deserialize_struct("api.v1.UpdateOrganizationPlanRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateOrganizationPlanResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.plan.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.UpdateOrganizationPlanResponse", len)?;
        if let Some(v) = self.plan.as_ref() {
            struct_ser.serialize_field("plan", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateOrganizationPlanResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "plan",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Plan,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "plan" => Ok(GeneratedField::Plan),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateOrganizationPlanResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.UpdateOrganizationPlanResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateOrganizationPlanResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut plan__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Plan => {
                            if plan__.is_some() {
                                return Err(serde::de::Error::duplicate_field("plan"));
                            }
                            plan__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateOrganizationPlanResponse {
                    plan: plan__,
                })
            }
        }
        deserializer.deserialize_struct("api.v1.UpdateOrganizationPlanResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateOrganizationRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.organization_pid.is_empty() {
            len += 1;
        }
        if self.name.is_some() {
            len += 1;
        }
        if self.slug.is_some() {
            len += 1;
        }
        if self.settings.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.UpdateOrganizationRequest", len)?;
        if !self.organization_pid.is_empty() {
            struct_ser.serialize_field("organizationPid", &self.organization_pid)?;
        }
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if let Some(v) = self.slug.as_ref() {
            struct_ser.serialize_field("slug", v)?;
        }
        if let Some(v) = self.settings.as_ref() {
            struct_ser.serialize_field("settings", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateOrganizationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "organization_pid",
            "organizationPid",
            "name",
            "slug",
            "settings",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OrganizationPid,
            Name,
            Slug,
            Settings,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "organizationPid" | "organization_pid" => Ok(GeneratedField::OrganizationPid),
                            "name" => Ok(GeneratedField::Name),
                            "slug" => Ok(GeneratedField::Slug),
                            "settings" => Ok(GeneratedField::Settings),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateOrganizationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.UpdateOrganizationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateOrganizationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut organization_pid__ = None;
                let mut name__ = None;
                let mut slug__ = None;
                let mut settings__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OrganizationPid => {
                            if organization_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationPid"));
                            }
                            organization_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = map_.next_value()?;
                        }
                        GeneratedField::Slug => {
                            if slug__.is_some() {
                                return Err(serde::de::Error::duplicate_field("slug"));
                            }
                            slug__ = map_.next_value()?;
                        }
                        GeneratedField::Settings => {
                            if settings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("settings"));
                            }
                            settings__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateOrganizationRequest {
                    organization_pid: organization_pid__.unwrap_or_default(),
                    name: name__,
                    slug: slug__,
                    settings: settings__,
                })
            }
        }
        deserializer.deserialize_struct("api.v1.UpdateOrganizationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateOrganizationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.organization.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.UpdateOrganizationResponse", len)?;
        if let Some(v) = self.organization.as_ref() {
            struct_ser.serialize_field("organization", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateOrganizationResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "organization",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Organization,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "organization" => Ok(GeneratedField::Organization),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateOrganizationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.UpdateOrganizationResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateOrganizationResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut organization__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Organization => {
                            if organization__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organization"));
                            }
                            organization__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateOrganizationResponse {
                    organization: organization__,
                })
            }
        }
        deserializer.deserialize_struct("api.v1.UpdateOrganizationResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateProfileRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.display_name.is_some() {
            len += 1;
        }
        if self.avatar_url.is_some() {
            len += 1;
        }
        if self.bio.is_some() {
            len += 1;
        }
        if self.settings.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.UpdateProfileRequest", len)?;
        if let Some(v) = self.display_name.as_ref() {
            struct_ser.serialize_field("displayName", v)?;
        }
        if let Some(v) = self.avatar_url.as_ref() {
            struct_ser.serialize_field("avatarUrl", v)?;
        }
        if let Some(v) = self.bio.as_ref() {
            struct_ser.serialize_field("bio", v)?;
        }
        if let Some(v) = self.settings.as_ref() {
            struct_ser.serialize_field("settings", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateProfileRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "display_name",
            "displayName",
            "avatar_url",
            "avatarUrl",
            "bio",
            "settings",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DisplayName,
            AvatarUrl,
            Bio,
            Settings,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "displayName" | "display_name" => Ok(GeneratedField::DisplayName),
                            "avatarUrl" | "avatar_url" => Ok(GeneratedField::AvatarUrl),
                            "bio" => Ok(GeneratedField::Bio),
                            "settings" => Ok(GeneratedField::Settings),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateProfileRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.UpdateProfileRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateProfileRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut display_name__ = None;
                let mut avatar_url__ = None;
                let mut bio__ = None;
                let mut settings__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DisplayName => {
                            if display_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayName"));
                            }
                            display_name__ = map_.next_value()?;
                        }
                        GeneratedField::AvatarUrl => {
                            if avatar_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("avatarUrl"));
                            }
                            avatar_url__ = map_.next_value()?;
                        }
                        GeneratedField::Bio => {
                            if bio__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bio"));
                            }
                            bio__ = map_.next_value()?;
                        }
                        GeneratedField::Settings => {
                            if settings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("settings"));
                            }
                            settings__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateProfileRequest {
                    display_name: display_name__,
                    avatar_url: avatar_url__,
                    bio: bio__,
                    settings: settings__,
                })
            }
        }
        deserializer.deserialize_struct("api.v1.UpdateProfileRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateProfileResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.user.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.UpdateProfileResponse", len)?;
        if let Some(v) = self.user.as_ref() {
            struct_ser.serialize_field("user", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateProfileResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            User,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "user" => Ok(GeneratedField::User),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateProfileResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.UpdateProfileResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateProfileResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateProfileResponse {
                    user: user__,
                })
            }
        }
        deserializer.deserialize_struct("api.v1.UpdateProfileResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for User {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.email.is_empty() {
            len += 1;
        }
        if self.display_name.is_some() {
            len += 1;
        }
        if self.avatar_url.is_some() {
            len += 1;
        }
        if self.bio.is_some() {
            len += 1;
        }
        if self.settings.is_some() {
            len += 1;
        }
        if !self.created_at.is_empty() {
            len += 1;
        }
        if !self.updated_at.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.User", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.email.is_empty() {
            struct_ser.serialize_field("email", &self.email)?;
        }
        if let Some(v) = self.display_name.as_ref() {
            struct_ser.serialize_field("displayName", v)?;
        }
        if let Some(v) = self.avatar_url.as_ref() {
            struct_ser.serialize_field("avatarUrl", v)?;
        }
        if let Some(v) = self.bio.as_ref() {
            struct_ser.serialize_field("bio", v)?;
        }
        if let Some(v) = self.settings.as_ref() {
            struct_ser.serialize_field("settings", v)?;
        }
        if !self.created_at.is_empty() {
            struct_ser.serialize_field("createdAt", &self.created_at)?;
        }
        if !self.updated_at.is_empty() {
            struct_ser.serialize_field("updatedAt", &self.updated_at)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for User {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "email",
            "display_name",
            "displayName",
            "avatar_url",
            "avatarUrl",
            "bio",
            "settings",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Email,
            DisplayName,
            AvatarUrl,
            Bio,
            Settings,
            CreatedAt,
            UpdatedAt,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "email" => Ok(GeneratedField::Email),
                            "displayName" | "display_name" => Ok(GeneratedField::DisplayName),
                            "avatarUrl" | "avatar_url" => Ok(GeneratedField::AvatarUrl),
                            "bio" => Ok(GeneratedField::Bio),
                            "settings" => Ok(GeneratedField::Settings),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = User;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.User")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<User, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut email__ = None;
                let mut display_name__ = None;
                let mut avatar_url__ = None;
                let mut bio__ = None;
                let mut settings__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Email => {
                            if email__.is_some() {
                                return Err(serde::de::Error::duplicate_field("email"));
                            }
                            email__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DisplayName => {
                            if display_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayName"));
                            }
                            display_name__ = map_.next_value()?;
                        }
                        GeneratedField::AvatarUrl => {
                            if avatar_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("avatarUrl"));
                            }
                            avatar_url__ = map_.next_value()?;
                        }
                        GeneratedField::Bio => {
                            if bio__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bio"));
                            }
                            bio__ = map_.next_value()?;
                        }
                        GeneratedField::Settings => {
                            if settings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("settings"));
                            }
                            settings__ = map_.next_value()?;
                        }
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdAt"));
                            }
                            created_at__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UpdatedAt => {
                            if updated_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updatedAt"));
                            }
                            updated_at__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(User {
                    id: id__.unwrap_or_default(),
                    email: email__.unwrap_or_default(),
                    display_name: display_name__,
                    avatar_url: avatar_url__,
                    bio: bio__,
                    settings: settings__,
                    created_at: created_at__.unwrap_or_default(),
                    updated_at: updated_at__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("api.v1.User", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UserSettings {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.theme.is_some() {
            len += 1;
        }
        if self.language.is_some() {
            len += 1;
        }
        if self.timezone.is_some() {
            len += 1;
        }
        if self.email_notifications {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.UserSettings", len)?;
        if let Some(v) = self.theme.as_ref() {
            struct_ser.serialize_field("theme", v)?;
        }
        if let Some(v) = self.language.as_ref() {
            struct_ser.serialize_field("language", v)?;
        }
        if let Some(v) = self.timezone.as_ref() {
            struct_ser.serialize_field("timezone", v)?;
        }
        if self.email_notifications {
            struct_ser.serialize_field("emailNotifications", &self.email_notifications)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UserSettings {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "theme",
            "language",
            "timezone",
            "email_notifications",
            "emailNotifications",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Theme,
            Language,
            Timezone,
            EmailNotifications,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "theme" => Ok(GeneratedField::Theme),
                            "language" => Ok(GeneratedField::Language),
                            "timezone" => Ok(GeneratedField::Timezone),
                            "emailNotifications" | "email_notifications" => Ok(GeneratedField::EmailNotifications),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UserSettings;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.UserSettings")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UserSettings, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut theme__ = None;
                let mut language__ = None;
                let mut timezone__ = None;
                let mut email_notifications__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Theme => {
                            if theme__.is_some() {
                                return Err(serde::de::Error::duplicate_field("theme"));
                            }
                            theme__ = map_.next_value()?;
                        }
                        GeneratedField::Language => {
                            if language__.is_some() {
                                return Err(serde::de::Error::duplicate_field("language"));
                            }
                            language__ = map_.next_value()?;
                        }
                        GeneratedField::Timezone => {
                            if timezone__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timezone"));
                            }
                            timezone__ = map_.next_value()?;
                        }
                        GeneratedField::EmailNotifications => {
                            if email_notifications__.is_some() {
                                return Err(serde::de::Error::duplicate_field("emailNotifications"));
                            }
                            email_notifications__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UserSettings {
                    theme: theme__,
                    language: language__,
                    timezone: timezone__,
                    email_notifications: email_notifications__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("api.v1.UserSettings", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UserSummary {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if self.display_name.is_some() {
            len += 1;
        }
        if !self.email.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.v1.UserSummary", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.display_name.as_ref() {
            struct_ser.serialize_field("displayName", v)?;
        }
        if !self.email.is_empty() {
            struct_ser.serialize_field("email", &self.email)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UserSummary {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "display_name",
            "displayName",
            "email",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            DisplayName,
            Email,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "displayName" | "display_name" => Ok(GeneratedField::DisplayName),
                            "email" => Ok(GeneratedField::Email),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UserSummary;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.v1.UserSummary")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UserSummary, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut display_name__ = None;
                let mut email__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DisplayName => {
                            if display_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayName"));
                            }
                            display_name__ = map_.next_value()?;
                        }
                        GeneratedField::Email => {
                            if email__.is_some() {
                                return Err(serde::de::Error::duplicate_field("email"));
                            }
                            email__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UserSummary {
                    id: id__.unwrap_or_default(),
                    display_name: display_name__,
                    email: email__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("api.v1.UserSummary", FIELDS, GeneratedVisitor)
    }
}
