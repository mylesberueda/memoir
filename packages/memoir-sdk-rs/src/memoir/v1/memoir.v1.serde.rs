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
impl serde::Serialize for DeleteFailedJobRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("memoir.v1.DeleteFailedJobRequest", len)?;
        if self.id != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("id", ToString::to_string(&self.id).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteFailedJobRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteFailedJobRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct memoir.v1.DeleteFailedJobRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteFailedJobRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(DeleteFailedJobRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("memoir.v1.DeleteFailedJobRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteFailedJobResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("memoir.v1.DeleteFailedJobResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteFailedJobResponse {
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
            type Value = DeleteFailedJobResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct memoir.v1.DeleteFailedJobResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteFailedJobResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(DeleteFailedJobResponse {
                })
            }
        }
        deserializer.deserialize_struct("memoir.v1.DeleteFailedJobResponse", FIELDS, GeneratedVisitor)
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
impl serde::Serialize for FailedJob {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.id != 0 {
            len += 1;
        }
        if !self.source_pid.is_empty() {
            len += 1;
        }
        if self.kind != 0 {
            len += 1;
        }
        if self.attempts != 0 {
            len += 1;
        }
        if self.failure_reason.is_some() {
            len += 1;
        }
        if self.updated_at.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("memoir.v1.FailedJob", len)?;
        if self.id != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("id", ToString::to_string(&self.id).as_str())?;
        }
        if !self.source_pid.is_empty() {
            struct_ser.serialize_field("sourcePid", &self.source_pid)?;
        }
        if self.kind != 0 {
            let v = JobKind::try_from(self.kind)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.kind)))?;
            struct_ser.serialize_field("kind", &v)?;
        }
        if self.attempts != 0 {
            struct_ser.serialize_field("attempts", &self.attempts)?;
        }
        if let Some(v) = self.failure_reason.as_ref() {
            struct_ser.serialize_field("failureReason", v)?;
        }
        if let Some(v) = self.updated_at.as_ref() {
            struct_ser.serialize_field("updatedAt", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FailedJob {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "source_pid",
            "sourcePid",
            "kind",
            "attempts",
            "failure_reason",
            "failureReason",
            "updated_at",
            "updatedAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            SourcePid,
            Kind,
            Attempts,
            FailureReason,
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
                            "sourcePid" | "source_pid" => Ok(GeneratedField::SourcePid),
                            "kind" => Ok(GeneratedField::Kind),
                            "attempts" => Ok(GeneratedField::Attempts),
                            "failureReason" | "failure_reason" => Ok(GeneratedField::FailureReason),
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
            type Value = FailedJob;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct memoir.v1.FailedJob")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FailedJob, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut source_pid__ = None;
                let mut kind__ = None;
                let mut attempts__ = None;
                let mut failure_reason__ = None;
                let mut updated_at__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::SourcePid => {
                            if source_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourcePid"));
                            }
                            source_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Kind => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kind"));
                            }
                            kind__ = Some(map_.next_value::<JobKind>()? as i32);
                        }
                        GeneratedField::Attempts => {
                            if attempts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attempts"));
                            }
                            attempts__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::FailureReason => {
                            if failure_reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("failureReason"));
                            }
                            failure_reason__ = map_.next_value()?;
                        }
                        GeneratedField::UpdatedAt => {
                            if updated_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updatedAt"));
                            }
                            updated_at__ = map_.next_value()?;
                        }
                    }
                }
                Ok(FailedJob {
                    id: id__.unwrap_or_default(),
                    source_pid: source_pid__.unwrap_or_default(),
                    kind: kind__.unwrap_or_default(),
                    attempts: attempts__.unwrap_or_default(),
                    failure_reason: failure_reason__,
                    updated_at: updated_at__,
                })
            }
        }
        deserializer.deserialize_struct("memoir.v1.FailedJob", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FilterCondition {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.field.is_empty() {
            len += 1;
        }
        if self.condition.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("memoir.v1.FilterCondition", len)?;
        if !self.field.is_empty() {
            struct_ser.serialize_field("field", &self.field)?;
        }
        if let Some(v) = self.condition.as_ref() {
            match v {
                filter_condition::Condition::Equals(v) => {
                    struct_ser.serialize_field("equals", v)?;
                }
                filter_condition::Condition::InValues(v) => {
                    struct_ser.serialize_field("inValues", v)?;
                }
                filter_condition::Condition::Range(v) => {
                    struct_ser.serialize_field("range", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FilterCondition {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "field",
            "equals",
            "in_values",
            "inValues",
            "range",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Field,
            Equals,
            InValues,
            Range,
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
                            "field" => Ok(GeneratedField::Field),
                            "equals" => Ok(GeneratedField::Equals),
                            "inValues" | "in_values" => Ok(GeneratedField::InValues),
                            "range" => Ok(GeneratedField::Range),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FilterCondition;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct memoir.v1.FilterCondition")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FilterCondition, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut field__ = None;
                let mut condition__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Field => {
                            if field__.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            field__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Equals => {
                            if condition__.is_some() {
                                return Err(serde::de::Error::duplicate_field("equals"));
                            }
                            condition__ = map_.next_value::<::std::option::Option<_>>()?.map(filter_condition::Condition::Equals)
;
                        }
                        GeneratedField::InValues => {
                            if condition__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inValues"));
                            }
                            condition__ = map_.next_value::<::std::option::Option<_>>()?.map(filter_condition::Condition::InValues)
;
                        }
                        GeneratedField::Range => {
                            if condition__.is_some() {
                                return Err(serde::de::Error::duplicate_field("range"));
                            }
                            condition__ = map_.next_value::<::std::option::Option<_>>()?.map(filter_condition::Condition::Range)
;
                        }
                    }
                }
                Ok(FilterCondition {
                    field: field__.unwrap_or_default(),
                    condition: condition__,
                })
            }
        }
        deserializer.deserialize_struct("memoir.v1.FilterCondition", FIELDS, GeneratedVisitor)
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
        let mut len = 0;
        if !self.deleted_pids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("memoir.v1.ForgetResponse", len)?;
        if !self.deleted_pids.is_empty() {
            struct_ser.serialize_field("deletedPids", &self.deleted_pids)?;
        }
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
            "deleted_pids",
            "deletedPids",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DeletedPids,
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
                            "deletedPids" | "deleted_pids" => Ok(GeneratedField::DeletedPids),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
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
                let mut deleted_pids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DeletedPids => {
                            if deleted_pids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deletedPids"));
                            }
                            deleted_pids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ForgetResponse {
                    deleted_pids: deleted_pids__.unwrap_or_default(),
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
impl serde::Serialize for IntegerList {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.values.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("memoir.v1.IntegerList", len)?;
        if !self.values.is_empty() {
            struct_ser.serialize_field("values", &self.values.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IntegerList {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "values",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Values,
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
                            "values" => Ok(GeneratedField::Values),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IntegerList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct memoir.v1.IntegerList")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<IntegerList, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut values__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Values => {
                            if values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("values"));
                            }
                            values__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                    }
                }
                Ok(IntegerList {
                    values: values__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("memoir.v1.IntegerList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for JobKind {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "JOB_KIND_UNSPECIFIED",
            Self::Embed => "JOB_KIND_EMBED",
            Self::Extract => "JOB_KIND_EXTRACT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for JobKind {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "JOB_KIND_UNSPECIFIED",
            "JOB_KIND_EMBED",
            "JOB_KIND_EXTRACT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = JobKind;

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
                    "JOB_KIND_UNSPECIFIED" => Ok(JobKind::Unspecified),
                    "JOB_KIND_EMBED" => Ok(JobKind::Embed),
                    "JOB_KIND_EXTRACT" => Ok(JobKind::Extract),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for KeywordList {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.values.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("memoir.v1.KeywordList", len)?;
        if !self.values.is_empty() {
            struct_ser.serialize_field("values", &self.values)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for KeywordList {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "values",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Values,
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
                            "values" => Ok(GeneratedField::Values),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = KeywordList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct memoir.v1.KeywordList")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<KeywordList, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut values__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Values => {
                            if values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("values"));
                            }
                            values__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(KeywordList {
                    values: values__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("memoir.v1.KeywordList", FIELDS, GeneratedVisitor)
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
impl serde::Serialize for ListFailedJobsRequest {
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
        let mut struct_ser = serializer.serialize_struct("memoir.v1.ListFailedJobsRequest", len)?;
        if self.limit != 0 {
            struct_ser.serialize_field("limit", &self.limit)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListFailedJobsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "limit",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Limit,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListFailedJobsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct memoir.v1.ListFailedJobsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListFailedJobsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut limit__ = None;
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
                    }
                }
                Ok(ListFailedJobsRequest {
                    limit: limit__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("memoir.v1.ListFailedJobsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListFailedJobsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.jobs.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("memoir.v1.ListFailedJobsResponse", len)?;
        if !self.jobs.is_empty() {
            struct_ser.serialize_field("jobs", &self.jobs)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListFailedJobsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "jobs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Jobs,
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
                            "jobs" => Ok(GeneratedField::Jobs),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListFailedJobsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct memoir.v1.ListFailedJobsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListFailedJobsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut jobs__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Jobs => {
                            if jobs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jobs"));
                            }
                            jobs__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListFailedJobsResponse {
                    jobs: jobs__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("memoir.v1.ListFailedJobsResponse", FIELDS, GeneratedVisitor)
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
impl serde::Serialize for MatchValue {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("memoir.v1.MatchValue", len)?;
        if let Some(v) = self.value.as_ref() {
            match v {
                match_value::Value::Keyword(v) => {
                    struct_ser.serialize_field("keyword", v)?;
                }
                match_value::Value::Integer(v) => {
                    #[allow(clippy::needless_borrow)]
                    #[allow(clippy::needless_borrows_for_generic_args)]
                    struct_ser.serialize_field("integer", ToString::to_string(&v).as_str())?;
                }
                match_value::Value::Boolean(v) => {
                    struct_ser.serialize_field("boolean", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MatchValue {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "keyword",
            "integer",
            "boolean",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Keyword,
            Integer,
            Boolean,
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
                            "keyword" => Ok(GeneratedField::Keyword),
                            "integer" => Ok(GeneratedField::Integer),
                            "boolean" => Ok(GeneratedField::Boolean),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MatchValue;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct memoir.v1.MatchValue")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MatchValue, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Keyword => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyword"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(match_value::Value::Keyword);
                        }
                        GeneratedField::Integer => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("integer"));
                            }
                            value__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| match_value::Value::Integer(x.0));
                        }
                        GeneratedField::Boolean => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("boolean"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(match_value::Value::Boolean);
                        }
                    }
                }
                Ok(MatchValue {
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("memoir.v1.MatchValue", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MatchValues {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.values.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("memoir.v1.MatchValues", len)?;
        if let Some(v) = self.values.as_ref() {
            match v {
                match_values::Values::Keywords(v) => {
                    struct_ser.serialize_field("keywords", v)?;
                }
                match_values::Values::Integers(v) => {
                    struct_ser.serialize_field("integers", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MatchValues {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "keywords",
            "integers",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Keywords,
            Integers,
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
                            "keywords" => Ok(GeneratedField::Keywords),
                            "integers" => Ok(GeneratedField::Integers),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MatchValues;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct memoir.v1.MatchValues")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MatchValues, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut values__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Keywords => {
                            if values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keywords"));
                            }
                            values__ = map_.next_value::<::std::option::Option<_>>()?.map(match_values::Values::Keywords)
;
                        }
                        GeneratedField::Integers => {
                            if values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("integers"));
                            }
                            values__ = map_.next_value::<::std::option::Option<_>>()?.map(match_values::Values::Integers)
;
                        }
                    }
                }
                Ok(MatchValues {
                    values: values__,
                })
            }
        }
        deserializer.deserialize_struct("memoir.v1.MatchValues", FIELDS, GeneratedVisitor)
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
impl serde::Serialize for MemoryFilter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.must.is_empty() {
            len += 1;
        }
        if !self.must_not.is_empty() {
            len += 1;
        }
        if !self.should.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("memoir.v1.MemoryFilter", len)?;
        if !self.must.is_empty() {
            struct_ser.serialize_field("must", &self.must)?;
        }
        if !self.must_not.is_empty() {
            struct_ser.serialize_field("mustNot", &self.must_not)?;
        }
        if !self.should.is_empty() {
            struct_ser.serialize_field("should", &self.should)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MemoryFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "must",
            "must_not",
            "mustNot",
            "should",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Must,
            MustNot,
            Should,
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
                            "must" => Ok(GeneratedField::Must),
                            "mustNot" | "must_not" => Ok(GeneratedField::MustNot),
                            "should" => Ok(GeneratedField::Should),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MemoryFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct memoir.v1.MemoryFilter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MemoryFilter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut must__ = None;
                let mut must_not__ = None;
                let mut should__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Must => {
                            if must__.is_some() {
                                return Err(serde::de::Error::duplicate_field("must"));
                            }
                            must__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MustNot => {
                            if must_not__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mustNot"));
                            }
                            must_not__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Should => {
                            if should__.is_some() {
                                return Err(serde::de::Error::duplicate_field("should"));
                            }
                            should__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MemoryFilter {
                    must: must__.unwrap_or_default(),
                    must_not: must_not__.unwrap_or_default(),
                    should: should__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("memoir.v1.MemoryFilter", FIELDS, GeneratedVisitor)
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
impl serde::Serialize for NumericRange {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.lt.is_some() {
            len += 1;
        }
        if self.lte.is_some() {
            len += 1;
        }
        if self.gt.is_some() {
            len += 1;
        }
        if self.gte.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("memoir.v1.NumericRange", len)?;
        if let Some(v) = self.lt.as_ref() {
            struct_ser.serialize_field("lt", v)?;
        }
        if let Some(v) = self.lte.as_ref() {
            struct_ser.serialize_field("lte", v)?;
        }
        if let Some(v) = self.gt.as_ref() {
            struct_ser.serialize_field("gt", v)?;
        }
        if let Some(v) = self.gte.as_ref() {
            struct_ser.serialize_field("gte", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NumericRange {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "lt",
            "lte",
            "gt",
            "gte",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Lt,
            Lte,
            Gt,
            Gte,
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
                            "lt" => Ok(GeneratedField::Lt),
                            "lte" => Ok(GeneratedField::Lte),
                            "gt" => Ok(GeneratedField::Gt),
                            "gte" => Ok(GeneratedField::Gte),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NumericRange;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct memoir.v1.NumericRange")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<NumericRange, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut lt__ = None;
                let mut lte__ = None;
                let mut gt__ = None;
                let mut gte__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Lt => {
                            if lt__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lt"));
                            }
                            lt__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Lte => {
                            if lte__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lte"));
                            }
                            lte__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Gt => {
                            if gt__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gt"));
                            }
                            gt__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Gte => {
                            if gte__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gte"));
                            }
                            gte__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(NumericRange {
                    lt: lt__,
                    lte: lte__,
                    gt: gt__,
                    gte: gte__,
                })
            }
        }
        deserializer.deserialize_struct("memoir.v1.NumericRange", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PendingJobsCountRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("memoir.v1.PendingJobsCountRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PendingJobsCountRequest {
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
            type Value = PendingJobsCountRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct memoir.v1.PendingJobsCountRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PendingJobsCountRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(PendingJobsCountRequest {
                })
            }
        }
        deserializer.deserialize_struct("memoir.v1.PendingJobsCountRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PendingJobsCountResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.count != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("memoir.v1.PendingJobsCountResponse", len)?;
        if self.count != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("count", ToString::to_string(&self.count).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PendingJobsCountResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "count",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Count,
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
                            "count" => Ok(GeneratedField::Count),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PendingJobsCountResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct memoir.v1.PendingJobsCountResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PendingJobsCountResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut count__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Count => {
                            if count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("count"));
                            }
                            count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(PendingJobsCountResponse {
                    count: count__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("memoir.v1.PendingJobsCountResponse", FIELDS, GeneratedVisitor)
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
impl serde::Serialize for ReconcileRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.only_retry_failed {
            len += 1;
        }
        if self.only_clean_orphans {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("memoir.v1.ReconcileRequest", len)?;
        if self.only_retry_failed {
            struct_ser.serialize_field("onlyRetryFailed", &self.only_retry_failed)?;
        }
        if self.only_clean_orphans {
            struct_ser.serialize_field("onlyCleanOrphans", &self.only_clean_orphans)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReconcileRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "only_retry_failed",
            "onlyRetryFailed",
            "only_clean_orphans",
            "onlyCleanOrphans",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OnlyRetryFailed,
            OnlyCleanOrphans,
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
                            "onlyRetryFailed" | "only_retry_failed" => Ok(GeneratedField::OnlyRetryFailed),
                            "onlyCleanOrphans" | "only_clean_orphans" => Ok(GeneratedField::OnlyCleanOrphans),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReconcileRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct memoir.v1.ReconcileRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReconcileRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut only_retry_failed__ = None;
                let mut only_clean_orphans__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OnlyRetryFailed => {
                            if only_retry_failed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("onlyRetryFailed"));
                            }
                            only_retry_failed__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OnlyCleanOrphans => {
                            if only_clean_orphans__.is_some() {
                                return Err(serde::de::Error::duplicate_field("onlyCleanOrphans"));
                            }
                            only_clean_orphans__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ReconcileRequest {
                    only_retry_failed: only_retry_failed__.unwrap_or_default(),
                    only_clean_orphans: only_clean_orphans__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("memoir.v1.ReconcileRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReconcileResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.failed_retried != 0 {
            len += 1;
        }
        if self.failed_recovered != 0 {
            len += 1;
        }
        if self.orphans_deleted != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("memoir.v1.ReconcileResponse", len)?;
        if self.failed_retried != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("failedRetried", ToString::to_string(&self.failed_retried).as_str())?;
        }
        if self.failed_recovered != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("failedRecovered", ToString::to_string(&self.failed_recovered).as_str())?;
        }
        if self.orphans_deleted != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("orphansDeleted", ToString::to_string(&self.orphans_deleted).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReconcileResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "failed_retried",
            "failedRetried",
            "failed_recovered",
            "failedRecovered",
            "orphans_deleted",
            "orphansDeleted",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FailedRetried,
            FailedRecovered,
            OrphansDeleted,
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
                            "failedRetried" | "failed_retried" => Ok(GeneratedField::FailedRetried),
                            "failedRecovered" | "failed_recovered" => Ok(GeneratedField::FailedRecovered),
                            "orphansDeleted" | "orphans_deleted" => Ok(GeneratedField::OrphansDeleted),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReconcileResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct memoir.v1.ReconcileResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReconcileResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut failed_retried__ = None;
                let mut failed_recovered__ = None;
                let mut orphans_deleted__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FailedRetried => {
                            if failed_retried__.is_some() {
                                return Err(serde::de::Error::duplicate_field("failedRetried"));
                            }
                            failed_retried__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::FailedRecovered => {
                            if failed_recovered__.is_some() {
                                return Err(serde::de::Error::duplicate_field("failedRecovered"));
                            }
                            failed_recovered__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::OrphansDeleted => {
                            if orphans_deleted__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orphansDeleted"));
                            }
                            orphans_deleted__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ReconcileResponse {
                    failed_retried: failed_retried__.unwrap_or_default(),
                    failed_recovered: failed_recovered__.unwrap_or_default(),
                    orphans_deleted: orphans_deleted__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("memoir.v1.ReconcileResponse", FIELDS, GeneratedVisitor)
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
impl serde::Serialize for RetryFailedJobsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.of_kind.is_some() {
            len += 1;
        }
        if self.dry_run {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("memoir.v1.RetryFailedJobsRequest", len)?;
        if let Some(v) = self.of_kind.as_ref() {
            let v = JobKind::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("ofKind", &v)?;
        }
        if self.dry_run {
            struct_ser.serialize_field("dryRun", &self.dry_run)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RetryFailedJobsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "of_kind",
            "ofKind",
            "dry_run",
            "dryRun",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OfKind,
            DryRun,
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
                            "ofKind" | "of_kind" => Ok(GeneratedField::OfKind),
                            "dryRun" | "dry_run" => Ok(GeneratedField::DryRun),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RetryFailedJobsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct memoir.v1.RetryFailedJobsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RetryFailedJobsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut of_kind__ = None;
                let mut dry_run__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OfKind => {
                            if of_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ofKind"));
                            }
                            of_kind__ = map_.next_value::<::std::option::Option<JobKind>>()?.map(|x| x as i32);
                        }
                        GeneratedField::DryRun => {
                            if dry_run__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dryRun"));
                            }
                            dry_run__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RetryFailedJobsRequest {
                    of_kind: of_kind__,
                    dry_run: dry_run__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("memoir.v1.RetryFailedJobsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RetryFailedJobsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.affected != 0 {
            len += 1;
        }
        if self.dry_run {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("memoir.v1.RetryFailedJobsResponse", len)?;
        if self.affected != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("affected", ToString::to_string(&self.affected).as_str())?;
        }
        if self.dry_run {
            struct_ser.serialize_field("dryRun", &self.dry_run)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RetryFailedJobsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "affected",
            "dry_run",
            "dryRun",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Affected,
            DryRun,
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
                            "affected" => Ok(GeneratedField::Affected),
                            "dryRun" | "dry_run" => Ok(GeneratedField::DryRun),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RetryFailedJobsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct memoir.v1.RetryFailedJobsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RetryFailedJobsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut affected__ = None;
                let mut dry_run__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Affected => {
                            if affected__.is_some() {
                                return Err(serde::de::Error::duplicate_field("affected"));
                            }
                            affected__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::DryRun => {
                            if dry_run__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dryRun"));
                            }
                            dry_run__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RetryFailedJobsResponse {
                    affected: affected__.unwrap_or_default(),
                    dry_run: dry_run__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("memoir.v1.RetryFailedJobsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RetryJobRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("memoir.v1.RetryJobRequest", len)?;
        if self.id != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("id", ToString::to_string(&self.id).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RetryJobRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RetryJobRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct memoir.v1.RetryJobRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RetryJobRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(RetryJobRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("memoir.v1.RetryJobRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RetryJobResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("memoir.v1.RetryJobResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RetryJobResponse {
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
            type Value = RetryJobResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct memoir.v1.RetryJobResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RetryJobResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(RetryJobResponse {
                })
            }
        }
        deserializer.deserialize_struct("memoir.v1.RetryJobResponse", FIELDS, GeneratedVisitor)
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
        if self.min_similarity.is_some() {
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
        if let Some(v) = self.min_similarity.as_ref() {
            struct_ser.serialize_field("minSimilarity", v)?;
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
            "min_similarity",
            "minSimilarity",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Scope,
            Query,
            Limit,
            MetadataFilter,
            MinSimilarity,
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
                            "minSimilarity" | "min_similarity" => Ok(GeneratedField::MinSimilarity),
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
                let mut min_similarity__ = None;
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
                        GeneratedField::MinSimilarity => {
                            if min_similarity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minSimilarity"));
                            }
                            min_similarity__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(SearchRequest {
                    scope: scope__,
                    query: query__.unwrap_or_default(),
                    limit: limit__.unwrap_or_default(),
                    metadata_filter: metadata_filter__,
                    min_similarity: min_similarity__,
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
impl serde::Serialize for UnsupersedeRequest {
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
        let mut struct_ser = serializer.serialize_struct("memoir.v1.UnsupersedeRequest", len)?;
        if !self.pid.is_empty() {
            struct_ser.serialize_field("pid", &self.pid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UnsupersedeRequest {
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
            type Value = UnsupersedeRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct memoir.v1.UnsupersedeRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UnsupersedeRequest, V::Error>
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
                Ok(UnsupersedeRequest {
                    pid: pid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("memoir.v1.UnsupersedeRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UnsupersedeResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("memoir.v1.UnsupersedeResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UnsupersedeResponse {
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
            type Value = UnsupersedeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct memoir.v1.UnsupersedeResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UnsupersedeResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(UnsupersedeResponse {
                })
            }
        }
        deserializer.deserialize_struct("memoir.v1.UnsupersedeResponse", FIELDS, GeneratedVisitor)
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
