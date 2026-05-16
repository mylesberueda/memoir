// @generated
impl serde::Serialize for ApiKey {
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
        if !self.key_id.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if self.role != 0 {
            len += 1;
        }
        if self.org_id.is_some() {
            len += 1;
        }
        if self.status != 0 {
            len += 1;
        }
        if self.created_at.is_some() {
            len += 1;
        }
        if self.updated_at.is_some() {
            len += 1;
        }
        if self.last_used_at.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("memoir.v1.ApiKey", len)?;
        if !self.pid.is_empty() {
            struct_ser.serialize_field("pid", &self.pid)?;
        }
        if !self.key_id.is_empty() {
            struct_ser.serialize_field("keyId", &self.key_id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.role != 0 {
            let v = ApiKeyRole::try_from(self.role)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.role)))?;
            struct_ser.serialize_field("role", &v)?;
        }
        if let Some(v) = self.org_id.as_ref() {
            struct_ser.serialize_field("orgId", v)?;
        }
        if self.status != 0 {
            let v = ApiKeyStatus::try_from(self.status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        if let Some(v) = self.created_at.as_ref() {
            struct_ser.serialize_field("createdAt", v)?;
        }
        if let Some(v) = self.updated_at.as_ref() {
            struct_ser.serialize_field("updatedAt", v)?;
        }
        if let Some(v) = self.last_used_at.as_ref() {
            struct_ser.serialize_field("lastUsedAt", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ApiKey {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pid",
            "key_id",
            "keyId",
            "name",
            "role",
            "org_id",
            "orgId",
            "status",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
            "last_used_at",
            "lastUsedAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pid,
            KeyId,
            Name,
            Role,
            OrgId,
            Status,
            CreatedAt,
            UpdatedAt,
            LastUsedAt,
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
                            "keyId" | "key_id" => Ok(GeneratedField::KeyId),
                            "name" => Ok(GeneratedField::Name),
                            "role" => Ok(GeneratedField::Role),
                            "orgId" | "org_id" => Ok(GeneratedField::OrgId),
                            "status" => Ok(GeneratedField::Status),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            "lastUsedAt" | "last_used_at" => Ok(GeneratedField::LastUsedAt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ApiKey;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct memoir.v1.ApiKey")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ApiKey, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pid__ = None;
                let mut key_id__ = None;
                let mut name__ = None;
                let mut role__ = None;
                let mut org_id__ = None;
                let mut status__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                let mut last_used_at__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Pid => {
                            if pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pid"));
                            }
                            pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::KeyId => {
                            if key_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyId"));
                            }
                            key_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Role => {
                            if role__.is_some() {
                                return Err(serde::de::Error::duplicate_field("role"));
                            }
                            role__ = Some(map_.next_value::<ApiKeyRole>()? as i32);
                        }
                        GeneratedField::OrgId => {
                            if org_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orgId"));
                            }
                            org_id__ = map_.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value::<ApiKeyStatus>()? as i32);
                        }
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdAt"));
                            }
                            created_at__ = map_.next_value()?;
                        }
                        GeneratedField::UpdatedAt => {
                            if updated_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updatedAt"));
                            }
                            updated_at__ = map_.next_value()?;
                        }
                        GeneratedField::LastUsedAt => {
                            if last_used_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastUsedAt"));
                            }
                            last_used_at__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ApiKey {
                    pid: pid__.unwrap_or_default(),
                    key_id: key_id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    role: role__.unwrap_or_default(),
                    org_id: org_id__,
                    status: status__.unwrap_or_default(),
                    created_at: created_at__,
                    updated_at: updated_at__,
                    last_used_at: last_used_at__,
                })
            }
        }
        deserializer.deserialize_struct("memoir.v1.ApiKey", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ApiKeyRole {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "API_KEY_ROLE_UNSPECIFIED",
            Self::Admin => "API_KEY_ROLE_ADMIN",
            Self::Integration => "API_KEY_ROLE_INTEGRATION",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ApiKeyRole {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "API_KEY_ROLE_UNSPECIFIED",
            "API_KEY_ROLE_ADMIN",
            "API_KEY_ROLE_INTEGRATION",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ApiKeyRole;

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
                    "API_KEY_ROLE_UNSPECIFIED" => Ok(ApiKeyRole::Unspecified),
                    "API_KEY_ROLE_ADMIN" => Ok(ApiKeyRole::Admin),
                    "API_KEY_ROLE_INTEGRATION" => Ok(ApiKeyRole::Integration),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ApiKeyStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "API_KEY_STATUS_UNSPECIFIED",
            Self::Active => "API_KEY_STATUS_ACTIVE",
            Self::Revoked => "API_KEY_STATUS_REVOKED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ApiKeyStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "API_KEY_STATUS_UNSPECIFIED",
            "API_KEY_STATUS_ACTIVE",
            "API_KEY_STATUS_REVOKED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ApiKeyStatus;

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
                    "API_KEY_STATUS_UNSPECIFIED" => Ok(ApiKeyStatus::Unspecified),
                    "API_KEY_STATUS_ACTIVE" => Ok(ApiKeyStatus::Active),
                    "API_KEY_STATUS_REVOKED" => Ok(ApiKeyStatus::Revoked),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ConsumeBootstrapTokenRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.token.is_empty() {
            len += 1;
        }
        if !self.username.is_empty() {
            len += 1;
        }
        if !self.password.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("memoir.v1.ConsumeBootstrapTokenRequest", len)?;
        if !self.token.is_empty() {
            struct_ser.serialize_field("token", &self.token)?;
        }
        if !self.username.is_empty() {
            struct_ser.serialize_field("username", &self.username)?;
        }
        if !self.password.is_empty() {
            struct_ser.serialize_field("password", &self.password)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ConsumeBootstrapTokenRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "token",
            "username",
            "password",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Token,
            Username,
            Password,
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
                            "token" => Ok(GeneratedField::Token),
                            "username" => Ok(GeneratedField::Username),
                            "password" => Ok(GeneratedField::Password),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ConsumeBootstrapTokenRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct memoir.v1.ConsumeBootstrapTokenRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ConsumeBootstrapTokenRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut token__ = None;
                let mut username__ = None;
                let mut password__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Token => {
                            if token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("token"));
                            }
                            token__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Username => {
                            if username__.is_some() {
                                return Err(serde::de::Error::duplicate_field("username"));
                            }
                            username__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Password => {
                            if password__.is_some() {
                                return Err(serde::de::Error::duplicate_field("password"));
                            }
                            password__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ConsumeBootstrapTokenRequest {
                    token: token__.unwrap_or_default(),
                    username: username__.unwrap_or_default(),
                    password: password__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("memoir.v1.ConsumeBootstrapTokenRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ConsumeBootstrapTokenResponse {
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
        let mut struct_ser = serializer.serialize_struct("memoir.v1.ConsumeBootstrapTokenResponse", len)?;
        if let Some(v) = self.user.as_ref() {
            struct_ser.serialize_field("user", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ConsumeBootstrapTokenResponse {
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
            type Value = ConsumeBootstrapTokenResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct memoir.v1.ConsumeBootstrapTokenResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ConsumeBootstrapTokenResponse, V::Error>
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
                Ok(ConsumeBootstrapTokenResponse {
                    user: user__,
                })
            }
        }
        deserializer.deserialize_struct("memoir.v1.ConsumeBootstrapTokenResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateApiKeyRequest {
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
        if self.role != 0 {
            len += 1;
        }
        if self.org_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("memoir.v1.CreateApiKeyRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.role != 0 {
            let v = ApiKeyRole::try_from(self.role)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.role)))?;
            struct_ser.serialize_field("role", &v)?;
        }
        if let Some(v) = self.org_id.as_ref() {
            struct_ser.serialize_field("orgId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateApiKeyRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "role",
            "org_id",
            "orgId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Role,
            OrgId,
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
                            "role" => Ok(GeneratedField::Role),
                            "orgId" | "org_id" => Ok(GeneratedField::OrgId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateApiKeyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct memoir.v1.CreateApiKeyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateApiKeyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut role__ = None;
                let mut org_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Role => {
                            if role__.is_some() {
                                return Err(serde::de::Error::duplicate_field("role"));
                            }
                            role__ = Some(map_.next_value::<ApiKeyRole>()? as i32);
                        }
                        GeneratedField::OrgId => {
                            if org_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orgId"));
                            }
                            org_id__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateApiKeyRequest {
                    name: name__.unwrap_or_default(),
                    role: role__.unwrap_or_default(),
                    org_id: org_id__,
                })
            }
        }
        deserializer.deserialize_struct("memoir.v1.CreateApiKeyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateApiKeyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.key.is_some() {
            len += 1;
        }
        if !self.plaintext.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("memoir.v1.CreateApiKeyResponse", len)?;
        if let Some(v) = self.key.as_ref() {
            struct_ser.serialize_field("key", v)?;
        }
        if !self.plaintext.is_empty() {
            struct_ser.serialize_field("plaintext", &self.plaintext)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateApiKeyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
            "plaintext",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            Plaintext,
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
                            "key" => Ok(GeneratedField::Key),
                            "plaintext" => Ok(GeneratedField::Plaintext),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateApiKeyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct memoir.v1.CreateApiKeyResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateApiKeyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut plaintext__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = map_.next_value()?;
                        }
                        GeneratedField::Plaintext => {
                            if plaintext__.is_some() {
                                return Err(serde::de::Error::duplicate_field("plaintext"));
                            }
                            plaintext__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateApiKeyResponse {
                    key: key__,
                    plaintext: plaintext__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("memoir.v1.CreateApiKeyResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateUserRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.username.is_empty() {
            len += 1;
        }
        if !self.password.is_empty() {
            len += 1;
        }
        if self.is_admin {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("memoir.v1.CreateUserRequest", len)?;
        if !self.username.is_empty() {
            struct_ser.serialize_field("username", &self.username)?;
        }
        if !self.password.is_empty() {
            struct_ser.serialize_field("password", &self.password)?;
        }
        if self.is_admin {
            struct_ser.serialize_field("isAdmin", &self.is_admin)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateUserRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "username",
            "password",
            "is_admin",
            "isAdmin",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Username,
            Password,
            IsAdmin,
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
                            "username" => Ok(GeneratedField::Username),
                            "password" => Ok(GeneratedField::Password),
                            "isAdmin" | "is_admin" => Ok(GeneratedField::IsAdmin),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateUserRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct memoir.v1.CreateUserRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateUserRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut username__ = None;
                let mut password__ = None;
                let mut is_admin__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Username => {
                            if username__.is_some() {
                                return Err(serde::de::Error::duplicate_field("username"));
                            }
                            username__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Password => {
                            if password__.is_some() {
                                return Err(serde::de::Error::duplicate_field("password"));
                            }
                            password__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsAdmin => {
                            if is_admin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isAdmin"));
                            }
                            is_admin__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateUserRequest {
                    username: username__.unwrap_or_default(),
                    password: password__.unwrap_or_default(),
                    is_admin: is_admin__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("memoir.v1.CreateUserRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateUserResponse {
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
        let mut struct_ser = serializer.serialize_struct("memoir.v1.CreateUserResponse", len)?;
        if let Some(v) = self.user.as_ref() {
            struct_ser.serialize_field("user", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateUserResponse {
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
            type Value = CreateUserResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct memoir.v1.CreateUserResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateUserResponse, V::Error>
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
                Ok(CreateUserResponse {
                    user: user__,
                })
            }
        }
        deserializer.deserialize_struct("memoir.v1.CreateUserResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteUserRequest {
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
        let mut struct_ser = serializer.serialize_struct("memoir.v1.DeleteUserRequest", len)?;
        if !self.pid.is_empty() {
            struct_ser.serialize_field("pid", &self.pid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteUserRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pid,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteUserRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct memoir.v1.DeleteUserRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteUserRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pid__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Pid => {
                            if pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pid"));
                            }
                            pid__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeleteUserRequest {
                    pid: pid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("memoir.v1.DeleteUserRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteUserResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("memoir.v1.DeleteUserResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteUserResponse {
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
            type Value = DeleteUserResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct memoir.v1.DeleteUserResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteUserResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(DeleteUserResponse {
                })
            }
        }
        deserializer.deserialize_struct("memoir.v1.DeleteUserResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ForgetRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.hard_delete {
            len += 1;
        }
        if self.target.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("memoir.v1.ForgetRequest", len)?;
        if self.hard_delete {
            struct_ser.serialize_field("hardDelete", &self.hard_delete)?;
        }
        if let Some(v) = self.target.as_ref() {
            match v {
                forget_request::Target::Pid(v) => {
                    struct_ser.serialize_field("pid", v)?;
                }
                forget_request::Target::Scope(v) => {
                    struct_ser.serialize_field("scope", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ForgetRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "hard_delete",
            "hardDelete",
            "pid",
            "scope",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HardDelete,
            Pid,
            Scope,
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
                            "hardDelete" | "hard_delete" => Ok(GeneratedField::HardDelete),
                            "pid" => Ok(GeneratedField::Pid),
                            "scope" => Ok(GeneratedField::Scope),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ForgetRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct memoir.v1.ForgetRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ForgetRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut hard_delete__ = None;
                let mut target__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::HardDelete => {
                            if hard_delete__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hardDelete"));
                            }
                            hard_delete__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pid => {
                            if target__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pid"));
                            }
                            target__ = map_.next_value::<::std::option::Option<_>>()?.map(forget_request::Target::Pid);
                        }
                        GeneratedField::Scope => {
                            if target__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scope"));
                            }
                            target__ = map_.next_value::<::std::option::Option<_>>()?.map(forget_request::Target::Scope)
;
                        }
                    }
                }
                Ok(ForgetRequest {
                    hard_delete: hard_delete__.unwrap_or_default(),
                    target: target__,
                })
            }
        }
        deserializer.deserialize_struct("memoir.v1.ForgetRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ForgetResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("memoir.v1.ForgetResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ForgetResponse {
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
            type Value = ForgetResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct memoir.v1.ForgetResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ForgetResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ForgetResponse {
                })
            }
        }
        deserializer.deserialize_struct("memoir.v1.ForgetResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetApiKeyRequest {
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
        let mut struct_ser = serializer.serialize_struct("memoir.v1.GetApiKeyRequest", len)?;
        if !self.pid.is_empty() {
            struct_ser.serialize_field("pid", &self.pid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetApiKeyRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pid,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetApiKeyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct memoir.v1.GetApiKeyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetApiKeyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pid__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Pid => {
                            if pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pid"));
                            }
                            pid__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetApiKeyRequest {
                    pid: pid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("memoir.v1.GetApiKeyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetApiKeyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("memoir.v1.GetApiKeyResponse", len)?;
        if let Some(v) = self.key.as_ref() {
            struct_ser.serialize_field("key", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetApiKeyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
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
                            "key" => Ok(GeneratedField::Key),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetApiKeyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct memoir.v1.GetApiKeyResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetApiKeyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetApiKeyResponse {
                    key: key__,
                })
            }
        }
        deserializer.deserialize_struct("memoir.v1.GetApiKeyResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetUserRequest {
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
        let mut struct_ser = serializer.serialize_struct("memoir.v1.GetUserRequest", len)?;
        if !self.pid.is_empty() {
            struct_ser.serialize_field("pid", &self.pid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetUserRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pid,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetUserRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct memoir.v1.GetUserRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetUserRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pid__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Pid => {
                            if pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pid"));
                            }
                            pid__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetUserRequest {
                    pid: pid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("memoir.v1.GetUserRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetUserResponse {
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
        let mut struct_ser = serializer.serialize_struct("memoir.v1.GetUserResponse", len)?;
        if let Some(v) = self.user.as_ref() {
            struct_ser.serialize_field("user", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetUserResponse {
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
            type Value = GetUserResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct memoir.v1.GetUserResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetUserResponse, V::Error>
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
                Ok(GetUserResponse {
                    user: user__,
                })
            }
        }
        deserializer.deserialize_struct("memoir.v1.GetUserResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListApiKeysRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.limit != 0 {
            len += 1;
        }
        if self.cursor.is_some() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("memoir.v1.ListApiKeysRequest", len)?;
        if self.limit != 0 {
            struct_ser.serialize_field("limit", &self.limit)?;
        }
        if let Some(v) = self.cursor.as_ref() {
            struct_ser.serialize_field("cursor", v)?;
        }
        if let Some(v) = self.status.as_ref() {
            let v = ApiKeyStatus::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListApiKeysRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "limit",
            "cursor",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Limit,
            Cursor,
            Status,
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
                            "limit" => Ok(GeneratedField::Limit),
                            "cursor" => Ok(GeneratedField::Cursor),
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListApiKeysRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct memoir.v1.ListApiKeysRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListApiKeysRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut limit__ = None;
                let mut cursor__ = None;
                let mut status__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Limit => {
                            if limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("limit"));
                            }
                            limit__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Cursor => {
                            if cursor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cursor"));
                            }
                            cursor__ = map_.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map_.next_value::<::std::option::Option<ApiKeyStatus>>()?.map(|x| x as i32);
                        }
                    }
                }
                Ok(ListApiKeysRequest {
                    limit: limit__.unwrap_or_default(),
                    cursor: cursor__,
                    status: status__,
                })
            }
        }
        deserializer.deserialize_struct("memoir.v1.ListApiKeysRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListApiKeysResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.keys.is_empty() {
            len += 1;
        }
        if self.next_cursor.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("memoir.v1.ListApiKeysResponse", len)?;
        if !self.keys.is_empty() {
            struct_ser.serialize_field("keys", &self.keys)?;
        }
        if let Some(v) = self.next_cursor.as_ref() {
            struct_ser.serialize_field("nextCursor", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListApiKeysResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "keys",
            "next_cursor",
            "nextCursor",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Keys,
            NextCursor,
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
                            "keys" => Ok(GeneratedField::Keys),
                            "nextCursor" | "next_cursor" => Ok(GeneratedField::NextCursor),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListApiKeysResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct memoir.v1.ListApiKeysResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListApiKeysResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut keys__ = None;
                let mut next_cursor__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Keys => {
                            if keys__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keys"));
                            }
                            keys__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextCursor => {
                            if next_cursor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextCursor"));
                            }
                            next_cursor__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListApiKeysResponse {
                    keys: keys__.unwrap_or_default(),
                    next_cursor: next_cursor__,
                })
            }
        }
        deserializer.deserialize_struct("memoir.v1.ListApiKeysResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListUsersRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.limit != 0 {
            len += 1;
        }
        if self.cursor.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("memoir.v1.ListUsersRequest", len)?;
        if self.limit != 0 {
            struct_ser.serialize_field("limit", &self.limit)?;
        }
        if let Some(v) = self.cursor.as_ref() {
            struct_ser.serialize_field("cursor", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListUsersRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "limit",
            "cursor",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Limit,
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
                            "limit" => Ok(GeneratedField::Limit),
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
            type Value = ListUsersRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct memoir.v1.ListUsersRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListUsersRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut limit__ = None;
                let mut cursor__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Limit => {
                            if limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("limit"));
                            }
                            limit__ = 
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
                Ok(ListUsersRequest {
                    limit: limit__.unwrap_or_default(),
                    cursor: cursor__,
                })
            }
        }
        deserializer.deserialize_struct("memoir.v1.ListUsersRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListUsersResponse {
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
        if self.next_cursor.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("memoir.v1.ListUsersResponse", len)?;
        if !self.users.is_empty() {
            struct_ser.serialize_field("users", &self.users)?;
        }
        if let Some(v) = self.next_cursor.as_ref() {
            struct_ser.serialize_field("nextCursor", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListUsersResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "users",
            "next_cursor",
            "nextCursor",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Users,
            NextCursor,
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
                            "nextCursor" | "next_cursor" => Ok(GeneratedField::NextCursor),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListUsersResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct memoir.v1.ListUsersResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListUsersResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut users__ = None;
                let mut next_cursor__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Users => {
                            if users__.is_some() {
                                return Err(serde::de::Error::duplicate_field("users"));
                            }
                            users__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextCursor => {
                            if next_cursor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextCursor"));
                            }
                            next_cursor__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListUsersResponse {
                    users: users__.unwrap_or_default(),
                    next_cursor: next_cursor__,
                })
            }
        }
        deserializer.deserialize_struct("memoir.v1.ListUsersResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LoginRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.username.is_empty() {
            len += 1;
        }
        if !self.password.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("memoir.v1.LoginRequest", len)?;
        if !self.username.is_empty() {
            struct_ser.serialize_field("username", &self.username)?;
        }
        if !self.password.is_empty() {
            struct_ser.serialize_field("password", &self.password)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LoginRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "username",
            "password",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Username,
            Password,
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
                            "username" => Ok(GeneratedField::Username),
                            "password" => Ok(GeneratedField::Password),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LoginRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct memoir.v1.LoginRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LoginRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut username__ = None;
                let mut password__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Username => {
                            if username__.is_some() {
                                return Err(serde::de::Error::duplicate_field("username"));
                            }
                            username__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Password => {
                            if password__.is_some() {
                                return Err(serde::de::Error::duplicate_field("password"));
                            }
                            password__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(LoginRequest {
                    username: username__.unwrap_or_default(),
                    password: password__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("memoir.v1.LoginRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LoginResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.access_token.is_empty() {
            len += 1;
        }
        if !self.refresh_token.is_empty() {
            len += 1;
        }
        if self.user.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("memoir.v1.LoginResponse", len)?;
        if !self.access_token.is_empty() {
            struct_ser.serialize_field("accessToken", &self.access_token)?;
        }
        if !self.refresh_token.is_empty() {
            struct_ser.serialize_field("refreshToken", &self.refresh_token)?;
        }
        if let Some(v) = self.user.as_ref() {
            struct_ser.serialize_field("user", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LoginResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "access_token",
            "accessToken",
            "refresh_token",
            "refreshToken",
            "user",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AccessToken,
            RefreshToken,
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
                            "accessToken" | "access_token" => Ok(GeneratedField::AccessToken),
                            "refreshToken" | "refresh_token" => Ok(GeneratedField::RefreshToken),
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
            type Value = LoginResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct memoir.v1.LoginResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LoginResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut access_token__ = None;
                let mut refresh_token__ = None;
                let mut user__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AccessToken => {
                            if access_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accessToken"));
                            }
                            access_token__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RefreshToken => {
                            if refresh_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("refreshToken"));
                            }
                            refresh_token__ = Some(map_.next_value()?);
                        }
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = map_.next_value()?;
                        }
                    }
                }
                Ok(LoginResponse {
                    access_token: access_token__.unwrap_or_default(),
                    refresh_token: refresh_token__.unwrap_or_default(),
                    user: user__,
                })
            }
        }
        deserializer.deserialize_struct("memoir.v1.LoginResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Memory {
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
        if self.scope.is_some() {
            len += 1;
        }
        if !self.content.is_empty() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        if self.created_at.is_some() {
            len += 1;
        }
        if self.processed_at.is_some() {
            len += 1;
        }
        if self.status != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("memoir.v1.Memory", len)?;
        if !self.pid.is_empty() {
            struct_ser.serialize_field("pid", &self.pid)?;
        }
        if let Some(v) = self.scope.as_ref() {
            struct_ser.serialize_field("scope", v)?;
        }
        if !self.content.is_empty() {
            struct_ser.serialize_field("content", &self.content)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if let Some(v) = self.created_at.as_ref() {
            struct_ser.serialize_field("createdAt", v)?;
        }
        if let Some(v) = self.processed_at.as_ref() {
            struct_ser.serialize_field("processedAt", v)?;
        }
        if self.status != 0 {
            let v = MemoryStatus::try_from(self.status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Memory {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pid",
            "scope",
            "content",
            "metadata",
            "created_at",
            "createdAt",
            "processed_at",
            "processedAt",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pid,
            Scope,
            Content,
            Metadata,
            CreatedAt,
            ProcessedAt,
            Status,
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
                            "scope" => Ok(GeneratedField::Scope),
                            "content" => Ok(GeneratedField::Content),
                            "metadata" => Ok(GeneratedField::Metadata),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "processedAt" | "processed_at" => Ok(GeneratedField::ProcessedAt),
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Memory;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct memoir.v1.Memory")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Memory, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pid__ = None;
                let mut scope__ = None;
                let mut content__ = None;
                let mut metadata__ = None;
                let mut created_at__ = None;
                let mut processed_at__ = None;
                let mut status__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Pid => {
                            if pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pid"));
                            }
                            pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Scope => {
                            if scope__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scope"));
                            }
                            scope__ = map_.next_value()?;
                        }
                        GeneratedField::Content => {
                            if content__.is_some() {
                                return Err(serde::de::Error::duplicate_field("content"));
                            }
                            content__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdAt"));
                            }
                            created_at__ = map_.next_value()?;
                        }
                        GeneratedField::ProcessedAt => {
                            if processed_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("processedAt"));
                            }
                            processed_at__ = map_.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value::<MemoryStatus>()? as i32);
                        }
                    }
                }
                Ok(Memory {
                    pid: pid__.unwrap_or_default(),
                    scope: scope__,
                    content: content__.unwrap_or_default(),
                    metadata: metadata__,
                    created_at: created_at__,
                    processed_at: processed_at__,
                    status: status__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("memoir.v1.Memory", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MemoryStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "MEMORY_STATUS_UNSPECIFIED",
            Self::Pending => "MEMORY_STATUS_PENDING",
            Self::Processed => "MEMORY_STATUS_PROCESSED",
            Self::Failed => "MEMORY_STATUS_FAILED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for MemoryStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "MEMORY_STATUS_UNSPECIFIED",
            "MEMORY_STATUS_PENDING",
            "MEMORY_STATUS_PROCESSED",
            "MEMORY_STATUS_FAILED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MemoryStatus;

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
                    "MEMORY_STATUS_UNSPECIFIED" => Ok(MemoryStatus::Unspecified),
                    "MEMORY_STATUS_PENDING" => Ok(MemoryStatus::Pending),
                    "MEMORY_STATUS_PROCESSED" => Ok(MemoryStatus::Processed),
                    "MEMORY_STATUS_FAILED" => Ok(MemoryStatus::Failed),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for RecallRequest {
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
        let mut struct_ser = serializer.serialize_struct("memoir.v1.RecallRequest", len)?;
        if !self.pid.is_empty() {
            struct_ser.serialize_field("pid", &self.pid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RecallRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pid,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RecallRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct memoir.v1.RecallRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RecallRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pid__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Pid => {
                            if pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pid"));
                            }
                            pid__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RecallRequest {
                    pid: pid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("memoir.v1.RecallRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RecallResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.memory.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("memoir.v1.RecallResponse", len)?;
        if let Some(v) = self.memory.as_ref() {
            struct_ser.serialize_field("memory", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RecallResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "memory",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Memory,
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
                            "memory" => Ok(GeneratedField::Memory),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RecallResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct memoir.v1.RecallResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RecallResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut memory__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Memory => {
                            if memory__.is_some() {
                                return Err(serde::de::Error::duplicate_field("memory"));
                            }
                            memory__ = map_.next_value()?;
                        }
                    }
                }
                Ok(RecallResponse {
                    memory: memory__,
                })
            }
        }
        deserializer.deserialize_struct("memoir.v1.RecallResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RefreshTokenRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.refresh_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("memoir.v1.RefreshTokenRequest", len)?;
        if !self.refresh_token.is_empty() {
            struct_ser.serialize_field("refreshToken", &self.refresh_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RefreshTokenRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "refresh_token",
            "refreshToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RefreshToken,
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
                            "refreshToken" | "refresh_token" => Ok(GeneratedField::RefreshToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RefreshTokenRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct memoir.v1.RefreshTokenRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RefreshTokenRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut refresh_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RefreshToken => {
                            if refresh_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("refreshToken"));
                            }
                            refresh_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RefreshTokenRequest {
                    refresh_token: refresh_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("memoir.v1.RefreshTokenRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RefreshTokenResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.access_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("memoir.v1.RefreshTokenResponse", len)?;
        if !self.access_token.is_empty() {
            struct_ser.serialize_field("accessToken", &self.access_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RefreshTokenResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "access_token",
            "accessToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AccessToken,
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
                            "accessToken" | "access_token" => Ok(GeneratedField::AccessToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RefreshTokenResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct memoir.v1.RefreshTokenResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RefreshTokenResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut access_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AccessToken => {
                            if access_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accessToken"));
                            }
                            access_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RefreshTokenResponse {
                    access_token: access_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("memoir.v1.RefreshTokenResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RememberRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.scope.is_some() {
            len += 1;
        }
        if !self.content.is_empty() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("memoir.v1.RememberRequest", len)?;
        if let Some(v) = self.scope.as_ref() {
            struct_ser.serialize_field("scope", v)?;
        }
        if !self.content.is_empty() {
            struct_ser.serialize_field("content", &self.content)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RememberRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "scope",
            "content",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Scope,
            Content,
            Metadata,
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
                            "scope" => Ok(GeneratedField::Scope),
                            "content" => Ok(GeneratedField::Content),
                            "metadata" => Ok(GeneratedField::Metadata),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RememberRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct memoir.v1.RememberRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RememberRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut scope__ = None;
                let mut content__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Scope => {
                            if scope__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scope"));
                            }
                            scope__ = map_.next_value()?;
                        }
                        GeneratedField::Content => {
                            if content__.is_some() {
                                return Err(serde::de::Error::duplicate_field("content"));
                            }
                            content__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                    }
                }
                Ok(RememberRequest {
                    scope: scope__,
                    content: content__.unwrap_or_default(),
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("memoir.v1.RememberRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RememberResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.memory.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("memoir.v1.RememberResponse", len)?;
        if let Some(v) = self.memory.as_ref() {
            struct_ser.serialize_field("memory", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RememberResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "memory",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Memory,
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
                            "memory" => Ok(GeneratedField::Memory),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RememberResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct memoir.v1.RememberResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RememberResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut memory__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Memory => {
                            if memory__.is_some() {
                                return Err(serde::de::Error::duplicate_field("memory"));
                            }
                            memory__ = map_.next_value()?;
                        }
                    }
                }
                Ok(RememberResponse {
                    memory: memory__,
                })
            }
        }
        deserializer.deserialize_struct("memoir.v1.RememberResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RevokeApiKeyRequest {
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
        let mut struct_ser = serializer.serialize_struct("memoir.v1.RevokeApiKeyRequest", len)?;
        if !self.pid.is_empty() {
            struct_ser.serialize_field("pid", &self.pid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RevokeApiKeyRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pid,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RevokeApiKeyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct memoir.v1.RevokeApiKeyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RevokeApiKeyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pid__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Pid => {
                            if pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pid"));
                            }
                            pid__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RevokeApiKeyRequest {
                    pid: pid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("memoir.v1.RevokeApiKeyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RevokeApiKeyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("memoir.v1.RevokeApiKeyResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RevokeApiKeyResponse {
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
            type Value = RevokeApiKeyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct memoir.v1.RevokeApiKeyResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RevokeApiKeyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(RevokeApiKeyResponse {
                })
            }
        }
        deserializer.deserialize_struct("memoir.v1.RevokeApiKeyResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RotateApiKeyRequest {
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
        let mut struct_ser = serializer.serialize_struct("memoir.v1.RotateApiKeyRequest", len)?;
        if !self.pid.is_empty() {
            struct_ser.serialize_field("pid", &self.pid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RotateApiKeyRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pid,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RotateApiKeyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct memoir.v1.RotateApiKeyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RotateApiKeyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pid__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Pid => {
                            if pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pid"));
                            }
                            pid__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RotateApiKeyRequest {
                    pid: pid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("memoir.v1.RotateApiKeyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RotateApiKeyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.key.is_some() {
            len += 1;
        }
        if !self.plaintext.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("memoir.v1.RotateApiKeyResponse", len)?;
        if let Some(v) = self.key.as_ref() {
            struct_ser.serialize_field("key", v)?;
        }
        if !self.plaintext.is_empty() {
            struct_ser.serialize_field("plaintext", &self.plaintext)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RotateApiKeyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
            "plaintext",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            Plaintext,
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
                            "key" => Ok(GeneratedField::Key),
                            "plaintext" => Ok(GeneratedField::Plaintext),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RotateApiKeyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct memoir.v1.RotateApiKeyResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RotateApiKeyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut plaintext__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = map_.next_value()?;
                        }
                        GeneratedField::Plaintext => {
                            if plaintext__.is_some() {
                                return Err(serde::de::Error::duplicate_field("plaintext"));
                            }
                            plaintext__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RotateApiKeyResponse {
                    key: key__,
                    plaintext: plaintext__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("memoir.v1.RotateApiKeyResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Scope {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.agent_id.is_empty() {
            len += 1;
        }
        if !self.org_id.is_empty() {
            len += 1;
        }
        if !self.user_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("memoir.v1.Scope", len)?;
        if !self.agent_id.is_empty() {
            struct_ser.serialize_field("agentId", &self.agent_id)?;
        }
        if !self.org_id.is_empty() {
            struct_ser.serialize_field("orgId", &self.org_id)?;
        }
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Scope {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "agent_id",
            "agentId",
            "org_id",
            "orgId",
            "user_id",
            "userId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AgentId,
            OrgId,
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
                            "agentId" | "agent_id" => Ok(GeneratedField::AgentId),
                            "orgId" | "org_id" => Ok(GeneratedField::OrgId),
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
            type Value = Scope;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct memoir.v1.Scope")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Scope, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut agent_id__ = None;
                let mut org_id__ = None;
                let mut user_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AgentId => {
                            if agent_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("agentId"));
                            }
                            agent_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrgId => {
                            if org_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orgId"));
                            }
                            org_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Scope {
                    agent_id: agent_id__.unwrap_or_default(),
                    org_id: org_id__.unwrap_or_default(),
                    user_id: user_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("memoir.v1.Scope", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SearchHit {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.memory.is_some() {
            len += 1;
        }
        if self.score != 0. {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("memoir.v1.SearchHit", len)?;
        if let Some(v) = self.memory.as_ref() {
            struct_ser.serialize_field("memory", v)?;
        }
        if self.score != 0. {
            struct_ser.serialize_field("score", &self.score)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SearchHit {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "memory",
            "score",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Memory,
            Score,
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
                            "memory" => Ok(GeneratedField::Memory),
                            "score" => Ok(GeneratedField::Score),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SearchHit;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct memoir.v1.SearchHit")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SearchHit, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut memory__ = None;
                let mut score__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Memory => {
                            if memory__.is_some() {
                                return Err(serde::de::Error::duplicate_field("memory"));
                            }
                            memory__ = map_.next_value()?;
                        }
                        GeneratedField::Score => {
                            if score__.is_some() {
                                return Err(serde::de::Error::duplicate_field("score"));
                            }
                            score__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(SearchHit {
                    memory: memory__,
                    score: score__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("memoir.v1.SearchHit", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SearchRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.scope.is_some() {
            len += 1;
        }
        if !self.query.is_empty() {
            len += 1;
        }
        if self.limit != 0 {
            len += 1;
        }
        if self.metadata_filter.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("memoir.v1.SearchRequest", len)?;
        if let Some(v) = self.scope.as_ref() {
            struct_ser.serialize_field("scope", v)?;
        }
        if !self.query.is_empty() {
            struct_ser.serialize_field("query", &self.query)?;
        }
        if self.limit != 0 {
            struct_ser.serialize_field("limit", &self.limit)?;
        }
        if let Some(v) = self.metadata_filter.as_ref() {
            struct_ser.serialize_field("metadataFilter", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SearchRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "scope",
            "query",
            "limit",
            "metadata_filter",
            "metadataFilter",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Scope,
            Query,
            Limit,
            MetadataFilter,
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
                            "scope" => Ok(GeneratedField::Scope),
                            "query" => Ok(GeneratedField::Query),
                            "limit" => Ok(GeneratedField::Limit),
                            "metadataFilter" | "metadata_filter" => Ok(GeneratedField::MetadataFilter),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SearchRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct memoir.v1.SearchRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SearchRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut scope__ = None;
                let mut query__ = None;
                let mut limit__ = None;
                let mut metadata_filter__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Scope => {
                            if scope__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scope"));
                            }
                            scope__ = map_.next_value()?;
                        }
                        GeneratedField::Query => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            query__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Limit => {
                            if limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("limit"));
                            }
                            limit__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MetadataFilter => {
                            if metadata_filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadataFilter"));
                            }
                            metadata_filter__ = map_.next_value()?;
                        }
                    }
                }
                Ok(SearchRequest {
                    scope: scope__,
                    query: query__.unwrap_or_default(),
                    limit: limit__.unwrap_or_default(),
                    metadata_filter: metadata_filter__,
                })
            }
        }
        deserializer.deserialize_struct("memoir.v1.SearchRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SearchResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.hits.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("memoir.v1.SearchResponse", len)?;
        if !self.hits.is_empty() {
            struct_ser.serialize_field("hits", &self.hits)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SearchResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "hits",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Hits,
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
                            "hits" => Ok(GeneratedField::Hits),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SearchResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct memoir.v1.SearchResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SearchResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut hits__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Hits => {
                            if hits__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hits"));
                            }
                            hits__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SearchResponse {
                    hits: hits__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("memoir.v1.SearchResponse", FIELDS, GeneratedVisitor)
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
        if !self.pid.is_empty() {
            len += 1;
        }
        if !self.username.is_empty() {
            len += 1;
        }
        if self.is_admin {
            len += 1;
        }
        if self.created_at.is_some() {
            len += 1;
        }
        if self.updated_at.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("memoir.v1.User", len)?;
        if !self.pid.is_empty() {
            struct_ser.serialize_field("pid", &self.pid)?;
        }
        if !self.username.is_empty() {
            struct_ser.serialize_field("username", &self.username)?;
        }
        if self.is_admin {
            struct_ser.serialize_field("isAdmin", &self.is_admin)?;
        }
        if let Some(v) = self.created_at.as_ref() {
            struct_ser.serialize_field("createdAt", v)?;
        }
        if let Some(v) = self.updated_at.as_ref() {
            struct_ser.serialize_field("updatedAt", v)?;
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
            "pid",
            "username",
            "is_admin",
            "isAdmin",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pid,
            Username,
            IsAdmin,
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
                            "pid" => Ok(GeneratedField::Pid),
                            "username" => Ok(GeneratedField::Username),
                            "isAdmin" | "is_admin" => Ok(GeneratedField::IsAdmin),
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
                formatter.write_str("struct memoir.v1.User")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<User, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pid__ = None;
                let mut username__ = None;
                let mut is_admin__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Pid => {
                            if pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pid"));
                            }
                            pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Username => {
                            if username__.is_some() {
                                return Err(serde::de::Error::duplicate_field("username"));
                            }
                            username__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsAdmin => {
                            if is_admin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isAdmin"));
                            }
                            is_admin__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdAt"));
                            }
                            created_at__ = map_.next_value()?;
                        }
                        GeneratedField::UpdatedAt => {
                            if updated_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updatedAt"));
                            }
                            updated_at__ = map_.next_value()?;
                        }
                    }
                }
                Ok(User {
                    pid: pid__.unwrap_or_default(),
                    username: username__.unwrap_or_default(),
                    is_admin: is_admin__.unwrap_or_default(),
                    created_at: created_at__,
                    updated_at: updated_at__,
                })
            }
        }
        deserializer.deserialize_struct("memoir.v1.User", FIELDS, GeneratedVisitor)
    }
}
