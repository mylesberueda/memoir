// @generated
impl serde::Serialize for AddDocumentsToGroupRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.group_pid.is_empty() {
            len += 1;
        }
        if !self.document_pids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.AddDocumentsToGroupRequest", len)?;
        if !self.group_pid.is_empty() {
            struct_ser.serialize_field("groupPid", &self.group_pid)?;
        }
        if !self.document_pids.is_empty() {
            struct_ser.serialize_field("documentPids", &self.document_pids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddDocumentsToGroupRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "group_pid",
            "groupPid",
            "document_pids",
            "documentPids",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            GroupPid,
            DocumentPids,
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
                            "groupPid" | "group_pid" => Ok(GeneratedField::GroupPid),
                            "documentPids" | "document_pids" => Ok(GeneratedField::DocumentPids),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AddDocumentsToGroupRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.AddDocumentsToGroupRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddDocumentsToGroupRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut group_pid__ = None;
                let mut document_pids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::GroupPid => {
                            if group_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("groupPid"));
                            }
                            group_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DocumentPids => {
                            if document_pids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("documentPids"));
                            }
                            document_pids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AddDocumentsToGroupRequest {
                    group_pid: group_pid__.unwrap_or_default(),
                    document_pids: document_pids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.AddDocumentsToGroupRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AddDocumentsToGroupResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.group.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.AddDocumentsToGroupResponse", len)?;
        if let Some(v) = self.group.as_ref() {
            struct_ser.serialize_field("group", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddDocumentsToGroupResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "group",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Group,
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
                            "group" => Ok(GeneratedField::Group),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AddDocumentsToGroupResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.AddDocumentsToGroupResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddDocumentsToGroupResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut group__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Group => {
                            if group__.is_some() {
                                return Err(serde::de::Error::duplicate_field("group"));
                            }
                            group__ = map_.next_value()?;
                        }
                    }
                }
                Ok(AddDocumentsToGroupResponse {
                    group: group__,
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.AddDocumentsToGroupResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Agent {
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
        if self.kind != 0 {
            len += 1;
        }
        if self.model.is_some() {
            len += 1;
        }
        if self.temperature != 0 {
            len += 1;
        }
        if !self.system_prompt.is_empty() {
            len += 1;
        }
        if !self.tools.is_empty() {
            len += 1;
        }
        if self.config.is_some() {
            len += 1;
        }
        if self.is_active {
            len += 1;
        }
        if !self.created_by_user_id.is_empty() {
            len += 1;
        }
        if !self.created_at.is_empty() {
            len += 1;
        }
        if !self.updated_at.is_empty() {
            len += 1;
        }
        if self.identifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.Agent", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.slug.is_empty() {
            struct_ser.serialize_field("slug", &self.slug)?;
        }
        if self.kind != 0 {
            let v = AgentKind::try_from(self.kind)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.kind)))?;
            struct_ser.serialize_field("kind", &v)?;
        }
        if let Some(v) = self.model.as_ref() {
            struct_ser.serialize_field("model", v)?;
        }
        if self.temperature != 0 {
            struct_ser.serialize_field("temperature", &self.temperature)?;
        }
        if !self.system_prompt.is_empty() {
            struct_ser.serialize_field("systemPrompt", &self.system_prompt)?;
        }
        if !self.tools.is_empty() {
            struct_ser.serialize_field("tools", &self.tools)?;
        }
        if let Some(v) = self.config.as_ref() {
            struct_ser.serialize_field("config", v)?;
        }
        if self.is_active {
            struct_ser.serialize_field("isActive", &self.is_active)?;
        }
        if !self.created_by_user_id.is_empty() {
            struct_ser.serialize_field("createdByUserId", &self.created_by_user_id)?;
        }
        if !self.created_at.is_empty() {
            struct_ser.serialize_field("createdAt", &self.created_at)?;
        }
        if !self.updated_at.is_empty() {
            struct_ser.serialize_field("updatedAt", &self.updated_at)?;
        }
        if let Some(v) = self.identifier.as_ref() {
            match v {
                agent::Identifier::Id(v) => {
                    struct_ser.serialize_field("id", v)?;
                }
                agent::Identifier::Pid(v) => {
                    struct_ser.serialize_field("pid", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Agent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "slug",
            "kind",
            "model",
            "temperature",
            "system_prompt",
            "systemPrompt",
            "tools",
            "config",
            "is_active",
            "isActive",
            "created_by_user_id",
            "createdByUserId",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
            "id",
            "pid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Slug,
            Kind,
            Model,
            Temperature,
            SystemPrompt,
            Tools,
            Config,
            IsActive,
            CreatedByUserId,
            CreatedAt,
            UpdatedAt,
            Id,
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
                            "name" => Ok(GeneratedField::Name),
                            "slug" => Ok(GeneratedField::Slug),
                            "kind" => Ok(GeneratedField::Kind),
                            "model" => Ok(GeneratedField::Model),
                            "temperature" => Ok(GeneratedField::Temperature),
                            "systemPrompt" | "system_prompt" => Ok(GeneratedField::SystemPrompt),
                            "tools" => Ok(GeneratedField::Tools),
                            "config" => Ok(GeneratedField::Config),
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
                            "createdByUserId" | "created_by_user_id" => Ok(GeneratedField::CreatedByUserId),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            "id" => Ok(GeneratedField::Id),
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
            type Value = Agent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.Agent")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Agent, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut slug__ = None;
                let mut kind__ = None;
                let mut model__ = None;
                let mut temperature__ = None;
                let mut system_prompt__ = None;
                let mut tools__ = None;
                let mut config__ = None;
                let mut is_active__ = None;
                let mut created_by_user_id__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                let mut identifier__ = None;
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
                        GeneratedField::Kind => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kind"));
                            }
                            kind__ = Some(map_.next_value::<AgentKind>()? as i32);
                        }
                        GeneratedField::Model => {
                            if model__.is_some() {
                                return Err(serde::de::Error::duplicate_field("model"));
                            }
                            model__ = map_.next_value()?;
                        }
                        GeneratedField::Temperature => {
                            if temperature__.is_some() {
                                return Err(serde::de::Error::duplicate_field("temperature"));
                            }
                            temperature__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::SystemPrompt => {
                            if system_prompt__.is_some() {
                                return Err(serde::de::Error::duplicate_field("systemPrompt"));
                            }
                            system_prompt__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Tools => {
                            if tools__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tools"));
                            }
                            tools__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Config => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config__ = map_.next_value()?;
                        }
                        GeneratedField::IsActive => {
                            if is_active__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isActive"));
                            }
                            is_active__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CreatedByUserId => {
                            if created_by_user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdByUserId"));
                            }
                            created_by_user_id__ = Some(map_.next_value()?);
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
                        GeneratedField::Id => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            identifier__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| agent::Identifier::Id(x.0));
                        }
                        GeneratedField::Pid => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pid"));
                            }
                            identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(agent::Identifier::Pid);
                        }
                    }
                }
                Ok(Agent {
                    name: name__.unwrap_or_default(),
                    slug: slug__.unwrap_or_default(),
                    kind: kind__.unwrap_or_default(),
                    model: model__,
                    temperature: temperature__.unwrap_or_default(),
                    system_prompt: system_prompt__.unwrap_or_default(),
                    tools: tools__.unwrap_or_default(),
                    config: config__,
                    is_active: is_active__.unwrap_or_default(),
                    created_by_user_id: created_by_user_id__.unwrap_or_default(),
                    created_at: created_at__.unwrap_or_default(),
                    updated_at: updated_at__.unwrap_or_default(),
                    identifier: identifier__,
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.Agent", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AgentConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.base.is_some() {
            len += 1;
        }
        if self.kind_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.AgentConfig", len)?;
        if let Some(v) = self.base.as_ref() {
            struct_ser.serialize_field("base", v)?;
        }
        if let Some(v) = self.kind_config.as_ref() {
            match v {
                agent_config::KindConfig::Startup(v) => {
                    struct_ser.serialize_field("startup", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AgentConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "base",
            "startup",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Base,
            Startup,
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
                            "base" => Ok(GeneratedField::Base),
                            "startup" => Ok(GeneratedField::Startup),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AgentConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.AgentConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AgentConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut base__ = None;
                let mut kind_config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Base => {
                            if base__.is_some() {
                                return Err(serde::de::Error::duplicate_field("base"));
                            }
                            base__ = map_.next_value()?;
                        }
                        GeneratedField::Startup => {
                            if kind_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startup"));
                            }
                            kind_config__ = map_.next_value::<::std::option::Option<_>>()?.map(agent_config::KindConfig::Startup)
;
                        }
                    }
                }
                Ok(AgentConfig {
                    base: base__,
                    kind_config: kind_config__,
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.AgentConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AgentKind {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "AGENT_KIND_UNSPECIFIED",
            Self::Startup => "AGENT_KIND_STARTUP",
            Self::Ephemeral => "AGENT_KIND_EPHEMERAL",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for AgentKind {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "AGENT_KIND_UNSPECIFIED",
            "AGENT_KIND_STARTUP",
            "AGENT_KIND_EPHEMERAL",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AgentKind;

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
                    "AGENT_KIND_UNSPECIFIED" => Ok(AgentKind::Unspecified),
                    "AGENT_KIND_STARTUP" => Ok(AgentKind::Startup),
                    "AGENT_KIND_EPHEMERAL" => Ok(AgentKind::Ephemeral),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for AgentModel {
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
        if !self.model_id.is_empty() {
            len += 1;
        }
        if self.provider.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.AgentModel", len)?;
        if !self.pid.is_empty() {
            struct_ser.serialize_field("pid", &self.pid)?;
        }
        if !self.model_id.is_empty() {
            struct_ser.serialize_field("modelId", &self.model_id)?;
        }
        if let Some(v) = self.provider.as_ref() {
            struct_ser.serialize_field("provider", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AgentModel {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pid",
            "model_id",
            "modelId",
            "provider",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pid,
            ModelId,
            Provider,
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
                            "modelId" | "model_id" => Ok(GeneratedField::ModelId),
                            "provider" => Ok(GeneratedField::Provider),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AgentModel;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.AgentModel")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AgentModel, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pid__ = None;
                let mut model_id__ = None;
                let mut provider__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Pid => {
                            if pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pid"));
                            }
                            pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ModelId => {
                            if model_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("modelId"));
                            }
                            model_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Provider => {
                            if provider__.is_some() {
                                return Err(serde::de::Error::duplicate_field("provider"));
                            }
                            provider__ = map_.next_value()?;
                        }
                    }
                }
                Ok(AgentModel {
                    pid: pid__.unwrap_or_default(),
                    model_id: model_id__.unwrap_or_default(),
                    provider: provider__,
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.AgentModel", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AgentProvider {
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
        let mut struct_ser = serializer.serialize_struct("rig.v1.AgentProvider", len)?;
        if !self.pid.is_empty() {
            struct_ser.serialize_field("pid", &self.pid)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AgentProvider {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pid",
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pid,
            Name,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AgentProvider;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.AgentProvider")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AgentProvider, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pid__ = None;
                let mut name__ = None;
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
                    }
                }
                Ok(AgentProvider {
                    pid: pid__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.AgentProvider", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AgentShare {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user_id.is_empty() {
            len += 1;
        }
        if self.permissions != 0 {
            len += 1;
        }
        if !self.shared_by.is_empty() {
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
        let mut struct_ser = serializer.serialize_struct("rig.v1.AgentShare", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if self.permissions != 0 {
            struct_ser.serialize_field("permissions", &self.permissions)?;
        }
        if !self.shared_by.is_empty() {
            struct_ser.serialize_field("sharedBy", &self.shared_by)?;
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
impl<'de> serde::Deserialize<'de> for AgentShare {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
            "permissions",
            "shared_by",
            "sharedBy",
            "created_at",
            "createdAt",
            "display_name",
            "displayName",
            "email",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
            Permissions,
            SharedBy,
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
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
                            "permissions" => Ok(GeneratedField::Permissions),
                            "sharedBy" | "shared_by" => Ok(GeneratedField::SharedBy),
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
            type Value = AgentShare;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.AgentShare")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AgentShare, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut permissions__ = None;
                let mut shared_by__ = None;
                let mut created_at__ = None;
                let mut display_name__ = None;
                let mut email__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Permissions => {
                            if permissions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("permissions"));
                            }
                            permissions__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::SharedBy => {
                            if shared_by__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sharedBy"));
                            }
                            shared_by__ = Some(map_.next_value()?);
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
                Ok(AgentShare {
                    user_id: user_id__.unwrap_or_default(),
                    permissions: permissions__.unwrap_or_default(),
                    shared_by: shared_by__.unwrap_or_default(),
                    created_at: created_at__.unwrap_or_default(),
                    display_name: display_name__,
                    email: email__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.AgentShare", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AttachDocumentsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.conversation_pid.is_empty() {
            len += 1;
        }
        if !self.document_pids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.AttachDocumentsRequest", len)?;
        if !self.conversation_pid.is_empty() {
            struct_ser.serialize_field("conversationPid", &self.conversation_pid)?;
        }
        if !self.document_pids.is_empty() {
            struct_ser.serialize_field("documentPids", &self.document_pids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AttachDocumentsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "conversation_pid",
            "conversationPid",
            "document_pids",
            "documentPids",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ConversationPid,
            DocumentPids,
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
                            "conversationPid" | "conversation_pid" => Ok(GeneratedField::ConversationPid),
                            "documentPids" | "document_pids" => Ok(GeneratedField::DocumentPids),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AttachDocumentsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.AttachDocumentsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AttachDocumentsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut conversation_pid__ = None;
                let mut document_pids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ConversationPid => {
                            if conversation_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("conversationPid"));
                            }
                            conversation_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DocumentPids => {
                            if document_pids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("documentPids"));
                            }
                            document_pids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AttachDocumentsRequest {
                    conversation_pid: conversation_pid__.unwrap_or_default(),
                    document_pids: document_pids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.AttachDocumentsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AttachDocumentsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.attached_count != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.AttachDocumentsResponse", len)?;
        if self.attached_count != 0 {
            struct_ser.serialize_field("attachedCount", &self.attached_count)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AttachDocumentsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "attached_count",
            "attachedCount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AttachedCount,
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
                            "attachedCount" | "attached_count" => Ok(GeneratedField::AttachedCount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AttachDocumentsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.AttachDocumentsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AttachDocumentsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut attached_count__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AttachedCount => {
                            if attached_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attachedCount"));
                            }
                            attached_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(AttachDocumentsResponse {
                    attached_count: attached_count__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.AttachDocumentsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BaseAgentConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.history_length.is_some() {
            len += 1;
        }
        if self.thinking_enabled.is_some() {
            len += 1;
        }
        if self.streaming_enabled.is_some() {
            len += 1;
        }
        if self.timeout_seconds.is_some() {
            len += 1;
        }
        if self.session_ttl_seconds.is_some() {
            len += 1;
        }
        if self.max_tokens.is_some() {
            len += 1;
        }
        if self.memory_enabled.is_some() {
            len += 1;
        }
        if self.memory_result_count.is_some() {
            len += 1;
        }
        if self.memory_similarity_threshold.is_some() {
            len += 1;
        }
        if self.compaction_threshold.is_some() {
            len += 1;
        }
        if self.compaction_keep_ratio.is_some() {
            len += 1;
        }
        if self.document_result_count.is_some() {
            len += 1;
        }
        if self.idle_timeout_seconds.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.BaseAgentConfig", len)?;
        if let Some(v) = self.history_length.as_ref() {
            struct_ser.serialize_field("historyLength", v)?;
        }
        if let Some(v) = self.thinking_enabled.as_ref() {
            struct_ser.serialize_field("thinkingEnabled", v)?;
        }
        if let Some(v) = self.streaming_enabled.as_ref() {
            struct_ser.serialize_field("streamingEnabled", v)?;
        }
        if let Some(v) = self.timeout_seconds.as_ref() {
            struct_ser.serialize_field("timeoutSeconds", v)?;
        }
        if let Some(v) = self.session_ttl_seconds.as_ref() {
            struct_ser.serialize_field("sessionTtlSeconds", v)?;
        }
        if let Some(v) = self.max_tokens.as_ref() {
            struct_ser.serialize_field("maxTokens", v)?;
        }
        if let Some(v) = self.memory_enabled.as_ref() {
            struct_ser.serialize_field("memoryEnabled", v)?;
        }
        if let Some(v) = self.memory_result_count.as_ref() {
            struct_ser.serialize_field("memoryResultCount", v)?;
        }
        if let Some(v) = self.memory_similarity_threshold.as_ref() {
            struct_ser.serialize_field("memorySimilarityThreshold", v)?;
        }
        if let Some(v) = self.compaction_threshold.as_ref() {
            struct_ser.serialize_field("compactionThreshold", v)?;
        }
        if let Some(v) = self.compaction_keep_ratio.as_ref() {
            struct_ser.serialize_field("compactionKeepRatio", v)?;
        }
        if let Some(v) = self.document_result_count.as_ref() {
            struct_ser.serialize_field("documentResultCount", v)?;
        }
        if let Some(v) = self.idle_timeout_seconds.as_ref() {
            struct_ser.serialize_field("idleTimeoutSeconds", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BaseAgentConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "history_length",
            "historyLength",
            "thinking_enabled",
            "thinkingEnabled",
            "streaming_enabled",
            "streamingEnabled",
            "timeout_seconds",
            "timeoutSeconds",
            "session_ttl_seconds",
            "sessionTtlSeconds",
            "max_tokens",
            "maxTokens",
            "memory_enabled",
            "memoryEnabled",
            "memory_result_count",
            "memoryResultCount",
            "memory_similarity_threshold",
            "memorySimilarityThreshold",
            "compaction_threshold",
            "compactionThreshold",
            "compaction_keep_ratio",
            "compactionKeepRatio",
            "document_result_count",
            "documentResultCount",
            "idle_timeout_seconds",
            "idleTimeoutSeconds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HistoryLength,
            ThinkingEnabled,
            StreamingEnabled,
            TimeoutSeconds,
            SessionTtlSeconds,
            MaxTokens,
            MemoryEnabled,
            MemoryResultCount,
            MemorySimilarityThreshold,
            CompactionThreshold,
            CompactionKeepRatio,
            DocumentResultCount,
            IdleTimeoutSeconds,
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
                            "historyLength" | "history_length" => Ok(GeneratedField::HistoryLength),
                            "thinkingEnabled" | "thinking_enabled" => Ok(GeneratedField::ThinkingEnabled),
                            "streamingEnabled" | "streaming_enabled" => Ok(GeneratedField::StreamingEnabled),
                            "timeoutSeconds" | "timeout_seconds" => Ok(GeneratedField::TimeoutSeconds),
                            "sessionTtlSeconds" | "session_ttl_seconds" => Ok(GeneratedField::SessionTtlSeconds),
                            "maxTokens" | "max_tokens" => Ok(GeneratedField::MaxTokens),
                            "memoryEnabled" | "memory_enabled" => Ok(GeneratedField::MemoryEnabled),
                            "memoryResultCount" | "memory_result_count" => Ok(GeneratedField::MemoryResultCount),
                            "memorySimilarityThreshold" | "memory_similarity_threshold" => Ok(GeneratedField::MemorySimilarityThreshold),
                            "compactionThreshold" | "compaction_threshold" => Ok(GeneratedField::CompactionThreshold),
                            "compactionKeepRatio" | "compaction_keep_ratio" => Ok(GeneratedField::CompactionKeepRatio),
                            "documentResultCount" | "document_result_count" => Ok(GeneratedField::DocumentResultCount),
                            "idleTimeoutSeconds" | "idle_timeout_seconds" => Ok(GeneratedField::IdleTimeoutSeconds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BaseAgentConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.BaseAgentConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BaseAgentConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut history_length__ = None;
                let mut thinking_enabled__ = None;
                let mut streaming_enabled__ = None;
                let mut timeout_seconds__ = None;
                let mut session_ttl_seconds__ = None;
                let mut max_tokens__ = None;
                let mut memory_enabled__ = None;
                let mut memory_result_count__ = None;
                let mut memory_similarity_threshold__ = None;
                let mut compaction_threshold__ = None;
                let mut compaction_keep_ratio__ = None;
                let mut document_result_count__ = None;
                let mut idle_timeout_seconds__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::HistoryLength => {
                            if history_length__.is_some() {
                                return Err(serde::de::Error::duplicate_field("historyLength"));
                            }
                            history_length__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::ThinkingEnabled => {
                            if thinking_enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("thinkingEnabled"));
                            }
                            thinking_enabled__ = map_.next_value()?;
                        }
                        GeneratedField::StreamingEnabled => {
                            if streaming_enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("streamingEnabled"));
                            }
                            streaming_enabled__ = map_.next_value()?;
                        }
                        GeneratedField::TimeoutSeconds => {
                            if timeout_seconds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeoutSeconds"));
                            }
                            timeout_seconds__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::SessionTtlSeconds => {
                            if session_ttl_seconds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sessionTtlSeconds"));
                            }
                            session_ttl_seconds__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::MaxTokens => {
                            if max_tokens__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxTokens"));
                            }
                            max_tokens__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::MemoryEnabled => {
                            if memory_enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("memoryEnabled"));
                            }
                            memory_enabled__ = map_.next_value()?;
                        }
                        GeneratedField::MemoryResultCount => {
                            if memory_result_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("memoryResultCount"));
                            }
                            memory_result_count__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::MemorySimilarityThreshold => {
                            if memory_similarity_threshold__.is_some() {
                                return Err(serde::de::Error::duplicate_field("memorySimilarityThreshold"));
                            }
                            memory_similarity_threshold__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::CompactionThreshold => {
                            if compaction_threshold__.is_some() {
                                return Err(serde::de::Error::duplicate_field("compactionThreshold"));
                            }
                            compaction_threshold__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::CompactionKeepRatio => {
                            if compaction_keep_ratio__.is_some() {
                                return Err(serde::de::Error::duplicate_field("compactionKeepRatio"));
                            }
                            compaction_keep_ratio__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::DocumentResultCount => {
                            if document_result_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("documentResultCount"));
                            }
                            document_result_count__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::IdleTimeoutSeconds => {
                            if idle_timeout_seconds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("idleTimeoutSeconds"));
                            }
                            idle_timeout_seconds__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(BaseAgentConfig {
                    history_length: history_length__,
                    thinking_enabled: thinking_enabled__,
                    streaming_enabled: streaming_enabled__,
                    timeout_seconds: timeout_seconds__,
                    session_ttl_seconds: session_ttl_seconds__,
                    max_tokens: max_tokens__,
                    memory_enabled: memory_enabled__,
                    memory_result_count: memory_result_count__,
                    memory_similarity_threshold: memory_similarity_threshold__,
                    compaction_threshold: compaction_threshold__,
                    compaction_keep_ratio: compaction_keep_ratio__,
                    document_result_count: document_result_count__,
                    idle_timeout_seconds: idle_timeout_seconds__,
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.BaseAgentConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CancelInferenceRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.request_id.is_empty() {
            len += 1;
        }
        if !self.conversation_pid.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.CancelInferenceRequest", len)?;
        if !self.request_id.is_empty() {
            struct_ser.serialize_field("requestId", &self.request_id)?;
        }
        if !self.conversation_pid.is_empty() {
            struct_ser.serialize_field("conversationPid", &self.conversation_pid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CancelInferenceRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "request_id",
            "requestId",
            "conversation_pid",
            "conversationPid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RequestId,
            ConversationPid,
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
                            "requestId" | "request_id" => Ok(GeneratedField::RequestId),
                            "conversationPid" | "conversation_pid" => Ok(GeneratedField::ConversationPid),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CancelInferenceRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.CancelInferenceRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CancelInferenceRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut request_id__ = None;
                let mut conversation_pid__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RequestId => {
                            if request_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestId"));
                            }
                            request_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ConversationPid => {
                            if conversation_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("conversationPid"));
                            }
                            conversation_pid__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CancelInferenceRequest {
                    request_id: request_id__.unwrap_or_default(),
                    conversation_pid: conversation_pid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.CancelInferenceRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CancelInferenceResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.cancelled {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.CancelInferenceResponse", len)?;
        if self.cancelled {
            struct_ser.serialize_field("cancelled", &self.cancelled)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CancelInferenceResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cancelled",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Cancelled,
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
                            "cancelled" => Ok(GeneratedField::Cancelled),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CancelInferenceResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.CancelInferenceResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CancelInferenceResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cancelled__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Cancelled => {
                            if cancelled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cancelled"));
                            }
                            cancelled__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CancelInferenceResponse {
                    cancelled: cancelled__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.CancelInferenceResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Conversation {
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
        if self.user_id.is_some() {
            len += 1;
        }
        if !self.agent_pid.is_empty() {
            len += 1;
        }
        if self.title.is_some() {
            len += 1;
        }
        if !self.messages.is_empty() {
            len += 1;
        }
        if self.message_count != 0 {
            len += 1;
        }
        if self.last_message_at.is_some() {
            len += 1;
        }
        if !self.created_at.is_empty() {
            len += 1;
        }
        if !self.updated_at.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.Conversation", len)?;
        if !self.pid.is_empty() {
            struct_ser.serialize_field("pid", &self.pid)?;
        }
        if let Some(v) = self.user_id.as_ref() {
            struct_ser.serialize_field("userId", v)?;
        }
        if !self.agent_pid.is_empty() {
            struct_ser.serialize_field("agentPid", &self.agent_pid)?;
        }
        if let Some(v) = self.title.as_ref() {
            struct_ser.serialize_field("title", v)?;
        }
        if !self.messages.is_empty() {
            struct_ser.serialize_field("messages", &self.messages)?;
        }
        if self.message_count != 0 {
            struct_ser.serialize_field("messageCount", &self.message_count)?;
        }
        if let Some(v) = self.last_message_at.as_ref() {
            struct_ser.serialize_field("lastMessageAt", v)?;
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
impl<'de> serde::Deserialize<'de> for Conversation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pid",
            "user_id",
            "userId",
            "agent_pid",
            "agentPid",
            "title",
            "messages",
            "message_count",
            "messageCount",
            "last_message_at",
            "lastMessageAt",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pid,
            UserId,
            AgentPid,
            Title,
            Messages,
            MessageCount,
            LastMessageAt,
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
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
                            "agentPid" | "agent_pid" => Ok(GeneratedField::AgentPid),
                            "title" => Ok(GeneratedField::Title),
                            "messages" => Ok(GeneratedField::Messages),
                            "messageCount" | "message_count" => Ok(GeneratedField::MessageCount),
                            "lastMessageAt" | "last_message_at" => Ok(GeneratedField::LastMessageAt),
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
            type Value = Conversation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.Conversation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Conversation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pid__ = None;
                let mut user_id__ = None;
                let mut agent_pid__ = None;
                let mut title__ = None;
                let mut messages__ = None;
                let mut message_count__ = None;
                let mut last_message_at__ = None;
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
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = map_.next_value()?;
                        }
                        GeneratedField::AgentPid => {
                            if agent_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("agentPid"));
                            }
                            agent_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = map_.next_value()?;
                        }
                        GeneratedField::Messages => {
                            if messages__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messages"));
                            }
                            messages__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MessageCount => {
                            if message_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messageCount"));
                            }
                            message_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::LastMessageAt => {
                            if last_message_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastMessageAt"));
                            }
                            last_message_at__ = map_.next_value()?;
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
                Ok(Conversation {
                    pid: pid__.unwrap_or_default(),
                    user_id: user_id__,
                    agent_pid: agent_pid__.unwrap_or_default(),
                    title: title__,
                    messages: messages__.unwrap_or_default(),
                    message_count: message_count__.unwrap_or_default(),
                    last_message_at: last_message_at__,
                    created_at: created_at__.unwrap_or_default(),
                    updated_at: updated_at__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.Conversation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ConversationShare {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user_id.is_empty() {
            len += 1;
        }
        if self.permissions != 0 {
            len += 1;
        }
        if !self.shared_by.is_empty() {
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
        let mut struct_ser = serializer.serialize_struct("rig.v1.ConversationShare", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if self.permissions != 0 {
            struct_ser.serialize_field("permissions", &self.permissions)?;
        }
        if !self.shared_by.is_empty() {
            struct_ser.serialize_field("sharedBy", &self.shared_by)?;
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
impl<'de> serde::Deserialize<'de> for ConversationShare {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
            "permissions",
            "shared_by",
            "sharedBy",
            "created_at",
            "createdAt",
            "display_name",
            "displayName",
            "email",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
            Permissions,
            SharedBy,
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
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
                            "permissions" => Ok(GeneratedField::Permissions),
                            "sharedBy" | "shared_by" => Ok(GeneratedField::SharedBy),
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
            type Value = ConversationShare;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.ConversationShare")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ConversationShare, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut permissions__ = None;
                let mut shared_by__ = None;
                let mut created_at__ = None;
                let mut display_name__ = None;
                let mut email__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Permissions => {
                            if permissions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("permissions"));
                            }
                            permissions__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::SharedBy => {
                            if shared_by__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sharedBy"));
                            }
                            shared_by__ = Some(map_.next_value()?);
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
                Ok(ConversationShare {
                    user_id: user_id__.unwrap_or_default(),
                    permissions: permissions__.unwrap_or_default(),
                    shared_by: shared_by__.unwrap_or_default(),
                    created_at: created_at__.unwrap_or_default(),
                    display_name: display_name__,
                    email: email__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.ConversationShare", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateAgentRequest {
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
        if !self.model_pid.is_empty() {
            len += 1;
        }
        if self.temperature != 0 {
            len += 1;
        }
        if !self.system_prompt.is_empty() {
            len += 1;
        }
        if self.config.is_some() {
            len += 1;
        }
        if self.provider_pid.is_some() {
            len += 1;
        }
        if !self.tool_pids.is_empty() {
            len += 1;
        }
        if self.kind != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.CreateAgentRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.model_pid.is_empty() {
            struct_ser.serialize_field("modelPid", &self.model_pid)?;
        }
        if self.temperature != 0 {
            struct_ser.serialize_field("temperature", &self.temperature)?;
        }
        if !self.system_prompt.is_empty() {
            struct_ser.serialize_field("systemPrompt", &self.system_prompt)?;
        }
        if let Some(v) = self.config.as_ref() {
            struct_ser.serialize_field("config", v)?;
        }
        if let Some(v) = self.provider_pid.as_ref() {
            struct_ser.serialize_field("providerPid", v)?;
        }
        if !self.tool_pids.is_empty() {
            struct_ser.serialize_field("toolPids", &self.tool_pids)?;
        }
        if self.kind != 0 {
            let v = AgentKind::try_from(self.kind)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.kind)))?;
            struct_ser.serialize_field("kind", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateAgentRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "model_pid",
            "modelPid",
            "temperature",
            "system_prompt",
            "systemPrompt",
            "config",
            "provider_pid",
            "providerPid",
            "tool_pids",
            "toolPids",
            "kind",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            ModelPid,
            Temperature,
            SystemPrompt,
            Config,
            ProviderPid,
            ToolPids,
            Kind,
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
                            "modelPid" | "model_pid" => Ok(GeneratedField::ModelPid),
                            "temperature" => Ok(GeneratedField::Temperature),
                            "systemPrompt" | "system_prompt" => Ok(GeneratedField::SystemPrompt),
                            "config" => Ok(GeneratedField::Config),
                            "providerPid" | "provider_pid" => Ok(GeneratedField::ProviderPid),
                            "toolPids" | "tool_pids" => Ok(GeneratedField::ToolPids),
                            "kind" => Ok(GeneratedField::Kind),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateAgentRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.CreateAgentRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateAgentRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut model_pid__ = None;
                let mut temperature__ = None;
                let mut system_prompt__ = None;
                let mut config__ = None;
                let mut provider_pid__ = None;
                let mut tool_pids__ = None;
                let mut kind__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ModelPid => {
                            if model_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("modelPid"));
                            }
                            model_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Temperature => {
                            if temperature__.is_some() {
                                return Err(serde::de::Error::duplicate_field("temperature"));
                            }
                            temperature__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::SystemPrompt => {
                            if system_prompt__.is_some() {
                                return Err(serde::de::Error::duplicate_field("systemPrompt"));
                            }
                            system_prompt__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Config => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config__ = map_.next_value()?;
                        }
                        GeneratedField::ProviderPid => {
                            if provider_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("providerPid"));
                            }
                            provider_pid__ = map_.next_value()?;
                        }
                        GeneratedField::ToolPids => {
                            if tool_pids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("toolPids"));
                            }
                            tool_pids__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Kind => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kind"));
                            }
                            kind__ = Some(map_.next_value::<AgentKind>()? as i32);
                        }
                    }
                }
                Ok(CreateAgentRequest {
                    name: name__.unwrap_or_default(),
                    model_pid: model_pid__.unwrap_or_default(),
                    temperature: temperature__.unwrap_or_default(),
                    system_prompt: system_prompt__.unwrap_or_default(),
                    config: config__,
                    provider_pid: provider_pid__,
                    tool_pids: tool_pids__.unwrap_or_default(),
                    kind: kind__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.CreateAgentRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateAgentResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.agent.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.CreateAgentResponse", len)?;
        if let Some(v) = self.agent.as_ref() {
            struct_ser.serialize_field("agent", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateAgentResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "agent",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Agent,
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
                            "agent" => Ok(GeneratedField::Agent),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateAgentResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.CreateAgentResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateAgentResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut agent__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Agent => {
                            if agent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("agent"));
                            }
                            agent__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateAgentResponse {
                    agent: agent__,
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.CreateAgentResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateConversationRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.agent_pid.is_empty() {
            len += 1;
        }
        if self.title.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.CreateConversationRequest", len)?;
        if !self.agent_pid.is_empty() {
            struct_ser.serialize_field("agentPid", &self.agent_pid)?;
        }
        if let Some(v) = self.title.as_ref() {
            struct_ser.serialize_field("title", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateConversationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "agent_pid",
            "agentPid",
            "title",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AgentPid,
            Title,
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
                            "agentPid" | "agent_pid" => Ok(GeneratedField::AgentPid),
                            "title" => Ok(GeneratedField::Title),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateConversationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.CreateConversationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateConversationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut agent_pid__ = None;
                let mut title__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AgentPid => {
                            if agent_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("agentPid"));
                            }
                            agent_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateConversationRequest {
                    agent_pid: agent_pid__.unwrap_or_default(),
                    title: title__,
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.CreateConversationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateConversationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.conversation.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.CreateConversationResponse", len)?;
        if let Some(v) = self.conversation.as_ref() {
            struct_ser.serialize_field("conversation", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateConversationResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "conversation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Conversation,
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
                            "conversation" => Ok(GeneratedField::Conversation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateConversationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.CreateConversationResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateConversationResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut conversation__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Conversation => {
                            if conversation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("conversation"));
                            }
                            conversation__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateConversationResponse {
                    conversation: conversation__,
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.CreateConversationResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateGroupRequest {
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
        if self.description.is_some() {
            len += 1;
        }
        if self.is_org_shared {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.CreateGroupRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        if self.is_org_shared {
            struct_ser.serialize_field("isOrgShared", &self.is_org_shared)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateGroupRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "description",
            "is_org_shared",
            "isOrgShared",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Description,
            IsOrgShared,
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
                            "description" => Ok(GeneratedField::Description),
                            "isOrgShared" | "is_org_shared" => Ok(GeneratedField::IsOrgShared),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateGroupRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.CreateGroupRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateGroupRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut description__ = None;
                let mut is_org_shared__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = map_.next_value()?;
                        }
                        GeneratedField::IsOrgShared => {
                            if is_org_shared__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isOrgShared"));
                            }
                            is_org_shared__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateGroupRequest {
                    name: name__.unwrap_or_default(),
                    description: description__,
                    is_org_shared: is_org_shared__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.CreateGroupRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateGroupResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.group.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.CreateGroupResponse", len)?;
        if let Some(v) = self.group.as_ref() {
            struct_ser.serialize_field("group", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateGroupResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "group",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Group,
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
                            "group" => Ok(GeneratedField::Group),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateGroupResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.CreateGroupResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateGroupResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut group__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Group => {
                            if group__.is_some() {
                                return Err(serde::de::Error::duplicate_field("group"));
                            }
                            group__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateGroupResponse {
                    group: group__,
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.CreateGroupResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateProviderRequest {
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
        if !self.provider_type.is_empty() {
            len += 1;
        }
        if self.source != 0 {
            len += 1;
        }
        if self.config.is_some() {
            len += 1;
        }
        if !self.credentials.is_empty() {
            len += 1;
        }
        if !self.endpoint_url.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.CreateProviderRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.provider_type.is_empty() {
            struct_ser.serialize_field("providerType", &self.provider_type)?;
        }
        if self.source != 0 {
            let v = ProviderSource::try_from(self.source)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.source)))?;
            struct_ser.serialize_field("source", &v)?;
        }
        if let Some(v) = self.config.as_ref() {
            struct_ser.serialize_field("config", v)?;
        }
        if !self.credentials.is_empty() {
            struct_ser.serialize_field("credentials", &self.credentials)?;
        }
        if !self.endpoint_url.is_empty() {
            struct_ser.serialize_field("endpointUrl", &self.endpoint_url)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateProviderRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "provider_type",
            "providerType",
            "source",
            "config",
            "credentials",
            "endpoint_url",
            "endpointUrl",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            ProviderType,
            Source,
            Config,
            Credentials,
            EndpointUrl,
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
                            "providerType" | "provider_type" => Ok(GeneratedField::ProviderType),
                            "source" => Ok(GeneratedField::Source),
                            "config" => Ok(GeneratedField::Config),
                            "credentials" => Ok(GeneratedField::Credentials),
                            "endpointUrl" | "endpoint_url" => Ok(GeneratedField::EndpointUrl),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateProviderRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.CreateProviderRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateProviderRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut provider_type__ = None;
                let mut source__ = None;
                let mut config__ = None;
                let mut credentials__ = None;
                let mut endpoint_url__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ProviderType => {
                            if provider_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("providerType"));
                            }
                            provider_type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Source => {
                            if source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            source__ = Some(map_.next_value::<ProviderSource>()? as i32);
                        }
                        GeneratedField::Config => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config__ = map_.next_value()?;
                        }
                        GeneratedField::Credentials => {
                            if credentials__.is_some() {
                                return Err(serde::de::Error::duplicate_field("credentials"));
                            }
                            credentials__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EndpointUrl => {
                            if endpoint_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endpointUrl"));
                            }
                            endpoint_url__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateProviderRequest {
                    name: name__.unwrap_or_default(),
                    provider_type: provider_type__.unwrap_or_default(),
                    source: source__.unwrap_or_default(),
                    config: config__,
                    credentials: credentials__.unwrap_or_default(),
                    endpoint_url: endpoint_url__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.CreateProviderRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateProviderResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.provider.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.CreateProviderResponse", len)?;
        if let Some(v) = self.provider.as_ref() {
            struct_ser.serialize_field("provider", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateProviderResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "provider",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Provider,
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
                            "provider" => Ok(GeneratedField::Provider),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateProviderResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.CreateProviderResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateProviderResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut provider__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Provider => {
                            if provider__.is_some() {
                                return Err(serde::de::Error::duplicate_field("provider"));
                            }
                            provider__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateProviderResponse {
                    provider: provider__,
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.CreateProviderResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteAgentRequest {
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
        let mut struct_ser = serializer.serialize_struct("rig.v1.DeleteAgentRequest", len)?;
        if !self.pid.is_empty() {
            struct_ser.serialize_field("pid", &self.pid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteAgentRequest {
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
            type Value = DeleteAgentRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.DeleteAgentRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteAgentRequest, V::Error>
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
                Ok(DeleteAgentRequest {
                    pid: pid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.DeleteAgentRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteAgentResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("rig.v1.DeleteAgentResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteAgentResponse {
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
            type Value = DeleteAgentResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.DeleteAgentResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteAgentResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(DeleteAgentResponse {
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.DeleteAgentResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteConversationRequest {
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
        let mut struct_ser = serializer.serialize_struct("rig.v1.DeleteConversationRequest", len)?;
        if !self.pid.is_empty() {
            struct_ser.serialize_field("pid", &self.pid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteConversationRequest {
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
            type Value = DeleteConversationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.DeleteConversationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteConversationRequest, V::Error>
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
                Ok(DeleteConversationRequest {
                    pid: pid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.DeleteConversationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteConversationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("rig.v1.DeleteConversationResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteConversationResponse {
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
            type Value = DeleteConversationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.DeleteConversationResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteConversationResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(DeleteConversationResponse {
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.DeleteConversationResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteDocumentRequest {
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
        let mut struct_ser = serializer.serialize_struct("rig.v1.DeleteDocumentRequest", len)?;
        if !self.pid.is_empty() {
            struct_ser.serialize_field("pid", &self.pid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteDocumentRequest {
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
            type Value = DeleteDocumentRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.DeleteDocumentRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteDocumentRequest, V::Error>
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
                Ok(DeleteDocumentRequest {
                    pid: pid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.DeleteDocumentRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteDocumentResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("rig.v1.DeleteDocumentResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteDocumentResponse {
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
            type Value = DeleteDocumentResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.DeleteDocumentResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteDocumentResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(DeleteDocumentResponse {
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.DeleteDocumentResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteGroupRequest {
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
        let mut struct_ser = serializer.serialize_struct("rig.v1.DeleteGroupRequest", len)?;
        if !self.pid.is_empty() {
            struct_ser.serialize_field("pid", &self.pid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteGroupRequest {
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
            type Value = DeleteGroupRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.DeleteGroupRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteGroupRequest, V::Error>
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
                Ok(DeleteGroupRequest {
                    pid: pid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.DeleteGroupRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteGroupResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("rig.v1.DeleteGroupResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteGroupResponse {
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
            type Value = DeleteGroupResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.DeleteGroupResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteGroupResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(DeleteGroupResponse {
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.DeleteGroupResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteProviderRequest {
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
        let mut struct_ser = serializer.serialize_struct("rig.v1.DeleteProviderRequest", len)?;
        if !self.pid.is_empty() {
            struct_ser.serialize_field("pid", &self.pid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteProviderRequest {
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
            type Value = DeleteProviderRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.DeleteProviderRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteProviderRequest, V::Error>
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
                Ok(DeleteProviderRequest {
                    pid: pid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.DeleteProviderRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteProviderResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("rig.v1.DeleteProviderResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteProviderResponse {
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
            type Value = DeleteProviderResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.DeleteProviderResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteProviderResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(DeleteProviderResponse {
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.DeleteProviderResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DetachDocumentsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.conversation_pid.is_empty() {
            len += 1;
        }
        if !self.document_pids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.DetachDocumentsRequest", len)?;
        if !self.conversation_pid.is_empty() {
            struct_ser.serialize_field("conversationPid", &self.conversation_pid)?;
        }
        if !self.document_pids.is_empty() {
            struct_ser.serialize_field("documentPids", &self.document_pids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DetachDocumentsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "conversation_pid",
            "conversationPid",
            "document_pids",
            "documentPids",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ConversationPid,
            DocumentPids,
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
                            "conversationPid" | "conversation_pid" => Ok(GeneratedField::ConversationPid),
                            "documentPids" | "document_pids" => Ok(GeneratedField::DocumentPids),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DetachDocumentsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.DetachDocumentsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DetachDocumentsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut conversation_pid__ = None;
                let mut document_pids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ConversationPid => {
                            if conversation_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("conversationPid"));
                            }
                            conversation_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DocumentPids => {
                            if document_pids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("documentPids"));
                            }
                            document_pids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DetachDocumentsRequest {
                    conversation_pid: conversation_pid__.unwrap_or_default(),
                    document_pids: document_pids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.DetachDocumentsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DetachDocumentsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.detached_count != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.DetachDocumentsResponse", len)?;
        if self.detached_count != 0 {
            struct_ser.serialize_field("detachedCount", &self.detached_count)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DetachDocumentsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "detached_count",
            "detachedCount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DetachedCount,
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
                            "detachedCount" | "detached_count" => Ok(GeneratedField::DetachedCount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DetachDocumentsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.DetachDocumentsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DetachDocumentsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut detached_count__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DetachedCount => {
                            if detached_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("detachedCount"));
                            }
                            detached_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(DetachDocumentsResponse {
                    detached_count: detached_count__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.DetachDocumentsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Document {
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
        if !self.filename.is_empty() {
            len += 1;
        }
        if !self.content_type.is_empty() {
            len += 1;
        }
        if self.size_bytes != 0 {
            len += 1;
        }
        if self.summary.is_some() {
            len += 1;
        }
        if self.status != 0 {
            len += 1;
        }
        if self.error_message.is_some() {
            len += 1;
        }
        if !self.created_at.is_empty() {
            len += 1;
        }
        if !self.updated_at.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.Document", len)?;
        if !self.pid.is_empty() {
            struct_ser.serialize_field("pid", &self.pid)?;
        }
        if !self.filename.is_empty() {
            struct_ser.serialize_field("filename", &self.filename)?;
        }
        if !self.content_type.is_empty() {
            struct_ser.serialize_field("contentType", &self.content_type)?;
        }
        if self.size_bytes != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("sizeBytes", ToString::to_string(&self.size_bytes).as_str())?;
        }
        if let Some(v) = self.summary.as_ref() {
            struct_ser.serialize_field("summary", v)?;
        }
        if self.status != 0 {
            let v = DocumentStatus::try_from(self.status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        if let Some(v) = self.error_message.as_ref() {
            struct_ser.serialize_field("errorMessage", v)?;
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
impl<'de> serde::Deserialize<'de> for Document {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pid",
            "filename",
            "content_type",
            "contentType",
            "size_bytes",
            "sizeBytes",
            "summary",
            "status",
            "error_message",
            "errorMessage",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pid,
            Filename,
            ContentType,
            SizeBytes,
            Summary,
            Status,
            ErrorMessage,
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
                            "filename" => Ok(GeneratedField::Filename),
                            "contentType" | "content_type" => Ok(GeneratedField::ContentType),
                            "sizeBytes" | "size_bytes" => Ok(GeneratedField::SizeBytes),
                            "summary" => Ok(GeneratedField::Summary),
                            "status" => Ok(GeneratedField::Status),
                            "errorMessage" | "error_message" => Ok(GeneratedField::ErrorMessage),
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
            type Value = Document;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.Document")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Document, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pid__ = None;
                let mut filename__ = None;
                let mut content_type__ = None;
                let mut size_bytes__ = None;
                let mut summary__ = None;
                let mut status__ = None;
                let mut error_message__ = None;
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
                        GeneratedField::Filename => {
                            if filename__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filename"));
                            }
                            filename__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ContentType => {
                            if content_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contentType"));
                            }
                            content_type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SizeBytes => {
                            if size_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sizeBytes"));
                            }
                            size_bytes__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Summary => {
                            if summary__.is_some() {
                                return Err(serde::de::Error::duplicate_field("summary"));
                            }
                            summary__ = map_.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value::<DocumentStatus>()? as i32);
                        }
                        GeneratedField::ErrorMessage => {
                            if error_message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorMessage"));
                            }
                            error_message__ = map_.next_value()?;
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
                Ok(Document {
                    pid: pid__.unwrap_or_default(),
                    filename: filename__.unwrap_or_default(),
                    content_type: content_type__.unwrap_or_default(),
                    size_bytes: size_bytes__.unwrap_or_default(),
                    summary: summary__,
                    status: status__.unwrap_or_default(),
                    error_message: error_message__,
                    created_at: created_at__.unwrap_or_default(),
                    updated_at: updated_at__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.Document", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DocumentChunk {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.document_pid.is_empty() {
            len += 1;
        }
        if !self.filename.is_empty() {
            len += 1;
        }
        if self.chunk_index != 0 {
            len += 1;
        }
        if !self.text.is_empty() {
            len += 1;
        }
        if self.score != 0. {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.DocumentChunk", len)?;
        if !self.document_pid.is_empty() {
            struct_ser.serialize_field("documentPid", &self.document_pid)?;
        }
        if !self.filename.is_empty() {
            struct_ser.serialize_field("filename", &self.filename)?;
        }
        if self.chunk_index != 0 {
            struct_ser.serialize_field("chunkIndex", &self.chunk_index)?;
        }
        if !self.text.is_empty() {
            struct_ser.serialize_field("text", &self.text)?;
        }
        if self.score != 0. {
            struct_ser.serialize_field("score", &self.score)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DocumentChunk {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "document_pid",
            "documentPid",
            "filename",
            "chunk_index",
            "chunkIndex",
            "text",
            "score",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DocumentPid,
            Filename,
            ChunkIndex,
            Text,
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
                            "documentPid" | "document_pid" => Ok(GeneratedField::DocumentPid),
                            "filename" => Ok(GeneratedField::Filename),
                            "chunkIndex" | "chunk_index" => Ok(GeneratedField::ChunkIndex),
                            "text" => Ok(GeneratedField::Text),
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
            type Value = DocumentChunk;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.DocumentChunk")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DocumentChunk, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut document_pid__ = None;
                let mut filename__ = None;
                let mut chunk_index__ = None;
                let mut text__ = None;
                let mut score__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DocumentPid => {
                            if document_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("documentPid"));
                            }
                            document_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Filename => {
                            if filename__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filename"));
                            }
                            filename__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ChunkIndex => {
                            if chunk_index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chunkIndex"));
                            }
                            chunk_index__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Text => {
                            if text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            text__ = Some(map_.next_value()?);
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
                Ok(DocumentChunk {
                    document_pid: document_pid__.unwrap_or_default(),
                    filename: filename__.unwrap_or_default(),
                    chunk_index: chunk_index__.unwrap_or_default(),
                    text: text__.unwrap_or_default(),
                    score: score__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.DocumentChunk", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DocumentGroup {
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
        if self.description.is_some() {
            len += 1;
        }
        if self.is_org_shared {
            len += 1;
        }
        if self.document_count != 0 {
            len += 1;
        }
        if !self.created_at.is_empty() {
            len += 1;
        }
        if !self.updated_at.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.DocumentGroup", len)?;
        if !self.pid.is_empty() {
            struct_ser.serialize_field("pid", &self.pid)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        if self.is_org_shared {
            struct_ser.serialize_field("isOrgShared", &self.is_org_shared)?;
        }
        if self.document_count != 0 {
            struct_ser.serialize_field("documentCount", &self.document_count)?;
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
impl<'de> serde::Deserialize<'de> for DocumentGroup {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pid",
            "name",
            "description",
            "is_org_shared",
            "isOrgShared",
            "document_count",
            "documentCount",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pid,
            Name,
            Description,
            IsOrgShared,
            DocumentCount,
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
                            "name" => Ok(GeneratedField::Name),
                            "description" => Ok(GeneratedField::Description),
                            "isOrgShared" | "is_org_shared" => Ok(GeneratedField::IsOrgShared),
                            "documentCount" | "document_count" => Ok(GeneratedField::DocumentCount),
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
            type Value = DocumentGroup;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.DocumentGroup")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DocumentGroup, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pid__ = None;
                let mut name__ = None;
                let mut description__ = None;
                let mut is_org_shared__ = None;
                let mut document_count__ = None;
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
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = map_.next_value()?;
                        }
                        GeneratedField::IsOrgShared => {
                            if is_org_shared__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isOrgShared"));
                            }
                            is_org_shared__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DocumentCount => {
                            if document_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("documentCount"));
                            }
                            document_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
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
                Ok(DocumentGroup {
                    pid: pid__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    description: description__,
                    is_org_shared: is_org_shared__.unwrap_or_default(),
                    document_count: document_count__.unwrap_or_default(),
                    created_at: created_at__.unwrap_or_default(),
                    updated_at: updated_at__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.DocumentGroup", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DocumentStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "DOCUMENT_STATUS_UNSPECIFIED",
            Self::Pending => "DOCUMENT_STATUS_PENDING",
            Self::Processing => "DOCUMENT_STATUS_PROCESSING",
            Self::Ready => "DOCUMENT_STATUS_READY",
            Self::Failed => "DOCUMENT_STATUS_FAILED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for DocumentStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "DOCUMENT_STATUS_UNSPECIFIED",
            "DOCUMENT_STATUS_PENDING",
            "DOCUMENT_STATUS_PROCESSING",
            "DOCUMENT_STATUS_READY",
            "DOCUMENT_STATUS_FAILED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DocumentStatus;

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
                    "DOCUMENT_STATUS_UNSPECIFIED" => Ok(DocumentStatus::Unspecified),
                    "DOCUMENT_STATUS_PENDING" => Ok(DocumentStatus::Pending),
                    "DOCUMENT_STATUS_PROCESSING" => Ok(DocumentStatus::Processing),
                    "DOCUMENT_STATUS_READY" => Ok(DocumentStatus::Ready),
                    "DOCUMENT_STATUS_FAILED" => Ok(DocumentStatus::Failed),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for GetAgentRequest {
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
        let mut struct_ser = serializer.serialize_struct("rig.v1.GetAgentRequest", len)?;
        if !self.pid.is_empty() {
            struct_ser.serialize_field("pid", &self.pid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetAgentRequest {
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
            type Value = GetAgentRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.GetAgentRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetAgentRequest, V::Error>
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
                Ok(GetAgentRequest {
                    pid: pid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.GetAgentRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetAgentResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.agent.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.GetAgentResponse", len)?;
        if let Some(v) = self.agent.as_ref() {
            struct_ser.serialize_field("agent", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetAgentResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "agent",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Agent,
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
                            "agent" => Ok(GeneratedField::Agent),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetAgentResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.GetAgentResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetAgentResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut agent__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Agent => {
                            if agent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("agent"));
                            }
                            agent__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetAgentResponse {
                    agent: agent__,
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.GetAgentResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetConversationRequest {
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
        let mut struct_ser = serializer.serialize_struct("rig.v1.GetConversationRequest", len)?;
        if !self.pid.is_empty() {
            struct_ser.serialize_field("pid", &self.pid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetConversationRequest {
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
            type Value = GetConversationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.GetConversationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetConversationRequest, V::Error>
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
                Ok(GetConversationRequest {
                    pid: pid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.GetConversationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetConversationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.conversation.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.GetConversationResponse", len)?;
        if let Some(v) = self.conversation.as_ref() {
            struct_ser.serialize_field("conversation", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetConversationResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "conversation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Conversation,
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
                            "conversation" => Ok(GeneratedField::Conversation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetConversationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.GetConversationResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetConversationResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut conversation__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Conversation => {
                            if conversation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("conversation"));
                            }
                            conversation__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetConversationResponse {
                    conversation: conversation__,
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.GetConversationResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetDocumentRequest {
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
        let mut struct_ser = serializer.serialize_struct("rig.v1.GetDocumentRequest", len)?;
        if !self.pid.is_empty() {
            struct_ser.serialize_field("pid", &self.pid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetDocumentRequest {
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
            type Value = GetDocumentRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.GetDocumentRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetDocumentRequest, V::Error>
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
                Ok(GetDocumentRequest {
                    pid: pid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.GetDocumentRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetDocumentResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.document.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.GetDocumentResponse", len)?;
        if let Some(v) = self.document.as_ref() {
            struct_ser.serialize_field("document", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetDocumentResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "document",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Document,
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
                            "document" => Ok(GeneratedField::Document),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetDocumentResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.GetDocumentResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetDocumentResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut document__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Document => {
                            if document__.is_some() {
                                return Err(serde::de::Error::duplicate_field("document"));
                            }
                            document__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetDocumentResponse {
                    document: document__,
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.GetDocumentResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetDownloadUrlRequest {
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
        let mut struct_ser = serializer.serialize_struct("rig.v1.GetDownloadUrlRequest", len)?;
        if !self.pid.is_empty() {
            struct_ser.serialize_field("pid", &self.pid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetDownloadUrlRequest {
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
            type Value = GetDownloadUrlRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.GetDownloadUrlRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetDownloadUrlRequest, V::Error>
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
                Ok(GetDownloadUrlRequest {
                    pid: pid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.GetDownloadUrlRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetDownloadUrlResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.download_url.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.GetDownloadUrlResponse", len)?;
        if !self.download_url.is_empty() {
            struct_ser.serialize_field("downloadUrl", &self.download_url)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetDownloadUrlResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "download_url",
            "downloadUrl",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DownloadUrl,
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
                            "downloadUrl" | "download_url" => Ok(GeneratedField::DownloadUrl),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetDownloadUrlResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.GetDownloadUrlResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetDownloadUrlResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut download_url__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DownloadUrl => {
                            if download_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("downloadUrl"));
                            }
                            download_url__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetDownloadUrlResponse {
                    download_url: download_url__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.GetDownloadUrlResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetGroupRequest {
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
        let mut struct_ser = serializer.serialize_struct("rig.v1.GetGroupRequest", len)?;
        if !self.pid.is_empty() {
            struct_ser.serialize_field("pid", &self.pid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetGroupRequest {
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
            type Value = GetGroupRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.GetGroupRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetGroupRequest, V::Error>
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
                Ok(GetGroupRequest {
                    pid: pid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.GetGroupRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetGroupResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.group.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.GetGroupResponse", len)?;
        if let Some(v) = self.group.as_ref() {
            struct_ser.serialize_field("group", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetGroupResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "group",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Group,
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
                            "group" => Ok(GeneratedField::Group),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetGroupResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.GetGroupResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetGroupResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut group__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Group => {
                            if group__.is_some() {
                                return Err(serde::de::Error::duplicate_field("group"));
                            }
                            group__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetGroupResponse {
                    group: group__,
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.GetGroupResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetModelRequest {
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
        let mut struct_ser = serializer.serialize_struct("rig.v1.GetModelRequest", len)?;
        if !self.pid.is_empty() {
            struct_ser.serialize_field("pid", &self.pid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetModelRequest {
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
            type Value = GetModelRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.GetModelRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetModelRequest, V::Error>
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
                Ok(GetModelRequest {
                    pid: pid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.GetModelRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetModelResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.model.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.GetModelResponse", len)?;
        if let Some(v) = self.model.as_ref() {
            struct_ser.serialize_field("model", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetModelResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "model",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Model,
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
                            "model" => Ok(GeneratedField::Model),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetModelResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.GetModelResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetModelResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut model__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Model => {
                            if model__.is_some() {
                                return Err(serde::de::Error::duplicate_field("model"));
                            }
                            model__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetModelResponse {
                    model: model__,
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.GetModelResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetProviderRequest {
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
        let mut struct_ser = serializer.serialize_struct("rig.v1.GetProviderRequest", len)?;
        if !self.pid.is_empty() {
            struct_ser.serialize_field("pid", &self.pid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetProviderRequest {
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
            type Value = GetProviderRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.GetProviderRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetProviderRequest, V::Error>
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
                Ok(GetProviderRequest {
                    pid: pid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.GetProviderRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetProviderResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.provider.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.GetProviderResponse", len)?;
        if let Some(v) = self.provider.as_ref() {
            struct_ser.serialize_field("provider", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetProviderResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "provider",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Provider,
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
                            "provider" => Ok(GeneratedField::Provider),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetProviderResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.GetProviderResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetProviderResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut provider__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Provider => {
                            if provider__.is_some() {
                                return Err(serde::de::Error::duplicate_field("provider"));
                            }
                            provider__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetProviderResponse {
                    provider: provider__,
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.GetProviderResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetToolRequest {
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
        let mut struct_ser = serializer.serialize_struct("rig.v1.GetToolRequest", len)?;
        if !self.pid.is_empty() {
            struct_ser.serialize_field("pid", &self.pid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetToolRequest {
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
            type Value = GetToolRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.GetToolRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetToolRequest, V::Error>
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
                Ok(GetToolRequest {
                    pid: pid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.GetToolRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetToolResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.tool.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.GetToolResponse", len)?;
        if let Some(v) = self.tool.as_ref() {
            struct_ser.serialize_field("tool", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetToolResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tool",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Tool,
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
                            "tool" => Ok(GeneratedField::Tool),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetToolResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.GetToolResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetToolResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tool__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Tool => {
                            if tool__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tool"));
                            }
                            tool__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetToolResponse {
                    tool: tool__,
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.GetToolResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetUserAssistantRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.GetUserAssistantRequest", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetUserAssistantRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = GetUserAssistantRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.GetUserAssistantRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetUserAssistantRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetUserAssistantRequest {
                    user_id: user_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.GetUserAssistantRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetUserAssistantResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.agent.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.GetUserAssistantResponse", len)?;
        if let Some(v) = self.agent.as_ref() {
            struct_ser.serialize_field("agent", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetUserAssistantResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "agent",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Agent,
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
                            "agent" => Ok(GeneratedField::Agent),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetUserAssistantResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.GetUserAssistantResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetUserAssistantResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut agent__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Agent => {
                            if agent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("agent"));
                            }
                            agent__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetUserAssistantResponse {
                    agent: agent__,
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.GetUserAssistantResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InferRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.agent_pid.is_empty() {
            len += 1;
        }
        if self.conversation_pid.is_some() {
            len += 1;
        }
        if !self.message.is_empty() {
            len += 1;
        }
        if !self.request_id.is_empty() {
            len += 1;
        }
        if !self.document_pids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.InferRequest", len)?;
        if !self.agent_pid.is_empty() {
            struct_ser.serialize_field("agentPid", &self.agent_pid)?;
        }
        if let Some(v) = self.conversation_pid.as_ref() {
            struct_ser.serialize_field("conversationPid", v)?;
        }
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", &self.message)?;
        }
        if !self.request_id.is_empty() {
            struct_ser.serialize_field("requestId", &self.request_id)?;
        }
        if !self.document_pids.is_empty() {
            struct_ser.serialize_field("documentPids", &self.document_pids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InferRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "agent_pid",
            "agentPid",
            "conversation_pid",
            "conversationPid",
            "message",
            "request_id",
            "requestId",
            "document_pids",
            "documentPids",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AgentPid,
            ConversationPid,
            Message,
            RequestId,
            DocumentPids,
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
                            "agentPid" | "agent_pid" => Ok(GeneratedField::AgentPid),
                            "conversationPid" | "conversation_pid" => Ok(GeneratedField::ConversationPid),
                            "message" => Ok(GeneratedField::Message),
                            "requestId" | "request_id" => Ok(GeneratedField::RequestId),
                            "documentPids" | "document_pids" => Ok(GeneratedField::DocumentPids),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InferRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.InferRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<InferRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut agent_pid__ = None;
                let mut conversation_pid__ = None;
                let mut message__ = None;
                let mut request_id__ = None;
                let mut document_pids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AgentPid => {
                            if agent_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("agentPid"));
                            }
                            agent_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ConversationPid => {
                            if conversation_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("conversationPid"));
                            }
                            conversation_pid__ = map_.next_value()?;
                        }
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RequestId => {
                            if request_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestId"));
                            }
                            request_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DocumentPids => {
                            if document_pids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("documentPids"));
                            }
                            document_pids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(InferRequest {
                    agent_pid: agent_pid__.unwrap_or_default(),
                    conversation_pid: conversation_pid__,
                    message: message__.unwrap_or_default(),
                    request_id: request_id__.unwrap_or_default(),
                    document_pids: document_pids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.InferRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InferResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.event.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.InferResponse", len)?;
        if let Some(v) = self.event.as_ref() {
            match v {
                infer_response::Event::PartStart(v) => {
                    struct_ser.serialize_field("partStart", v)?;
                }
                infer_response::Event::PartDelta(v) => {
                    struct_ser.serialize_field("partDelta", v)?;
                }
                infer_response::Event::PartEnd(v) => {
                    struct_ser.serialize_field("partEnd", v)?;
                }
                infer_response::Event::ToolStatus(v) => {
                    struct_ser.serialize_field("toolStatus", v)?;
                }
                infer_response::Event::Complete(v) => {
                    struct_ser.serialize_field("complete", v)?;
                }
                infer_response::Event::Cancelled(v) => {
                    struct_ser.serialize_field("cancelled", v)?;
                }
                infer_response::Event::Acknowledged(v) => {
                    struct_ser.serialize_field("acknowledged", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InferResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "part_start",
            "partStart",
            "part_delta",
            "partDelta",
            "part_end",
            "partEnd",
            "tool_status",
            "toolStatus",
            "complete",
            "cancelled",
            "acknowledged",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PartStart,
            PartDelta,
            PartEnd,
            ToolStatus,
            Complete,
            Cancelled,
            Acknowledged,
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
                            "partStart" | "part_start" => Ok(GeneratedField::PartStart),
                            "partDelta" | "part_delta" => Ok(GeneratedField::PartDelta),
                            "partEnd" | "part_end" => Ok(GeneratedField::PartEnd),
                            "toolStatus" | "tool_status" => Ok(GeneratedField::ToolStatus),
                            "complete" => Ok(GeneratedField::Complete),
                            "cancelled" => Ok(GeneratedField::Cancelled),
                            "acknowledged" => Ok(GeneratedField::Acknowledged),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InferResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.InferResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<InferResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut event__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PartStart => {
                            if event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("partStart"));
                            }
                            event__ = map_.next_value::<::std::option::Option<_>>()?.map(infer_response::Event::PartStart)
;
                        }
                        GeneratedField::PartDelta => {
                            if event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("partDelta"));
                            }
                            event__ = map_.next_value::<::std::option::Option<_>>()?.map(infer_response::Event::PartDelta)
;
                        }
                        GeneratedField::PartEnd => {
                            if event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("partEnd"));
                            }
                            event__ = map_.next_value::<::std::option::Option<_>>()?.map(infer_response::Event::PartEnd)
;
                        }
                        GeneratedField::ToolStatus => {
                            if event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("toolStatus"));
                            }
                            event__ = map_.next_value::<::std::option::Option<_>>()?.map(infer_response::Event::ToolStatus)
;
                        }
                        GeneratedField::Complete => {
                            if event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("complete"));
                            }
                            event__ = map_.next_value::<::std::option::Option<_>>()?.map(infer_response::Event::Complete)
;
                        }
                        GeneratedField::Cancelled => {
                            if event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cancelled"));
                            }
                            event__ = map_.next_value::<::std::option::Option<_>>()?.map(infer_response::Event::Cancelled)
;
                        }
                        GeneratedField::Acknowledged => {
                            if event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("acknowledged"));
                            }
                            event__ = map_.next_value::<::std::option::Option<_>>()?.map(infer_response::Event::Acknowledged)
;
                        }
                    }
                }
                Ok(InferResponse {
                    event: event__,
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.InferResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InferSyncResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.request_id.is_empty() {
            len += 1;
        }
        if !self.conversation_pid.is_empty() {
            len += 1;
        }
        if !self.message_pid.is_empty() {
            len += 1;
        }
        if self.message.is_some() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.InferSyncResponse", len)?;
        if !self.request_id.is_empty() {
            struct_ser.serialize_field("requestId", &self.request_id)?;
        }
        if !self.conversation_pid.is_empty() {
            struct_ser.serialize_field("conversationPid", &self.conversation_pid)?;
        }
        if !self.message_pid.is_empty() {
            struct_ser.serialize_field("messagePid", &self.message_pid)?;
        }
        if let Some(v) = self.message.as_ref() {
            struct_ser.serialize_field("message", v)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InferSyncResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "request_id",
            "requestId",
            "conversation_pid",
            "conversationPid",
            "message_pid",
            "messagePid",
            "message",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RequestId,
            ConversationPid,
            MessagePid,
            Message,
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
                            "requestId" | "request_id" => Ok(GeneratedField::RequestId),
                            "conversationPid" | "conversation_pid" => Ok(GeneratedField::ConversationPid),
                            "messagePid" | "message_pid" => Ok(GeneratedField::MessagePid),
                            "message" => Ok(GeneratedField::Message),
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
            type Value = InferSyncResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.InferSyncResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<InferSyncResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut request_id__ = None;
                let mut conversation_pid__ = None;
                let mut message_pid__ = None;
                let mut message__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RequestId => {
                            if request_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestId"));
                            }
                            request_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ConversationPid => {
                            if conversation_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("conversationPid"));
                            }
                            conversation_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MessagePid => {
                            if message_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messagePid"));
                            }
                            message_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = map_.next_value()?;
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                    }
                }
                Ok(InferSyncResponse {
                    request_id: request_id__.unwrap_or_default(),
                    conversation_pid: conversation_pid__.unwrap_or_default(),
                    message_pid: message_pid__.unwrap_or_default(),
                    message: message__,
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.InferSyncResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InferenceCancelled {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.reason.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.InferenceCancelled", len)?;
        if !self.reason.is_empty() {
            struct_ser.serialize_field("reason", &self.reason)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InferenceCancelled {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "reason",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Reason,
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
                            "reason" => Ok(GeneratedField::Reason),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InferenceCancelled;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.InferenceCancelled")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<InferenceCancelled, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut reason__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Reason => {
                            if reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            reason__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(InferenceCancelled {
                    reason: reason__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.InferenceCancelled", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InferenceComplete {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.conversation_pid.is_empty() {
            len += 1;
        }
        if self.message.is_some() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.InferenceComplete", len)?;
        if !self.conversation_pid.is_empty() {
            struct_ser.serialize_field("conversationPid", &self.conversation_pid)?;
        }
        if let Some(v) = self.message.as_ref() {
            struct_ser.serialize_field("message", v)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InferenceComplete {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "conversation_pid",
            "conversationPid",
            "message",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ConversationPid,
            Message,
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
                            "conversationPid" | "conversation_pid" => Ok(GeneratedField::ConversationPid),
                            "message" => Ok(GeneratedField::Message),
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
            type Value = InferenceComplete;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.InferenceComplete")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<InferenceComplete, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut conversation_pid__ = None;
                let mut message__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ConversationPid => {
                            if conversation_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("conversationPid"));
                            }
                            conversation_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = map_.next_value()?;
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                    }
                }
                Ok(InferenceComplete {
                    conversation_pid: conversation_pid__.unwrap_or_default(),
                    message: message__,
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.InferenceComplete", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListAgentSharesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.agent_pid.is_empty() {
            len += 1;
        }
        if self.page_size != 0 {
            len += 1;
        }
        if self.cursor.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.ListAgentSharesRequest", len)?;
        if !self.agent_pid.is_empty() {
            struct_ser.serialize_field("agentPid", &self.agent_pid)?;
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
impl<'de> serde::Deserialize<'de> for ListAgentSharesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "agent_pid",
            "agentPid",
            "page_size",
            "pageSize",
            "cursor",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AgentPid,
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
                            "agentPid" | "agent_pid" => Ok(GeneratedField::AgentPid),
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
            type Value = ListAgentSharesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.ListAgentSharesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListAgentSharesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut agent_pid__ = None;
                let mut page_size__ = None;
                let mut cursor__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AgentPid => {
                            if agent_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("agentPid"));
                            }
                            agent_pid__ = Some(map_.next_value()?);
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
                Ok(ListAgentSharesRequest {
                    agent_pid: agent_pid__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                    cursor: cursor__,
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.ListAgentSharesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListAgentSharesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.shares.is_empty() {
            len += 1;
        }
        if self.next_cursor.is_some() {
            len += 1;
        }
        if self.total != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.ListAgentSharesResponse", len)?;
        if !self.shares.is_empty() {
            struct_ser.serialize_field("shares", &self.shares)?;
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
impl<'de> serde::Deserialize<'de> for ListAgentSharesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "shares",
            "next_cursor",
            "nextCursor",
            "total",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Shares,
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
                            "shares" => Ok(GeneratedField::Shares),
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
            type Value = ListAgentSharesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.ListAgentSharesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListAgentSharesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut shares__ = None;
                let mut next_cursor__ = None;
                let mut total__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Shares => {
                            if shares__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shares"));
                            }
                            shares__ = Some(map_.next_value()?);
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
                Ok(ListAgentSharesResponse {
                    shares: shares__.unwrap_or_default(),
                    next_cursor: next_cursor__,
                    total: total__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.ListAgentSharesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListAgentsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.user_id.is_some() {
            len += 1;
        }
        if self.is_active.is_some() {
            len += 1;
        }
        if self.provider_pid.is_some() {
            len += 1;
        }
        if self.page != 0 {
            len += 1;
        }
        if self.page_size != 0 {
            len += 1;
        }
        if self.with_assistants.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.ListAgentsRequest", len)?;
        if let Some(v) = self.user_id.as_ref() {
            struct_ser.serialize_field("userId", v)?;
        }
        if let Some(v) = self.is_active.as_ref() {
            struct_ser.serialize_field("isActive", v)?;
        }
        if let Some(v) = self.provider_pid.as_ref() {
            struct_ser.serialize_field("providerPid", v)?;
        }
        if self.page != 0 {
            struct_ser.serialize_field("page", &self.page)?;
        }
        if self.page_size != 0 {
            struct_ser.serialize_field("pageSize", &self.page_size)?;
        }
        if let Some(v) = self.with_assistants.as_ref() {
            struct_ser.serialize_field("withAssistants", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListAgentsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
            "is_active",
            "isActive",
            "provider_pid",
            "providerPid",
            "page",
            "page_size",
            "pageSize",
            "with_assistants",
            "withAssistants",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
            IsActive,
            ProviderPid,
            Page,
            PageSize,
            WithAssistants,
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
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
                            "providerPid" | "provider_pid" => Ok(GeneratedField::ProviderPid),
                            "page" => Ok(GeneratedField::Page),
                            "pageSize" | "page_size" => Ok(GeneratedField::PageSize),
                            "withAssistants" | "with_assistants" => Ok(GeneratedField::WithAssistants),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListAgentsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.ListAgentsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListAgentsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut is_active__ = None;
                let mut provider_pid__ = None;
                let mut page__ = None;
                let mut page_size__ = None;
                let mut with_assistants__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = map_.next_value()?;
                        }
                        GeneratedField::IsActive => {
                            if is_active__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isActive"));
                            }
                            is_active__ = map_.next_value()?;
                        }
                        GeneratedField::ProviderPid => {
                            if provider_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("providerPid"));
                            }
                            provider_pid__ = map_.next_value()?;
                        }
                        GeneratedField::Page => {
                            if page__.is_some() {
                                return Err(serde::de::Error::duplicate_field("page"));
                            }
                            page__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PageSize => {
                            if page_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageSize"));
                            }
                            page_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::WithAssistants => {
                            if with_assistants__.is_some() {
                                return Err(serde::de::Error::duplicate_field("withAssistants"));
                            }
                            with_assistants__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListAgentsRequest {
                    user_id: user_id__,
                    is_active: is_active__,
                    provider_pid: provider_pid__,
                    page: page__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                    with_assistants: with_assistants__,
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.ListAgentsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListAgentsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.agents.is_empty() {
            len += 1;
        }
        if self.total != 0 {
            len += 1;
        }
        if self.page != 0 {
            len += 1;
        }
        if self.page_size != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.ListAgentsResponse", len)?;
        if !self.agents.is_empty() {
            struct_ser.serialize_field("agents", &self.agents)?;
        }
        if self.total != 0 {
            struct_ser.serialize_field("total", &self.total)?;
        }
        if self.page != 0 {
            struct_ser.serialize_field("page", &self.page)?;
        }
        if self.page_size != 0 {
            struct_ser.serialize_field("pageSize", &self.page_size)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListAgentsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "agents",
            "total",
            "page",
            "page_size",
            "pageSize",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Agents,
            Total,
            Page,
            PageSize,
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
                            "agents" => Ok(GeneratedField::Agents),
                            "total" => Ok(GeneratedField::Total),
                            "page" => Ok(GeneratedField::Page),
                            "pageSize" | "page_size" => Ok(GeneratedField::PageSize),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListAgentsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.ListAgentsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListAgentsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut agents__ = None;
                let mut total__ = None;
                let mut page__ = None;
                let mut page_size__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Agents => {
                            if agents__.is_some() {
                                return Err(serde::de::Error::duplicate_field("agents"));
                            }
                            agents__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Total => {
                            if total__.is_some() {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            total__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Page => {
                            if page__.is_some() {
                                return Err(serde::de::Error::duplicate_field("page"));
                            }
                            page__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PageSize => {
                            if page_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageSize"));
                            }
                            page_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ListAgentsResponse {
                    agents: agents__.unwrap_or_default(),
                    total: total__.unwrap_or_default(),
                    page: page__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.ListAgentsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListConversationDocumentsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.conversation_pid.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.ListConversationDocumentsRequest", len)?;
        if !self.conversation_pid.is_empty() {
            struct_ser.serialize_field("conversationPid", &self.conversation_pid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListConversationDocumentsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "conversation_pid",
            "conversationPid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ConversationPid,
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
                            "conversationPid" | "conversation_pid" => Ok(GeneratedField::ConversationPid),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListConversationDocumentsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.ListConversationDocumentsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListConversationDocumentsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut conversation_pid__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ConversationPid => {
                            if conversation_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("conversationPid"));
                            }
                            conversation_pid__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListConversationDocumentsRequest {
                    conversation_pid: conversation_pid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.ListConversationDocumentsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListConversationDocumentsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.documents.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.ListConversationDocumentsResponse", len)?;
        if !self.documents.is_empty() {
            struct_ser.serialize_field("documents", &self.documents)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListConversationDocumentsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "documents",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Documents,
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
                            "documents" => Ok(GeneratedField::Documents),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListConversationDocumentsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.ListConversationDocumentsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListConversationDocumentsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut documents__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Documents => {
                            if documents__.is_some() {
                                return Err(serde::de::Error::duplicate_field("documents"));
                            }
                            documents__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListConversationDocumentsResponse {
                    documents: documents__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.ListConversationDocumentsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListConversationSharesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.conversation_pid.is_empty() {
            len += 1;
        }
        if self.page_size != 0 {
            len += 1;
        }
        if self.cursor.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.ListConversationSharesRequest", len)?;
        if !self.conversation_pid.is_empty() {
            struct_ser.serialize_field("conversationPid", &self.conversation_pid)?;
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
impl<'de> serde::Deserialize<'de> for ListConversationSharesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "conversation_pid",
            "conversationPid",
            "page_size",
            "pageSize",
            "cursor",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ConversationPid,
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
                            "conversationPid" | "conversation_pid" => Ok(GeneratedField::ConversationPid),
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
            type Value = ListConversationSharesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.ListConversationSharesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListConversationSharesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut conversation_pid__ = None;
                let mut page_size__ = None;
                let mut cursor__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ConversationPid => {
                            if conversation_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("conversationPid"));
                            }
                            conversation_pid__ = Some(map_.next_value()?);
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
                Ok(ListConversationSharesRequest {
                    conversation_pid: conversation_pid__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                    cursor: cursor__,
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.ListConversationSharesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListConversationSharesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.shares.is_empty() {
            len += 1;
        }
        if self.next_cursor.is_some() {
            len += 1;
        }
        if self.total != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.ListConversationSharesResponse", len)?;
        if !self.shares.is_empty() {
            struct_ser.serialize_field("shares", &self.shares)?;
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
impl<'de> serde::Deserialize<'de> for ListConversationSharesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "shares",
            "next_cursor",
            "nextCursor",
            "total",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Shares,
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
                            "shares" => Ok(GeneratedField::Shares),
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
            type Value = ListConversationSharesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.ListConversationSharesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListConversationSharesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut shares__ = None;
                let mut next_cursor__ = None;
                let mut total__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Shares => {
                            if shares__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shares"));
                            }
                            shares__ = Some(map_.next_value()?);
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
                Ok(ListConversationSharesResponse {
                    shares: shares__.unwrap_or_default(),
                    next_cursor: next_cursor__,
                    total: total__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.ListConversationSharesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListConversationsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.is_active.is_some() {
            len += 1;
        }
        if self.agent_pid.is_some() {
            len += 1;
        }
        if self.page != 0 {
            len += 1;
        }
        if self.page_size != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.ListConversationsRequest", len)?;
        if let Some(v) = self.is_active.as_ref() {
            struct_ser.serialize_field("isActive", v)?;
        }
        if let Some(v) = self.agent_pid.as_ref() {
            struct_ser.serialize_field("agentPid", v)?;
        }
        if self.page != 0 {
            struct_ser.serialize_field("page", &self.page)?;
        }
        if self.page_size != 0 {
            struct_ser.serialize_field("pageSize", &self.page_size)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListConversationsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "is_active",
            "isActive",
            "agent_pid",
            "agentPid",
            "page",
            "page_size",
            "pageSize",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IsActive,
            AgentPid,
            Page,
            PageSize,
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
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
                            "agentPid" | "agent_pid" => Ok(GeneratedField::AgentPid),
                            "page" => Ok(GeneratedField::Page),
                            "pageSize" | "page_size" => Ok(GeneratedField::PageSize),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListConversationsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.ListConversationsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListConversationsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut is_active__ = None;
                let mut agent_pid__ = None;
                let mut page__ = None;
                let mut page_size__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::IsActive => {
                            if is_active__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isActive"));
                            }
                            is_active__ = map_.next_value()?;
                        }
                        GeneratedField::AgentPid => {
                            if agent_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("agentPid"));
                            }
                            agent_pid__ = map_.next_value()?;
                        }
                        GeneratedField::Page => {
                            if page__.is_some() {
                                return Err(serde::de::Error::duplicate_field("page"));
                            }
                            page__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PageSize => {
                            if page_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageSize"));
                            }
                            page_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ListConversationsRequest {
                    is_active: is_active__,
                    agent_pid: agent_pid__,
                    page: page__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.ListConversationsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListConversationsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.conversations.is_empty() {
            len += 1;
        }
        if self.total != 0 {
            len += 1;
        }
        if self.page != 0 {
            len += 1;
        }
        if self.page_size != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.ListConversationsResponse", len)?;
        if !self.conversations.is_empty() {
            struct_ser.serialize_field("conversations", &self.conversations)?;
        }
        if self.total != 0 {
            struct_ser.serialize_field("total", &self.total)?;
        }
        if self.page != 0 {
            struct_ser.serialize_field("page", &self.page)?;
        }
        if self.page_size != 0 {
            struct_ser.serialize_field("pageSize", &self.page_size)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListConversationsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "conversations",
            "total",
            "page",
            "page_size",
            "pageSize",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Conversations,
            Total,
            Page,
            PageSize,
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
                            "conversations" => Ok(GeneratedField::Conversations),
                            "total" => Ok(GeneratedField::Total),
                            "page" => Ok(GeneratedField::Page),
                            "pageSize" | "page_size" => Ok(GeneratedField::PageSize),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListConversationsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.ListConversationsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListConversationsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut conversations__ = None;
                let mut total__ = None;
                let mut page__ = None;
                let mut page_size__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Conversations => {
                            if conversations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("conversations"));
                            }
                            conversations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Total => {
                            if total__.is_some() {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            total__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Page => {
                            if page__.is_some() {
                                return Err(serde::de::Error::duplicate_field("page"));
                            }
                            page__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PageSize => {
                            if page_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageSize"));
                            }
                            page_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ListConversationsResponse {
                    conversations: conversations__.unwrap_or_default(),
                    total: total__.unwrap_or_default(),
                    page: page__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.ListConversationsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListDocumentsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.group_pid.is_some() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        if self.page != 0 {
            len += 1;
        }
        if self.page_size != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.ListDocumentsRequest", len)?;
        if let Some(v) = self.group_pid.as_ref() {
            struct_ser.serialize_field("groupPid", v)?;
        }
        if let Some(v) = self.status.as_ref() {
            let v = DocumentStatus::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        if self.page != 0 {
            struct_ser.serialize_field("page", &self.page)?;
        }
        if self.page_size != 0 {
            struct_ser.serialize_field("pageSize", &self.page_size)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListDocumentsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "group_pid",
            "groupPid",
            "status",
            "page",
            "page_size",
            "pageSize",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            GroupPid,
            Status,
            Page,
            PageSize,
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
                            "groupPid" | "group_pid" => Ok(GeneratedField::GroupPid),
                            "status" => Ok(GeneratedField::Status),
                            "page" => Ok(GeneratedField::Page),
                            "pageSize" | "page_size" => Ok(GeneratedField::PageSize),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListDocumentsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.ListDocumentsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListDocumentsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut group_pid__ = None;
                let mut status__ = None;
                let mut page__ = None;
                let mut page_size__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::GroupPid => {
                            if group_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("groupPid"));
                            }
                            group_pid__ = map_.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map_.next_value::<::std::option::Option<DocumentStatus>>()?.map(|x| x as i32);
                        }
                        GeneratedField::Page => {
                            if page__.is_some() {
                                return Err(serde::de::Error::duplicate_field("page"));
                            }
                            page__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PageSize => {
                            if page_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageSize"));
                            }
                            page_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ListDocumentsRequest {
                    group_pid: group_pid__,
                    status: status__,
                    page: page__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.ListDocumentsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListDocumentsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.documents.is_empty() {
            len += 1;
        }
        if self.total != 0 {
            len += 1;
        }
        if self.page != 0 {
            len += 1;
        }
        if self.page_size != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.ListDocumentsResponse", len)?;
        if !self.documents.is_empty() {
            struct_ser.serialize_field("documents", &self.documents)?;
        }
        if self.total != 0 {
            struct_ser.serialize_field("total", &self.total)?;
        }
        if self.page != 0 {
            struct_ser.serialize_field("page", &self.page)?;
        }
        if self.page_size != 0 {
            struct_ser.serialize_field("pageSize", &self.page_size)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListDocumentsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "documents",
            "total",
            "page",
            "page_size",
            "pageSize",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Documents,
            Total,
            Page,
            PageSize,
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
                            "documents" => Ok(GeneratedField::Documents),
                            "total" => Ok(GeneratedField::Total),
                            "page" => Ok(GeneratedField::Page),
                            "pageSize" | "page_size" => Ok(GeneratedField::PageSize),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListDocumentsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.ListDocumentsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListDocumentsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut documents__ = None;
                let mut total__ = None;
                let mut page__ = None;
                let mut page_size__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Documents => {
                            if documents__.is_some() {
                                return Err(serde::de::Error::duplicate_field("documents"));
                            }
                            documents__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Total => {
                            if total__.is_some() {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            total__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Page => {
                            if page__.is_some() {
                                return Err(serde::de::Error::duplicate_field("page"));
                            }
                            page__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PageSize => {
                            if page_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageSize"));
                            }
                            page_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ListDocumentsResponse {
                    documents: documents__.unwrap_or_default(),
                    total: total__.unwrap_or_default(),
                    page: page__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.ListDocumentsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListGroupsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.page != 0 {
            len += 1;
        }
        if self.page_size != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.ListGroupsRequest", len)?;
        if self.page != 0 {
            struct_ser.serialize_field("page", &self.page)?;
        }
        if self.page_size != 0 {
            struct_ser.serialize_field("pageSize", &self.page_size)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListGroupsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "page",
            "page_size",
            "pageSize",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Page,
            PageSize,
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
                            "page" => Ok(GeneratedField::Page),
                            "pageSize" | "page_size" => Ok(GeneratedField::PageSize),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListGroupsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.ListGroupsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListGroupsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut page__ = None;
                let mut page_size__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Page => {
                            if page__.is_some() {
                                return Err(serde::de::Error::duplicate_field("page"));
                            }
                            page__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PageSize => {
                            if page_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageSize"));
                            }
                            page_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ListGroupsRequest {
                    page: page__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.ListGroupsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListGroupsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.groups.is_empty() {
            len += 1;
        }
        if self.total != 0 {
            len += 1;
        }
        if self.page != 0 {
            len += 1;
        }
        if self.page_size != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.ListGroupsResponse", len)?;
        if !self.groups.is_empty() {
            struct_ser.serialize_field("groups", &self.groups)?;
        }
        if self.total != 0 {
            struct_ser.serialize_field("total", &self.total)?;
        }
        if self.page != 0 {
            struct_ser.serialize_field("page", &self.page)?;
        }
        if self.page_size != 0 {
            struct_ser.serialize_field("pageSize", &self.page_size)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListGroupsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "groups",
            "total",
            "page",
            "page_size",
            "pageSize",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Groups,
            Total,
            Page,
            PageSize,
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
                            "groups" => Ok(GeneratedField::Groups),
                            "total" => Ok(GeneratedField::Total),
                            "page" => Ok(GeneratedField::Page),
                            "pageSize" | "page_size" => Ok(GeneratedField::PageSize),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListGroupsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.ListGroupsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListGroupsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut groups__ = None;
                let mut total__ = None;
                let mut page__ = None;
                let mut page_size__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Groups => {
                            if groups__.is_some() {
                                return Err(serde::de::Error::duplicate_field("groups"));
                            }
                            groups__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Total => {
                            if total__.is_some() {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            total__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Page => {
                            if page__.is_some() {
                                return Err(serde::de::Error::duplicate_field("page"));
                            }
                            page__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PageSize => {
                            if page_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageSize"));
                            }
                            page_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ListGroupsResponse {
                    groups: groups__.unwrap_or_default(),
                    total: total__.unwrap_or_default(),
                    page: page__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.ListGroupsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListModelsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.provider_pid.is_some() {
            len += 1;
        }
        if self.provider_type.is_some() {
            len += 1;
        }
        if self.is_active.is_some() {
            len += 1;
        }
        if self.include_deprecated.is_some() {
            len += 1;
        }
        if self.page != 0 {
            len += 1;
        }
        if self.page_size != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.ListModelsRequest", len)?;
        if let Some(v) = self.provider_pid.as_ref() {
            struct_ser.serialize_field("providerPid", v)?;
        }
        if let Some(v) = self.provider_type.as_ref() {
            struct_ser.serialize_field("providerType", v)?;
        }
        if let Some(v) = self.is_active.as_ref() {
            struct_ser.serialize_field("isActive", v)?;
        }
        if let Some(v) = self.include_deprecated.as_ref() {
            struct_ser.serialize_field("includeDeprecated", v)?;
        }
        if self.page != 0 {
            struct_ser.serialize_field("page", &self.page)?;
        }
        if self.page_size != 0 {
            struct_ser.serialize_field("pageSize", &self.page_size)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListModelsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "provider_pid",
            "providerPid",
            "provider_type",
            "providerType",
            "is_active",
            "isActive",
            "include_deprecated",
            "includeDeprecated",
            "page",
            "page_size",
            "pageSize",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProviderPid,
            ProviderType,
            IsActive,
            IncludeDeprecated,
            Page,
            PageSize,
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
                            "providerPid" | "provider_pid" => Ok(GeneratedField::ProviderPid),
                            "providerType" | "provider_type" => Ok(GeneratedField::ProviderType),
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
                            "includeDeprecated" | "include_deprecated" => Ok(GeneratedField::IncludeDeprecated),
                            "page" => Ok(GeneratedField::Page),
                            "pageSize" | "page_size" => Ok(GeneratedField::PageSize),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListModelsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.ListModelsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListModelsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut provider_pid__ = None;
                let mut provider_type__ = None;
                let mut is_active__ = None;
                let mut include_deprecated__ = None;
                let mut page__ = None;
                let mut page_size__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProviderPid => {
                            if provider_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("providerPid"));
                            }
                            provider_pid__ = map_.next_value()?;
                        }
                        GeneratedField::ProviderType => {
                            if provider_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("providerType"));
                            }
                            provider_type__ = map_.next_value()?;
                        }
                        GeneratedField::IsActive => {
                            if is_active__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isActive"));
                            }
                            is_active__ = map_.next_value()?;
                        }
                        GeneratedField::IncludeDeprecated => {
                            if include_deprecated__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includeDeprecated"));
                            }
                            include_deprecated__ = map_.next_value()?;
                        }
                        GeneratedField::Page => {
                            if page__.is_some() {
                                return Err(serde::de::Error::duplicate_field("page"));
                            }
                            page__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PageSize => {
                            if page_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageSize"));
                            }
                            page_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ListModelsRequest {
                    provider_pid: provider_pid__,
                    provider_type: provider_type__,
                    is_active: is_active__,
                    include_deprecated: include_deprecated__,
                    page: page__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.ListModelsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListModelsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.models.is_empty() {
            len += 1;
        }
        if self.total != 0 {
            len += 1;
        }
        if self.page != 0 {
            len += 1;
        }
        if self.page_size != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.ListModelsResponse", len)?;
        if !self.models.is_empty() {
            struct_ser.serialize_field("models", &self.models)?;
        }
        if self.total != 0 {
            struct_ser.serialize_field("total", &self.total)?;
        }
        if self.page != 0 {
            struct_ser.serialize_field("page", &self.page)?;
        }
        if self.page_size != 0 {
            struct_ser.serialize_field("pageSize", &self.page_size)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListModelsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "models",
            "total",
            "page",
            "page_size",
            "pageSize",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Models,
            Total,
            Page,
            PageSize,
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
                            "models" => Ok(GeneratedField::Models),
                            "total" => Ok(GeneratedField::Total),
                            "page" => Ok(GeneratedField::Page),
                            "pageSize" | "page_size" => Ok(GeneratedField::PageSize),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListModelsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.ListModelsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListModelsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut models__ = None;
                let mut total__ = None;
                let mut page__ = None;
                let mut page_size__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Models => {
                            if models__.is_some() {
                                return Err(serde::de::Error::duplicate_field("models"));
                            }
                            models__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Total => {
                            if total__.is_some() {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            total__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Page => {
                            if page__.is_some() {
                                return Err(serde::de::Error::duplicate_field("page"));
                            }
                            page__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PageSize => {
                            if page_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageSize"));
                            }
                            page_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ListModelsResponse {
                    models: models__.unwrap_or_default(),
                    total: total__.unwrap_or_default(),
                    page: page__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.ListModelsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListProvidersRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.is_active.is_some() {
            len += 1;
        }
        if self.source.is_some() {
            len += 1;
        }
        if self.provider_type.is_some() {
            len += 1;
        }
        if self.page != 0 {
            len += 1;
        }
        if self.page_size != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.ListProvidersRequest", len)?;
        if let Some(v) = self.is_active.as_ref() {
            struct_ser.serialize_field("isActive", v)?;
        }
        if let Some(v) = self.source.as_ref() {
            let v = ProviderSource::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("source", &v)?;
        }
        if let Some(v) = self.provider_type.as_ref() {
            struct_ser.serialize_field("providerType", v)?;
        }
        if self.page != 0 {
            struct_ser.serialize_field("page", &self.page)?;
        }
        if self.page_size != 0 {
            struct_ser.serialize_field("pageSize", &self.page_size)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListProvidersRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "is_active",
            "isActive",
            "source",
            "provider_type",
            "providerType",
            "page",
            "page_size",
            "pageSize",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IsActive,
            Source,
            ProviderType,
            Page,
            PageSize,
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
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
                            "source" => Ok(GeneratedField::Source),
                            "providerType" | "provider_type" => Ok(GeneratedField::ProviderType),
                            "page" => Ok(GeneratedField::Page),
                            "pageSize" | "page_size" => Ok(GeneratedField::PageSize),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListProvidersRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.ListProvidersRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListProvidersRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut is_active__ = None;
                let mut source__ = None;
                let mut provider_type__ = None;
                let mut page__ = None;
                let mut page_size__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::IsActive => {
                            if is_active__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isActive"));
                            }
                            is_active__ = map_.next_value()?;
                        }
                        GeneratedField::Source => {
                            if source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            source__ = map_.next_value::<::std::option::Option<ProviderSource>>()?.map(|x| x as i32);
                        }
                        GeneratedField::ProviderType => {
                            if provider_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("providerType"));
                            }
                            provider_type__ = map_.next_value()?;
                        }
                        GeneratedField::Page => {
                            if page__.is_some() {
                                return Err(serde::de::Error::duplicate_field("page"));
                            }
                            page__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PageSize => {
                            if page_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageSize"));
                            }
                            page_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ListProvidersRequest {
                    is_active: is_active__,
                    source: source__,
                    provider_type: provider_type__,
                    page: page__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.ListProvidersRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListProvidersResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.providers.is_empty() {
            len += 1;
        }
        if self.total != 0 {
            len += 1;
        }
        if self.page != 0 {
            len += 1;
        }
        if self.page_size != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.ListProvidersResponse", len)?;
        if !self.providers.is_empty() {
            struct_ser.serialize_field("providers", &self.providers)?;
        }
        if self.total != 0 {
            struct_ser.serialize_field("total", &self.total)?;
        }
        if self.page != 0 {
            struct_ser.serialize_field("page", &self.page)?;
        }
        if self.page_size != 0 {
            struct_ser.serialize_field("pageSize", &self.page_size)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListProvidersResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "providers",
            "total",
            "page",
            "page_size",
            "pageSize",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Providers,
            Total,
            Page,
            PageSize,
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
                            "providers" => Ok(GeneratedField::Providers),
                            "total" => Ok(GeneratedField::Total),
                            "page" => Ok(GeneratedField::Page),
                            "pageSize" | "page_size" => Ok(GeneratedField::PageSize),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListProvidersResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.ListProvidersResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListProvidersResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut providers__ = None;
                let mut total__ = None;
                let mut page__ = None;
                let mut page_size__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Providers => {
                            if providers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("providers"));
                            }
                            providers__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Total => {
                            if total__.is_some() {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            total__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Page => {
                            if page__.is_some() {
                                return Err(serde::de::Error::duplicate_field("page"));
                            }
                            page__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PageSize => {
                            if page_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageSize"));
                            }
                            page_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ListProvidersResponse {
                    providers: providers__.unwrap_or_default(),
                    total: total__.unwrap_or_default(),
                    page: page__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.ListProvidersResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListToolsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.is_active.is_some() {
            len += 1;
        }
        if self.tool_type.is_some() {
            len += 1;
        }
        if self.page != 0 {
            len += 1;
        }
        if self.page_size != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.ListToolsRequest", len)?;
        if let Some(v) = self.is_active.as_ref() {
            struct_ser.serialize_field("isActive", v)?;
        }
        if let Some(v) = self.tool_type.as_ref() {
            struct_ser.serialize_field("toolType", v)?;
        }
        if self.page != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("page", ToString::to_string(&self.page).as_str())?;
        }
        if self.page_size != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("pageSize", ToString::to_string(&self.page_size).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListToolsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "is_active",
            "isActive",
            "tool_type",
            "toolType",
            "page",
            "page_size",
            "pageSize",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IsActive,
            ToolType,
            Page,
            PageSize,
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
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
                            "toolType" | "tool_type" => Ok(GeneratedField::ToolType),
                            "page" => Ok(GeneratedField::Page),
                            "pageSize" | "page_size" => Ok(GeneratedField::PageSize),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListToolsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.ListToolsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListToolsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut is_active__ = None;
                let mut tool_type__ = None;
                let mut page__ = None;
                let mut page_size__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::IsActive => {
                            if is_active__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isActive"));
                            }
                            is_active__ = map_.next_value()?;
                        }
                        GeneratedField::ToolType => {
                            if tool_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("toolType"));
                            }
                            tool_type__ = map_.next_value()?;
                        }
                        GeneratedField::Page => {
                            if page__.is_some() {
                                return Err(serde::de::Error::duplicate_field("page"));
                            }
                            page__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PageSize => {
                            if page_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageSize"));
                            }
                            page_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ListToolsRequest {
                    is_active: is_active__,
                    tool_type: tool_type__,
                    page: page__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.ListToolsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListToolsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tools.is_empty() {
            len += 1;
        }
        if self.total != 0 {
            len += 1;
        }
        if self.page != 0 {
            len += 1;
        }
        if self.page_size != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.ListToolsResponse", len)?;
        if !self.tools.is_empty() {
            struct_ser.serialize_field("tools", &self.tools)?;
        }
        if self.total != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("total", ToString::to_string(&self.total).as_str())?;
        }
        if self.page != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("page", ToString::to_string(&self.page).as_str())?;
        }
        if self.page_size != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("pageSize", ToString::to_string(&self.page_size).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListToolsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tools",
            "total",
            "page",
            "page_size",
            "pageSize",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Tools,
            Total,
            Page,
            PageSize,
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
                            "tools" => Ok(GeneratedField::Tools),
                            "total" => Ok(GeneratedField::Total),
                            "page" => Ok(GeneratedField::Page),
                            "pageSize" | "page_size" => Ok(GeneratedField::PageSize),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListToolsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.ListToolsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListToolsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tools__ = None;
                let mut total__ = None;
                let mut page__ = None;
                let mut page_size__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Tools => {
                            if tools__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tools"));
                            }
                            tools__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Total => {
                            if total__.is_some() {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            total__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Page => {
                            if page__.is_some() {
                                return Err(serde::de::Error::duplicate_field("page"));
                            }
                            page__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PageSize => {
                            if page_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageSize"));
                            }
                            page_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ListToolsResponse {
                    tools: tools__.unwrap_or_default(),
                    total: total__.unwrap_or_default(),
                    page: page__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.ListToolsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MediaContent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.media_type.is_some() {
            len += 1;
        }
        if self.source.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.MediaContent", len)?;
        if let Some(v) = self.media_type.as_ref() {
            struct_ser.serialize_field("mediaType", v)?;
        }
        if let Some(v) = self.source.as_ref() {
            match v {
                media_content::Source::Url(v) => {
                    struct_ser.serialize_field("url", v)?;
                }
                media_content::Source::Data(v) => {
                    #[allow(clippy::needless_borrow)]
                    #[allow(clippy::needless_borrows_for_generic_args)]
                    struct_ser.serialize_field("data", pbjson::private::base64::encode(&v).as_str())?;
                }
                media_content::Source::Base64(v) => {
                    struct_ser.serialize_field("base64", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MediaContent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "media_type",
            "mediaType",
            "url",
            "data",
            "base64",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MediaType,
            Url,
            Data,
            Base64,
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
                            "mediaType" | "media_type" => Ok(GeneratedField::MediaType),
                            "url" => Ok(GeneratedField::Url),
                            "data" => Ok(GeneratedField::Data),
                            "base64" => Ok(GeneratedField::Base64),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MediaContent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.MediaContent")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MediaContent, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut media_type__ = None;
                let mut source__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MediaType => {
                            if media_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mediaType"));
                            }
                            media_type__ = map_.next_value()?;
                        }
                        GeneratedField::Url => {
                            if source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("url"));
                            }
                            source__ = map_.next_value::<::std::option::Option<_>>()?.map(media_content::Source::Url);
                        }
                        GeneratedField::Data => {
                            if source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            source__ = map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| media_content::Source::Data(x.0));
                        }
                        GeneratedField::Base64 => {
                            if source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("base64"));
                            }
                            source__ = map_.next_value::<::std::option::Option<_>>()?.map(media_content::Source::Base64);
                        }
                    }
                }
                Ok(MediaContent {
                    media_type: media_type__,
                    source: source__,
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.MediaContent", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Message {
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
        if !self.role.is_empty() {
            len += 1;
        }
        if self.status != 0 {
            len += 1;
        }
        if !self.parts.is_empty() {
            len += 1;
        }
        if !self.created_at.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.Message", len)?;
        if !self.pid.is_empty() {
            struct_ser.serialize_field("pid", &self.pid)?;
        }
        if !self.role.is_empty() {
            struct_ser.serialize_field("role", &self.role)?;
        }
        if self.status != 0 {
            let v = MessageStatus::try_from(self.status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        if !self.parts.is_empty() {
            struct_ser.serialize_field("parts", &self.parts)?;
        }
        if !self.created_at.is_empty() {
            struct_ser.serialize_field("createdAt", &self.created_at)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Message {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pid",
            "role",
            "status",
            "parts",
            "created_at",
            "createdAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pid,
            Role,
            Status,
            Parts,
            CreatedAt,
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
                            "role" => Ok(GeneratedField::Role),
                            "status" => Ok(GeneratedField::Status),
                            "parts" => Ok(GeneratedField::Parts),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Message;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.Message")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Message, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pid__ = None;
                let mut role__ = None;
                let mut status__ = None;
                let mut parts__ = None;
                let mut created_at__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Pid => {
                            if pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pid"));
                            }
                            pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Role => {
                            if role__.is_some() {
                                return Err(serde::de::Error::duplicate_field("role"));
                            }
                            role__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value::<MessageStatus>()? as i32);
                        }
                        GeneratedField::Parts => {
                            if parts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parts"));
                            }
                            parts__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdAt"));
                            }
                            created_at__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Message {
                    pid: pid__.unwrap_or_default(),
                    role: role__.unwrap_or_default(),
                    status: status__.unwrap_or_default(),
                    parts: parts__.unwrap_or_default(),
                    created_at: created_at__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.Message", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MessageAcknowledged {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.conversation_pid.is_empty() {
            len += 1;
        }
        if !self.message_pid.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.MessageAcknowledged", len)?;
        if !self.conversation_pid.is_empty() {
            struct_ser.serialize_field("conversationPid", &self.conversation_pid)?;
        }
        if !self.message_pid.is_empty() {
            struct_ser.serialize_field("messagePid", &self.message_pid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MessageAcknowledged {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "conversation_pid",
            "conversationPid",
            "message_pid",
            "messagePid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ConversationPid,
            MessagePid,
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
                            "conversationPid" | "conversation_pid" => Ok(GeneratedField::ConversationPid),
                            "messagePid" | "message_pid" => Ok(GeneratedField::MessagePid),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MessageAcknowledged;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.MessageAcknowledged")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MessageAcknowledged, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut conversation_pid__ = None;
                let mut message_pid__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ConversationPid => {
                            if conversation_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("conversationPid"));
                            }
                            conversation_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MessagePid => {
                            if message_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messagePid"));
                            }
                            message_pid__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MessageAcknowledged {
                    conversation_pid: conversation_pid__.unwrap_or_default(),
                    message_pid: message_pid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.MessageAcknowledged", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MessagePart {
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
        if self.kind != 0 {
            len += 1;
        }
        if self.status != 0 {
            len += 1;
        }
        if self.content.is_some() {
            len += 1;
        }
        if self.tool_call.is_some() {
            len += 1;
        }
        if self.tool_result.is_some() {
            len += 1;
        }
        if self.media.is_some() {
            len += 1;
        }
        if self.summary.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.MessagePart", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if self.kind != 0 {
            let v = MessagePartKind::try_from(self.kind)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.kind)))?;
            struct_ser.serialize_field("kind", &v)?;
        }
        if self.status != 0 {
            let v = MessagePartStatus::try_from(self.status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        if let Some(v) = self.content.as_ref() {
            struct_ser.serialize_field("content", v)?;
        }
        if let Some(v) = self.tool_call.as_ref() {
            struct_ser.serialize_field("toolCall", v)?;
        }
        if let Some(v) = self.tool_result.as_ref() {
            struct_ser.serialize_field("toolResult", v)?;
        }
        if let Some(v) = self.media.as_ref() {
            struct_ser.serialize_field("media", v)?;
        }
        if let Some(v) = self.summary.as_ref() {
            struct_ser.serialize_field("summary", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MessagePart {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "kind",
            "status",
            "content",
            "tool_call",
            "toolCall",
            "tool_result",
            "toolResult",
            "media",
            "summary",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Kind,
            Status,
            Content,
            ToolCall,
            ToolResult,
            Media,
            Summary,
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
                            "kind" => Ok(GeneratedField::Kind),
                            "status" => Ok(GeneratedField::Status),
                            "content" => Ok(GeneratedField::Content),
                            "toolCall" | "tool_call" => Ok(GeneratedField::ToolCall),
                            "toolResult" | "tool_result" => Ok(GeneratedField::ToolResult),
                            "media" => Ok(GeneratedField::Media),
                            "summary" => Ok(GeneratedField::Summary),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MessagePart;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.MessagePart")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MessagePart, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut kind__ = None;
                let mut status__ = None;
                let mut content__ = None;
                let mut tool_call__ = None;
                let mut tool_result__ = None;
                let mut media__ = None;
                let mut summary__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Kind => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kind"));
                            }
                            kind__ = Some(map_.next_value::<MessagePartKind>()? as i32);
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value::<MessagePartStatus>()? as i32);
                        }
                        GeneratedField::Content => {
                            if content__.is_some() {
                                return Err(serde::de::Error::duplicate_field("content"));
                            }
                            content__ = map_.next_value()?;
                        }
                        GeneratedField::ToolCall => {
                            if tool_call__.is_some() {
                                return Err(serde::de::Error::duplicate_field("toolCall"));
                            }
                            tool_call__ = map_.next_value()?;
                        }
                        GeneratedField::ToolResult => {
                            if tool_result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("toolResult"));
                            }
                            tool_result__ = map_.next_value()?;
                        }
                        GeneratedField::Media => {
                            if media__.is_some() {
                                return Err(serde::de::Error::duplicate_field("media"));
                            }
                            media__ = map_.next_value()?;
                        }
                        GeneratedField::Summary => {
                            if summary__.is_some() {
                                return Err(serde::de::Error::duplicate_field("summary"));
                            }
                            summary__ = map_.next_value()?;
                        }
                    }
                }
                Ok(MessagePart {
                    id: id__.unwrap_or_default(),
                    kind: kind__.unwrap_or_default(),
                    status: status__.unwrap_or_default(),
                    content: content__,
                    tool_call: tool_call__,
                    tool_result: tool_result__,
                    media: media__,
                    summary: summary__,
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.MessagePart", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MessagePartKind {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "MESSAGE_PART_KIND_UNSPECIFIED",
            Self::Text => "MESSAGE_PART_KIND_TEXT",
            Self::Thinking => "MESSAGE_PART_KIND_THINKING",
            Self::ToolCall => "MESSAGE_PART_KIND_TOOL_CALL",
            Self::ToolResult => "MESSAGE_PART_KIND_TOOL_RESULT",
            Self::Image => "MESSAGE_PART_KIND_IMAGE",
            Self::Audio => "MESSAGE_PART_KIND_AUDIO",
            Self::Video => "MESSAGE_PART_KIND_VIDEO",
            Self::Document => "MESSAGE_PART_KIND_DOCUMENT",
            Self::Metadata => "MESSAGE_PART_KIND_METADATA",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for MessagePartKind {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "MESSAGE_PART_KIND_UNSPECIFIED",
            "MESSAGE_PART_KIND_TEXT",
            "MESSAGE_PART_KIND_THINKING",
            "MESSAGE_PART_KIND_TOOL_CALL",
            "MESSAGE_PART_KIND_TOOL_RESULT",
            "MESSAGE_PART_KIND_IMAGE",
            "MESSAGE_PART_KIND_AUDIO",
            "MESSAGE_PART_KIND_VIDEO",
            "MESSAGE_PART_KIND_DOCUMENT",
            "MESSAGE_PART_KIND_METADATA",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MessagePartKind;

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
                    "MESSAGE_PART_KIND_UNSPECIFIED" => Ok(MessagePartKind::Unspecified),
                    "MESSAGE_PART_KIND_TEXT" => Ok(MessagePartKind::Text),
                    "MESSAGE_PART_KIND_THINKING" => Ok(MessagePartKind::Thinking),
                    "MESSAGE_PART_KIND_TOOL_CALL" => Ok(MessagePartKind::ToolCall),
                    "MESSAGE_PART_KIND_TOOL_RESULT" => Ok(MessagePartKind::ToolResult),
                    "MESSAGE_PART_KIND_IMAGE" => Ok(MessagePartKind::Image),
                    "MESSAGE_PART_KIND_AUDIO" => Ok(MessagePartKind::Audio),
                    "MESSAGE_PART_KIND_VIDEO" => Ok(MessagePartKind::Video),
                    "MESSAGE_PART_KIND_DOCUMENT" => Ok(MessagePartKind::Document),
                    "MESSAGE_PART_KIND_METADATA" => Ok(MessagePartKind::Metadata),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for MessagePartStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "MESSAGE_PART_STATUS_UNSPECIFIED",
            Self::Pending => "MESSAGE_PART_STATUS_PENDING",
            Self::Streaming => "MESSAGE_PART_STATUS_STREAMING",
            Self::Complete => "MESSAGE_PART_STATUS_COMPLETE",
            Self::Failed => "MESSAGE_PART_STATUS_FAILED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for MessagePartStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "MESSAGE_PART_STATUS_UNSPECIFIED",
            "MESSAGE_PART_STATUS_PENDING",
            "MESSAGE_PART_STATUS_STREAMING",
            "MESSAGE_PART_STATUS_COMPLETE",
            "MESSAGE_PART_STATUS_FAILED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MessagePartStatus;

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
                    "MESSAGE_PART_STATUS_UNSPECIFIED" => Ok(MessagePartStatus::Unspecified),
                    "MESSAGE_PART_STATUS_PENDING" => Ok(MessagePartStatus::Pending),
                    "MESSAGE_PART_STATUS_STREAMING" => Ok(MessagePartStatus::Streaming),
                    "MESSAGE_PART_STATUS_COMPLETE" => Ok(MessagePartStatus::Complete),
                    "MESSAGE_PART_STATUS_FAILED" => Ok(MessagePartStatus::Failed),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for MessageStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "MESSAGE_STATUS_UNSPECIFIED",
            Self::Complete => "MESSAGE_STATUS_COMPLETE",
            Self::Cancelled => "MESSAGE_STATUS_CANCELLED",
            Self::Error => "MESSAGE_STATUS_ERROR",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for MessageStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "MESSAGE_STATUS_UNSPECIFIED",
            "MESSAGE_STATUS_COMPLETE",
            "MESSAGE_STATUS_CANCELLED",
            "MESSAGE_STATUS_ERROR",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MessageStatus;

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
                    "MESSAGE_STATUS_UNSPECIFIED" => Ok(MessageStatus::Unspecified),
                    "MESSAGE_STATUS_COMPLETE" => Ok(MessageStatus::Complete),
                    "MESSAGE_STATUS_CANCELLED" => Ok(MessageStatus::Cancelled),
                    "MESSAGE_STATUS_ERROR" => Ok(MessageStatus::Error),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Model {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.model_id.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.provider_pid.is_empty() {
            len += 1;
        }
        if !self.provider_type.is_empty() {
            len += 1;
        }
        if !self.provider_name.is_empty() {
            len += 1;
        }
        if self.context_length.is_some() {
            len += 1;
        }
        if self.capabilities.is_some() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        if self.is_active {
            len += 1;
        }
        if self.deprecation_message.is_some() {
            len += 1;
        }
        if self.last_fetched_at.is_some() {
            len += 1;
        }
        if !self.created_at.is_empty() {
            len += 1;
        }
        if !self.updated_at.is_empty() {
            len += 1;
        }
        if self.identifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.Model", len)?;
        if !self.model_id.is_empty() {
            struct_ser.serialize_field("modelId", &self.model_id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.provider_pid.is_empty() {
            struct_ser.serialize_field("providerPid", &self.provider_pid)?;
        }
        if !self.provider_type.is_empty() {
            struct_ser.serialize_field("providerType", &self.provider_type)?;
        }
        if !self.provider_name.is_empty() {
            struct_ser.serialize_field("providerName", &self.provider_name)?;
        }
        if let Some(v) = self.context_length.as_ref() {
            struct_ser.serialize_field("contextLength", v)?;
        }
        if let Some(v) = self.capabilities.as_ref() {
            struct_ser.serialize_field("capabilities", v)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if self.is_active {
            struct_ser.serialize_field("isActive", &self.is_active)?;
        }
        if let Some(v) = self.deprecation_message.as_ref() {
            struct_ser.serialize_field("deprecationMessage", v)?;
        }
        if let Some(v) = self.last_fetched_at.as_ref() {
            struct_ser.serialize_field("lastFetchedAt", v)?;
        }
        if !self.created_at.is_empty() {
            struct_ser.serialize_field("createdAt", &self.created_at)?;
        }
        if !self.updated_at.is_empty() {
            struct_ser.serialize_field("updatedAt", &self.updated_at)?;
        }
        if let Some(v) = self.identifier.as_ref() {
            match v {
                model::Identifier::Id(v) => {
                    struct_ser.serialize_field("id", v)?;
                }
                model::Identifier::Pid(v) => {
                    struct_ser.serialize_field("pid", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Model {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "model_id",
            "modelId",
            "name",
            "provider_pid",
            "providerPid",
            "provider_type",
            "providerType",
            "provider_name",
            "providerName",
            "context_length",
            "contextLength",
            "capabilities",
            "metadata",
            "is_active",
            "isActive",
            "deprecation_message",
            "deprecationMessage",
            "last_fetched_at",
            "lastFetchedAt",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
            "id",
            "pid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ModelId,
            Name,
            ProviderPid,
            ProviderType,
            ProviderName,
            ContextLength,
            Capabilities,
            Metadata,
            IsActive,
            DeprecationMessage,
            LastFetchedAt,
            CreatedAt,
            UpdatedAt,
            Id,
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
                            "modelId" | "model_id" => Ok(GeneratedField::ModelId),
                            "name" => Ok(GeneratedField::Name),
                            "providerPid" | "provider_pid" => Ok(GeneratedField::ProviderPid),
                            "providerType" | "provider_type" => Ok(GeneratedField::ProviderType),
                            "providerName" | "provider_name" => Ok(GeneratedField::ProviderName),
                            "contextLength" | "context_length" => Ok(GeneratedField::ContextLength),
                            "capabilities" => Ok(GeneratedField::Capabilities),
                            "metadata" => Ok(GeneratedField::Metadata),
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
                            "deprecationMessage" | "deprecation_message" => Ok(GeneratedField::DeprecationMessage),
                            "lastFetchedAt" | "last_fetched_at" => Ok(GeneratedField::LastFetchedAt),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            "id" => Ok(GeneratedField::Id),
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
            type Value = Model;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.Model")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Model, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut model_id__ = None;
                let mut name__ = None;
                let mut provider_pid__ = None;
                let mut provider_type__ = None;
                let mut provider_name__ = None;
                let mut context_length__ = None;
                let mut capabilities__ = None;
                let mut metadata__ = None;
                let mut is_active__ = None;
                let mut deprecation_message__ = None;
                let mut last_fetched_at__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                let mut identifier__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ModelId => {
                            if model_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("modelId"));
                            }
                            model_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ProviderPid => {
                            if provider_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("providerPid"));
                            }
                            provider_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ProviderType => {
                            if provider_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("providerType"));
                            }
                            provider_type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ProviderName => {
                            if provider_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("providerName"));
                            }
                            provider_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ContextLength => {
                            if context_length__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contextLength"));
                            }
                            context_length__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Capabilities => {
                            if capabilities__.is_some() {
                                return Err(serde::de::Error::duplicate_field("capabilities"));
                            }
                            capabilities__ = map_.next_value()?;
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                        GeneratedField::IsActive => {
                            if is_active__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isActive"));
                            }
                            is_active__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DeprecationMessage => {
                            if deprecation_message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deprecationMessage"));
                            }
                            deprecation_message__ = map_.next_value()?;
                        }
                        GeneratedField::LastFetchedAt => {
                            if last_fetched_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastFetchedAt"));
                            }
                            last_fetched_at__ = map_.next_value()?;
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
                        GeneratedField::Id => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            identifier__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| model::Identifier::Id(x.0));
                        }
                        GeneratedField::Pid => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pid"));
                            }
                            identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(model::Identifier::Pid);
                        }
                    }
                }
                Ok(Model {
                    model_id: model_id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    provider_pid: provider_pid__.unwrap_or_default(),
                    provider_type: provider_type__.unwrap_or_default(),
                    provider_name: provider_name__.unwrap_or_default(),
                    context_length: context_length__,
                    capabilities: capabilities__,
                    metadata: metadata__,
                    is_active: is_active__.unwrap_or_default(),
                    deprecation_message: deprecation_message__,
                    last_fetched_at: last_fetched_at__,
                    created_at: created_at__.unwrap_or_default(),
                    updated_at: updated_at__.unwrap_or_default(),
                    identifier: identifier__,
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.Model", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ModelCapabilities {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.vision {
            len += 1;
        }
        if self.function_calling {
            len += 1;
        }
        if self.json_mode {
            len += 1;
        }
        if self.streaming {
            len += 1;
        }
        if self.system_prompt {
            len += 1;
        }
        if self.multi_turn {
            len += 1;
        }
        if self.thinking {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.ModelCapabilities", len)?;
        if self.vision {
            struct_ser.serialize_field("vision", &self.vision)?;
        }
        if self.function_calling {
            struct_ser.serialize_field("functionCalling", &self.function_calling)?;
        }
        if self.json_mode {
            struct_ser.serialize_field("jsonMode", &self.json_mode)?;
        }
        if self.streaming {
            struct_ser.serialize_field("streaming", &self.streaming)?;
        }
        if self.system_prompt {
            struct_ser.serialize_field("systemPrompt", &self.system_prompt)?;
        }
        if self.multi_turn {
            struct_ser.serialize_field("multiTurn", &self.multi_turn)?;
        }
        if self.thinking {
            struct_ser.serialize_field("thinking", &self.thinking)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ModelCapabilities {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "vision",
            "function_calling",
            "functionCalling",
            "json_mode",
            "jsonMode",
            "streaming",
            "system_prompt",
            "systemPrompt",
            "multi_turn",
            "multiTurn",
            "thinking",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Vision,
            FunctionCalling,
            JsonMode,
            Streaming,
            SystemPrompt,
            MultiTurn,
            Thinking,
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
                            "vision" => Ok(GeneratedField::Vision),
                            "functionCalling" | "function_calling" => Ok(GeneratedField::FunctionCalling),
                            "jsonMode" | "json_mode" => Ok(GeneratedField::JsonMode),
                            "streaming" => Ok(GeneratedField::Streaming),
                            "systemPrompt" | "system_prompt" => Ok(GeneratedField::SystemPrompt),
                            "multiTurn" | "multi_turn" => Ok(GeneratedField::MultiTurn),
                            "thinking" => Ok(GeneratedField::Thinking),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ModelCapabilities;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.ModelCapabilities")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ModelCapabilities, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut vision__ = None;
                let mut function_calling__ = None;
                let mut json_mode__ = None;
                let mut streaming__ = None;
                let mut system_prompt__ = None;
                let mut multi_turn__ = None;
                let mut thinking__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Vision => {
                            if vision__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vision"));
                            }
                            vision__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FunctionCalling => {
                            if function_calling__.is_some() {
                                return Err(serde::de::Error::duplicate_field("functionCalling"));
                            }
                            function_calling__ = Some(map_.next_value()?);
                        }
                        GeneratedField::JsonMode => {
                            if json_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jsonMode"));
                            }
                            json_mode__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Streaming => {
                            if streaming__.is_some() {
                                return Err(serde::de::Error::duplicate_field("streaming"));
                            }
                            streaming__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SystemPrompt => {
                            if system_prompt__.is_some() {
                                return Err(serde::de::Error::duplicate_field("systemPrompt"));
                            }
                            system_prompt__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MultiTurn => {
                            if multi_turn__.is_some() {
                                return Err(serde::de::Error::duplicate_field("multiTurn"));
                            }
                            multi_turn__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Thinking => {
                            if thinking__.is_some() {
                                return Err(serde::de::Error::duplicate_field("thinking"));
                            }
                            thinking__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ModelCapabilities {
                    vision: vision__.unwrap_or_default(),
                    function_calling: function_calling__.unwrap_or_default(),
                    json_mode: json_mode__.unwrap_or_default(),
                    streaming: streaming__.unwrap_or_default(),
                    system_prompt: system_prompt__.unwrap_or_default(),
                    multi_turn: multi_turn__.unwrap_or_default(),
                    thinking: thinking__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.ModelCapabilities", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ModelMetadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.owned_by.is_some() {
            len += 1;
        }
        if self.family.is_some() {
            len += 1;
        }
        if self.input_cost_per_mtok.is_some() {
            len += 1;
        }
        if self.output_cost_per_mtok.is_some() {
            len += 1;
        }
        if self.max_output_tokens.is_some() {
            len += 1;
        }
        if self.quantization.is_some() {
            len += 1;
        }
        if self.size_bytes.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.ModelMetadata", len)?;
        if let Some(v) = self.owned_by.as_ref() {
            struct_ser.serialize_field("ownedBy", v)?;
        }
        if let Some(v) = self.family.as_ref() {
            struct_ser.serialize_field("family", v)?;
        }
        if let Some(v) = self.input_cost_per_mtok.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("inputCostPerMtok", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.output_cost_per_mtok.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("outputCostPerMtok", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.max_output_tokens.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("maxOutputTokens", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.quantization.as_ref() {
            struct_ser.serialize_field("quantization", v)?;
        }
        if let Some(v) = self.size_bytes.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("sizeBytes", ToString::to_string(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ModelMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "owned_by",
            "ownedBy",
            "family",
            "input_cost_per_mtok",
            "inputCostPerMtok",
            "output_cost_per_mtok",
            "outputCostPerMtok",
            "max_output_tokens",
            "maxOutputTokens",
            "quantization",
            "size_bytes",
            "sizeBytes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OwnedBy,
            Family,
            InputCostPerMtok,
            OutputCostPerMtok,
            MaxOutputTokens,
            Quantization,
            SizeBytes,
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
                            "ownedBy" | "owned_by" => Ok(GeneratedField::OwnedBy),
                            "family" => Ok(GeneratedField::Family),
                            "inputCostPerMtok" | "input_cost_per_mtok" => Ok(GeneratedField::InputCostPerMtok),
                            "outputCostPerMtok" | "output_cost_per_mtok" => Ok(GeneratedField::OutputCostPerMtok),
                            "maxOutputTokens" | "max_output_tokens" => Ok(GeneratedField::MaxOutputTokens),
                            "quantization" => Ok(GeneratedField::Quantization),
                            "sizeBytes" | "size_bytes" => Ok(GeneratedField::SizeBytes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ModelMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.ModelMetadata")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ModelMetadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut owned_by__ = None;
                let mut family__ = None;
                let mut input_cost_per_mtok__ = None;
                let mut output_cost_per_mtok__ = None;
                let mut max_output_tokens__ = None;
                let mut quantization__ = None;
                let mut size_bytes__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OwnedBy => {
                            if owned_by__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ownedBy"));
                            }
                            owned_by__ = map_.next_value()?;
                        }
                        GeneratedField::Family => {
                            if family__.is_some() {
                                return Err(serde::de::Error::duplicate_field("family"));
                            }
                            family__ = map_.next_value()?;
                        }
                        GeneratedField::InputCostPerMtok => {
                            if input_cost_per_mtok__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inputCostPerMtok"));
                            }
                            input_cost_per_mtok__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::OutputCostPerMtok => {
                            if output_cost_per_mtok__.is_some() {
                                return Err(serde::de::Error::duplicate_field("outputCostPerMtok"));
                            }
                            output_cost_per_mtok__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::MaxOutputTokens => {
                            if max_output_tokens__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxOutputTokens"));
                            }
                            max_output_tokens__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Quantization => {
                            if quantization__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantization"));
                            }
                            quantization__ = map_.next_value()?;
                        }
                        GeneratedField::SizeBytes => {
                            if size_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sizeBytes"));
                            }
                            size_bytes__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(ModelMetadata {
                    owned_by: owned_by__,
                    family: family__,
                    input_cost_per_mtok: input_cost_per_mtok__,
                    output_cost_per_mtok: output_cost_per_mtok__,
                    max_output_tokens: max_output_tokens__,
                    quantization: quantization__,
                    size_bytes: size_bytes__,
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.ModelMetadata", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PartDelta {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.part_id.is_empty() {
            len += 1;
        }
        if self.kind != 0 {
            len += 1;
        }
        if self.delta.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.PartDelta", len)?;
        if !self.part_id.is_empty() {
            struct_ser.serialize_field("partId", &self.part_id)?;
        }
        if self.kind != 0 {
            let v = MessagePartKind::try_from(self.kind)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.kind)))?;
            struct_ser.serialize_field("kind", &v)?;
        }
        if let Some(v) = self.delta.as_ref() {
            match v {
                part_delta::Delta::Content(v) => {
                    struct_ser.serialize_field("content", v)?;
                }
                part_delta::Delta::Arguments(v) => {
                    struct_ser.serialize_field("arguments", v)?;
                }
                part_delta::Delta::Media(v) => {
                    struct_ser.serialize_field("media", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PartDelta {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "part_id",
            "partId",
            "kind",
            "content",
            "arguments",
            "media",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PartId,
            Kind,
            Content,
            Arguments,
            Media,
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
                            "partId" | "part_id" => Ok(GeneratedField::PartId),
                            "kind" => Ok(GeneratedField::Kind),
                            "content" => Ok(GeneratedField::Content),
                            "arguments" => Ok(GeneratedField::Arguments),
                            "media" => Ok(GeneratedField::Media),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PartDelta;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.PartDelta")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PartDelta, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut part_id__ = None;
                let mut kind__ = None;
                let mut delta__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PartId => {
                            if part_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("partId"));
                            }
                            part_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Kind => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kind"));
                            }
                            kind__ = Some(map_.next_value::<MessagePartKind>()? as i32);
                        }
                        GeneratedField::Content => {
                            if delta__.is_some() {
                                return Err(serde::de::Error::duplicate_field("content"));
                            }
                            delta__ = map_.next_value::<::std::option::Option<_>>()?.map(part_delta::Delta::Content);
                        }
                        GeneratedField::Arguments => {
                            if delta__.is_some() {
                                return Err(serde::de::Error::duplicate_field("arguments"));
                            }
                            delta__ = map_.next_value::<::std::option::Option<_>>()?.map(part_delta::Delta::Arguments)
;
                        }
                        GeneratedField::Media => {
                            if delta__.is_some() {
                                return Err(serde::de::Error::duplicate_field("media"));
                            }
                            delta__ = map_.next_value::<::std::option::Option<_>>()?.map(part_delta::Delta::Media)
;
                        }
                    }
                }
                Ok(PartDelta {
                    part_id: part_id__.unwrap_or_default(),
                    kind: kind__.unwrap_or_default(),
                    delta: delta__,
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.PartDelta", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PartEnd {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.part_id.is_empty() {
            len += 1;
        }
        if self.status != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.PartEnd", len)?;
        if !self.part_id.is_empty() {
            struct_ser.serialize_field("partId", &self.part_id)?;
        }
        if self.status != 0 {
            let v = MessagePartStatus::try_from(self.status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PartEnd {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "part_id",
            "partId",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PartId,
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
                            "partId" | "part_id" => Ok(GeneratedField::PartId),
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
            type Value = PartEnd;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.PartEnd")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PartEnd, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut part_id__ = None;
                let mut status__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PartId => {
                            if part_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("partId"));
                            }
                            part_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value::<MessagePartStatus>()? as i32);
                        }
                    }
                }
                Ok(PartEnd {
                    part_id: part_id__.unwrap_or_default(),
                    status: status__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.PartEnd", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PartStart {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.part_id.is_empty() {
            len += 1;
        }
        if self.kind != 0 {
            len += 1;
        }
        if self.tool_name.is_some() {
            len += 1;
        }
        if self.tool_call_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.PartStart", len)?;
        if !self.part_id.is_empty() {
            struct_ser.serialize_field("partId", &self.part_id)?;
        }
        if self.kind != 0 {
            let v = MessagePartKind::try_from(self.kind)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.kind)))?;
            struct_ser.serialize_field("kind", &v)?;
        }
        if let Some(v) = self.tool_name.as_ref() {
            struct_ser.serialize_field("toolName", v)?;
        }
        if let Some(v) = self.tool_call_id.as_ref() {
            struct_ser.serialize_field("toolCallId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PartStart {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "part_id",
            "partId",
            "kind",
            "tool_name",
            "toolName",
            "tool_call_id",
            "toolCallId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PartId,
            Kind,
            ToolName,
            ToolCallId,
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
                            "partId" | "part_id" => Ok(GeneratedField::PartId),
                            "kind" => Ok(GeneratedField::Kind),
                            "toolName" | "tool_name" => Ok(GeneratedField::ToolName),
                            "toolCallId" | "tool_call_id" => Ok(GeneratedField::ToolCallId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PartStart;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.PartStart")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PartStart, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut part_id__ = None;
                let mut kind__ = None;
                let mut tool_name__ = None;
                let mut tool_call_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PartId => {
                            if part_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("partId"));
                            }
                            part_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Kind => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kind"));
                            }
                            kind__ = Some(map_.next_value::<MessagePartKind>()? as i32);
                        }
                        GeneratedField::ToolName => {
                            if tool_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("toolName"));
                            }
                            tool_name__ = map_.next_value()?;
                        }
                        GeneratedField::ToolCallId => {
                            if tool_call_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("toolCallId"));
                            }
                            tool_call_id__ = map_.next_value()?;
                        }
                    }
                }
                Ok(PartStart {
                    part_id: part_id__.unwrap_or_default(),
                    kind: kind__.unwrap_or_default(),
                    tool_name: tool_name__,
                    tool_call_id: tool_call_id__,
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.PartStart", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Provider {
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
        if !self.provider_type.is_empty() {
            len += 1;
        }
        if self.source != 0 {
            len += 1;
        }
        if self.user_id.is_some() {
            len += 1;
        }
        if self.config.is_some() {
            len += 1;
        }
        if !self.credentials.is_empty() {
            len += 1;
        }
        if !self.endpoint_url.is_empty() {
            len += 1;
        }
        if self.is_active {
            len += 1;
        }
        if !self.created_at.is_empty() {
            len += 1;
        }
        if !self.updated_at.is_empty() {
            len += 1;
        }
        if self.identifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.Provider", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.provider_type.is_empty() {
            struct_ser.serialize_field("providerType", &self.provider_type)?;
        }
        if self.source != 0 {
            let v = ProviderSource::try_from(self.source)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.source)))?;
            struct_ser.serialize_field("source", &v)?;
        }
        if let Some(v) = self.user_id.as_ref() {
            struct_ser.serialize_field("userId", v)?;
        }
        if let Some(v) = self.config.as_ref() {
            struct_ser.serialize_field("config", v)?;
        }
        if !self.credentials.is_empty() {
            struct_ser.serialize_field("credentials", &self.credentials)?;
        }
        if !self.endpoint_url.is_empty() {
            struct_ser.serialize_field("endpointUrl", &self.endpoint_url)?;
        }
        if self.is_active {
            struct_ser.serialize_field("isActive", &self.is_active)?;
        }
        if !self.created_at.is_empty() {
            struct_ser.serialize_field("createdAt", &self.created_at)?;
        }
        if !self.updated_at.is_empty() {
            struct_ser.serialize_field("updatedAt", &self.updated_at)?;
        }
        if let Some(v) = self.identifier.as_ref() {
            match v {
                provider::Identifier::Id(v) => {
                    struct_ser.serialize_field("id", v)?;
                }
                provider::Identifier::Pid(v) => {
                    struct_ser.serialize_field("pid", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Provider {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "provider_type",
            "providerType",
            "source",
            "user_id",
            "userId",
            "config",
            "credentials",
            "endpoint_url",
            "endpointUrl",
            "is_active",
            "isActive",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
            "id",
            "pid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            ProviderType,
            Source,
            UserId,
            Config,
            Credentials,
            EndpointUrl,
            IsActive,
            CreatedAt,
            UpdatedAt,
            Id,
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
                            "name" => Ok(GeneratedField::Name),
                            "providerType" | "provider_type" => Ok(GeneratedField::ProviderType),
                            "source" => Ok(GeneratedField::Source),
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
                            "config" => Ok(GeneratedField::Config),
                            "credentials" => Ok(GeneratedField::Credentials),
                            "endpointUrl" | "endpoint_url" => Ok(GeneratedField::EndpointUrl),
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            "id" => Ok(GeneratedField::Id),
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
            type Value = Provider;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.Provider")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Provider, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut provider_type__ = None;
                let mut source__ = None;
                let mut user_id__ = None;
                let mut config__ = None;
                let mut credentials__ = None;
                let mut endpoint_url__ = None;
                let mut is_active__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                let mut identifier__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ProviderType => {
                            if provider_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("providerType"));
                            }
                            provider_type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Source => {
                            if source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            source__ = Some(map_.next_value::<ProviderSource>()? as i32);
                        }
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = map_.next_value()?;
                        }
                        GeneratedField::Config => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config__ = map_.next_value()?;
                        }
                        GeneratedField::Credentials => {
                            if credentials__.is_some() {
                                return Err(serde::de::Error::duplicate_field("credentials"));
                            }
                            credentials__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EndpointUrl => {
                            if endpoint_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endpointUrl"));
                            }
                            endpoint_url__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsActive => {
                            if is_active__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isActive"));
                            }
                            is_active__ = Some(map_.next_value()?);
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
                        GeneratedField::Id => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            identifier__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| provider::Identifier::Id(x.0));
                        }
                        GeneratedField::Pid => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pid"));
                            }
                            identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(provider::Identifier::Pid);
                        }
                    }
                }
                Ok(Provider {
                    name: name__.unwrap_or_default(),
                    provider_type: provider_type__.unwrap_or_default(),
                    source: source__.unwrap_or_default(),
                    user_id: user_id__,
                    config: config__,
                    credentials: credentials__.unwrap_or_default(),
                    endpoint_url: endpoint_url__.unwrap_or_default(),
                    is_active: is_active__.unwrap_or_default(),
                    created_at: created_at__.unwrap_or_default(),
                    updated_at: updated_at__.unwrap_or_default(),
                    identifier: identifier__,
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.Provider", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ProviderSource {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "PROVIDER_SOURCE_UNSPECIFIED",
            Self::System => "PROVIDER_SOURCE_SYSTEM",
            Self::User => "PROVIDER_SOURCE_USER",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ProviderSource {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "PROVIDER_SOURCE_UNSPECIFIED",
            "PROVIDER_SOURCE_SYSTEM",
            "PROVIDER_SOURCE_USER",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProviderSource;

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
                    "PROVIDER_SOURCE_UNSPECIFIED" => Ok(ProviderSource::Unspecified),
                    "PROVIDER_SOURCE_SYSTEM" => Ok(ProviderSource::System),
                    "PROVIDER_SOURCE_USER" => Ok(ProviderSource::User),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for RefreshProviderModelsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.provider_pid.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.RefreshProviderModelsRequest", len)?;
        if let Some(v) = self.provider_pid.as_ref() {
            struct_ser.serialize_field("providerPid", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RefreshProviderModelsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "provider_pid",
            "providerPid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProviderPid,
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
                            "providerPid" | "provider_pid" => Ok(GeneratedField::ProviderPid),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RefreshProviderModelsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.RefreshProviderModelsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RefreshProviderModelsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut provider_pid__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProviderPid => {
                            if provider_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("providerPid"));
                            }
                            provider_pid__ = map_.next_value()?;
                        }
                    }
                }
                Ok(RefreshProviderModelsRequest {
                    provider_pid: provider_pid__,
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.RefreshProviderModelsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RefreshProviderModelsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.providers_queued != 0 {
            len += 1;
        }
        if !self.provider_pids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.RefreshProviderModelsResponse", len)?;
        if self.providers_queued != 0 {
            struct_ser.serialize_field("providersQueued", &self.providers_queued)?;
        }
        if !self.provider_pids.is_empty() {
            struct_ser.serialize_field("providerPids", &self.provider_pids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RefreshProviderModelsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "providers_queued",
            "providersQueued",
            "provider_pids",
            "providerPids",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProvidersQueued,
            ProviderPids,
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
                            "providersQueued" | "providers_queued" => Ok(GeneratedField::ProvidersQueued),
                            "providerPids" | "provider_pids" => Ok(GeneratedField::ProviderPids),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RefreshProviderModelsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.RefreshProviderModelsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RefreshProviderModelsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut providers_queued__ = None;
                let mut provider_pids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProvidersQueued => {
                            if providers_queued__.is_some() {
                                return Err(serde::de::Error::duplicate_field("providersQueued"));
                            }
                            providers_queued__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ProviderPids => {
                            if provider_pids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("providerPids"));
                            }
                            provider_pids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RefreshProviderModelsResponse {
                    providers_queued: providers_queued__.unwrap_or_default(),
                    provider_pids: provider_pids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.RefreshProviderModelsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemoveDocumentsFromGroupRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.group_pid.is_empty() {
            len += 1;
        }
        if !self.document_pids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.RemoveDocumentsFromGroupRequest", len)?;
        if !self.group_pid.is_empty() {
            struct_ser.serialize_field("groupPid", &self.group_pid)?;
        }
        if !self.document_pids.is_empty() {
            struct_ser.serialize_field("documentPids", &self.document_pids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoveDocumentsFromGroupRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "group_pid",
            "groupPid",
            "document_pids",
            "documentPids",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            GroupPid,
            DocumentPids,
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
                            "groupPid" | "group_pid" => Ok(GeneratedField::GroupPid),
                            "documentPids" | "document_pids" => Ok(GeneratedField::DocumentPids),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RemoveDocumentsFromGroupRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.RemoveDocumentsFromGroupRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemoveDocumentsFromGroupRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut group_pid__ = None;
                let mut document_pids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::GroupPid => {
                            if group_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("groupPid"));
                            }
                            group_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DocumentPids => {
                            if document_pids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("documentPids"));
                            }
                            document_pids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RemoveDocumentsFromGroupRequest {
                    group_pid: group_pid__.unwrap_or_default(),
                    document_pids: document_pids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.RemoveDocumentsFromGroupRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemoveDocumentsFromGroupResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.group.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.RemoveDocumentsFromGroupResponse", len)?;
        if let Some(v) = self.group.as_ref() {
            struct_ser.serialize_field("group", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoveDocumentsFromGroupResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "group",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Group,
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
                            "group" => Ok(GeneratedField::Group),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RemoveDocumentsFromGroupResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.RemoveDocumentsFromGroupResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemoveDocumentsFromGroupResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut group__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Group => {
                            if group__.is_some() {
                                return Err(serde::de::Error::duplicate_field("group"));
                            }
                            group__ = map_.next_value()?;
                        }
                    }
                }
                Ok(RemoveDocumentsFromGroupResponse {
                    group: group__,
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.RemoveDocumentsFromGroupResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SearchChunksRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.query.is_empty() {
            len += 1;
        }
        if self.top_k.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.SearchChunksRequest", len)?;
        if !self.query.is_empty() {
            struct_ser.serialize_field("query", &self.query)?;
        }
        if let Some(v) = self.top_k.as_ref() {
            struct_ser.serialize_field("topK", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SearchChunksRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "query",
            "top_k",
            "topK",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Query,
            TopK,
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
                            "query" => Ok(GeneratedField::Query),
                            "topK" | "top_k" => Ok(GeneratedField::TopK),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SearchChunksRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.SearchChunksRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SearchChunksRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut query__ = None;
                let mut top_k__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Query => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            query__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TopK => {
                            if top_k__.is_some() {
                                return Err(serde::de::Error::duplicate_field("topK"));
                            }
                            top_k__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(SearchChunksRequest {
                    query: query__.unwrap_or_default(),
                    top_k: top_k__,
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.SearchChunksRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SearchChunksResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.chunks.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.SearchChunksResponse", len)?;
        if !self.chunks.is_empty() {
            struct_ser.serialize_field("chunks", &self.chunks)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SearchChunksResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "chunks",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Chunks,
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
                            "chunks" => Ok(GeneratedField::Chunks),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SearchChunksResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.SearchChunksResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SearchChunksResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut chunks__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Chunks => {
                            if chunks__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chunks"));
                            }
                            chunks__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SearchChunksResponse {
                    chunks: chunks__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.SearchChunksResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SearchDocumentsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.query.is_empty() {
            len += 1;
        }
        if !self.group_pids.is_empty() {
            len += 1;
        }
        if self.top_n.is_some() {
            len += 1;
        }
        if self.top_k.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.SearchDocumentsRequest", len)?;
        if !self.query.is_empty() {
            struct_ser.serialize_field("query", &self.query)?;
        }
        if !self.group_pids.is_empty() {
            struct_ser.serialize_field("groupPids", &self.group_pids)?;
        }
        if let Some(v) = self.top_n.as_ref() {
            struct_ser.serialize_field("topN", v)?;
        }
        if let Some(v) = self.top_k.as_ref() {
            struct_ser.serialize_field("topK", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SearchDocumentsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "query",
            "group_pids",
            "groupPids",
            "top_n",
            "topN",
            "top_k",
            "topK",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Query,
            GroupPids,
            TopN,
            TopK,
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
                            "query" => Ok(GeneratedField::Query),
                            "groupPids" | "group_pids" => Ok(GeneratedField::GroupPids),
                            "topN" | "top_n" => Ok(GeneratedField::TopN),
                            "topK" | "top_k" => Ok(GeneratedField::TopK),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SearchDocumentsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.SearchDocumentsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SearchDocumentsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut query__ = None;
                let mut group_pids__ = None;
                let mut top_n__ = None;
                let mut top_k__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Query => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            query__ = Some(map_.next_value()?);
                        }
                        GeneratedField::GroupPids => {
                            if group_pids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("groupPids"));
                            }
                            group_pids__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TopN => {
                            if top_n__.is_some() {
                                return Err(serde::de::Error::duplicate_field("topN"));
                            }
                            top_n__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::TopK => {
                            if top_k__.is_some() {
                                return Err(serde::de::Error::duplicate_field("topK"));
                            }
                            top_k__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(SearchDocumentsRequest {
                    query: query__.unwrap_or_default(),
                    group_pids: group_pids__.unwrap_or_default(),
                    top_n: top_n__,
                    top_k: top_k__,
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.SearchDocumentsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SearchDocumentsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.results.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.SearchDocumentsResponse", len)?;
        if !self.results.is_empty() {
            struct_ser.serialize_field("results", &self.results)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SearchDocumentsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "results",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Results,
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
                            "results" => Ok(GeneratedField::Results),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SearchDocumentsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.SearchDocumentsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SearchDocumentsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut results__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Results => {
                            if results__.is_some() {
                                return Err(serde::de::Error::duplicate_field("results"));
                            }
                            results__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SearchDocumentsResponse {
                    results: results__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.SearchDocumentsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SearchResult {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.document.is_some() {
            len += 1;
        }
        if !self.chunks.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.SearchResult", len)?;
        if let Some(v) = self.document.as_ref() {
            struct_ser.serialize_field("document", v)?;
        }
        if !self.chunks.is_empty() {
            struct_ser.serialize_field("chunks", &self.chunks)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SearchResult {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "document",
            "chunks",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Document,
            Chunks,
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
                            "document" => Ok(GeneratedField::Document),
                            "chunks" => Ok(GeneratedField::Chunks),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SearchResult;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.SearchResult")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SearchResult, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut document__ = None;
                let mut chunks__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Document => {
                            if document__.is_some() {
                                return Err(serde::de::Error::duplicate_field("document"));
                            }
                            document__ = map_.next_value()?;
                        }
                        GeneratedField::Chunks => {
                            if chunks__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chunks"));
                            }
                            chunks__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SearchResult {
                    document: document__,
                    chunks: chunks__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.SearchResult", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ShareAgentRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.agent_pid.is_empty() {
            len += 1;
        }
        if !self.user_id.is_empty() {
            len += 1;
        }
        if self.permissions != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.ShareAgentRequest", len)?;
        if !self.agent_pid.is_empty() {
            struct_ser.serialize_field("agentPid", &self.agent_pid)?;
        }
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if self.permissions != 0 {
            struct_ser.serialize_field("permissions", &self.permissions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ShareAgentRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "agent_pid",
            "agentPid",
            "user_id",
            "userId",
            "permissions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AgentPid,
            UserId,
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
                            "agentPid" | "agent_pid" => Ok(GeneratedField::AgentPid),
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
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
            type Value = ShareAgentRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.ShareAgentRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ShareAgentRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut agent_pid__ = None;
                let mut user_id__ = None;
                let mut permissions__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AgentPid => {
                            if agent_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("agentPid"));
                            }
                            agent_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Permissions => {
                            if permissions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("permissions"));
                            }
                            permissions__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ShareAgentRequest {
                    agent_pid: agent_pid__.unwrap_or_default(),
                    user_id: user_id__.unwrap_or_default(),
                    permissions: permissions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.ShareAgentRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ShareAgentResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("rig.v1.ShareAgentResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ShareAgentResponse {
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
            type Value = ShareAgentResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.ShareAgentResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ShareAgentResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ShareAgentResponse {
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.ShareAgentResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ShareConversationRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.conversation_pid.is_empty() {
            len += 1;
        }
        if !self.user_id.is_empty() {
            len += 1;
        }
        if self.permissions != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.ShareConversationRequest", len)?;
        if !self.conversation_pid.is_empty() {
            struct_ser.serialize_field("conversationPid", &self.conversation_pid)?;
        }
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if self.permissions != 0 {
            struct_ser.serialize_field("permissions", &self.permissions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ShareConversationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "conversation_pid",
            "conversationPid",
            "user_id",
            "userId",
            "permissions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ConversationPid,
            UserId,
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
                            "conversationPid" | "conversation_pid" => Ok(GeneratedField::ConversationPid),
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
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
            type Value = ShareConversationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.ShareConversationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ShareConversationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut conversation_pid__ = None;
                let mut user_id__ = None;
                let mut permissions__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ConversationPid => {
                            if conversation_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("conversationPid"));
                            }
                            conversation_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Permissions => {
                            if permissions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("permissions"));
                            }
                            permissions__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ShareConversationRequest {
                    conversation_pid: conversation_pid__.unwrap_or_default(),
                    user_id: user_id__.unwrap_or_default(),
                    permissions: permissions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.ShareConversationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ShareConversationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("rig.v1.ShareConversationResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ShareConversationResponse {
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
            type Value = ShareConversationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.ShareConversationResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ShareConversationResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ShareConversationResponse {
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.ShareConversationResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StartupAgentConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.use_system_providers_on_creation.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.StartupAgentConfig", len)?;
        if let Some(v) = self.use_system_providers_on_creation.as_ref() {
            struct_ser.serialize_field("useSystemProvidersOnCreation", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StartupAgentConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "use_system_providers_on_creation",
            "useSystemProvidersOnCreation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UseSystemProvidersOnCreation,
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
                            "useSystemProvidersOnCreation" | "use_system_providers_on_creation" => Ok(GeneratedField::UseSystemProvidersOnCreation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StartupAgentConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.StartupAgentConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StartupAgentConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut use_system_providers_on_creation__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UseSystemProvidersOnCreation => {
                            if use_system_providers_on_creation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("useSystemProvidersOnCreation"));
                            }
                            use_system_providers_on_creation__ = map_.next_value()?;
                        }
                    }
                }
                Ok(StartupAgentConfig {
                    use_system_providers_on_creation: use_system_providers_on_creation__,
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.StartupAgentConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Tool {
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
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.tool_type.is_empty() {
            len += 1;
        }
        if self.is_active {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.Tool", len)?;
        if !self.pid.is_empty() {
            struct_ser.serialize_field("pid", &self.pid)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.tool_type.is_empty() {
            struct_ser.serialize_field("toolType", &self.tool_type)?;
        }
        if self.is_active {
            struct_ser.serialize_field("isActive", &self.is_active)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Tool {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pid",
            "name",
            "description",
            "tool_type",
            "toolType",
            "is_active",
            "isActive",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pid,
            Name,
            Description,
            ToolType,
            IsActive,
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
                            "description" => Ok(GeneratedField::Description),
                            "toolType" | "tool_type" => Ok(GeneratedField::ToolType),
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Tool;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.Tool")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Tool, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pid__ = None;
                let mut name__ = None;
                let mut description__ = None;
                let mut tool_type__ = None;
                let mut is_active__ = None;
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
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ToolType => {
                            if tool_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("toolType"));
                            }
                            tool_type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsActive => {
                            if is_active__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isActive"));
                            }
                            is_active__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Tool {
                    pid: pid__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    tool_type: tool_type__.unwrap_or_default(),
                    is_active: is_active__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.Tool", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ToolCall {
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
        if !self.name.is_empty() {
            len += 1;
        }
        if self.arguments.is_some() {
            len += 1;
        }
        if self.status != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.ToolCall", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.arguments.as_ref() {
            struct_ser.serialize_field("arguments", v)?;
        }
        if self.status != 0 {
            let v = ToolExecutionStatus::try_from(self.status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ToolCall {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "name",
            "arguments",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Name,
            Arguments,
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
                            "id" => Ok(GeneratedField::Id),
                            "name" => Ok(GeneratedField::Name),
                            "arguments" => Ok(GeneratedField::Arguments),
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
            type Value = ToolCall;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.ToolCall")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ToolCall, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut name__ = None;
                let mut arguments__ = None;
                let mut status__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Arguments => {
                            if arguments__.is_some() {
                                return Err(serde::de::Error::duplicate_field("arguments"));
                            }
                            arguments__ = map_.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value::<ToolExecutionStatus>()? as i32);
                        }
                    }
                }
                Ok(ToolCall {
                    id: id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    arguments: arguments__,
                    status: status__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.ToolCall", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ToolExecutionStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "TOOL_EXECUTION_STATUS_UNSPECIFIED",
            Self::Calling => "TOOL_EXECUTION_STATUS_CALLING",
            Self::Running => "TOOL_EXECUTION_STATUS_RUNNING",
            Self::Completed => "TOOL_EXECUTION_STATUS_COMPLETED",
            Self::Failed => "TOOL_EXECUTION_STATUS_FAILED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ToolExecutionStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "TOOL_EXECUTION_STATUS_UNSPECIFIED",
            "TOOL_EXECUTION_STATUS_CALLING",
            "TOOL_EXECUTION_STATUS_RUNNING",
            "TOOL_EXECUTION_STATUS_COMPLETED",
            "TOOL_EXECUTION_STATUS_FAILED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ToolExecutionStatus;

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
                    "TOOL_EXECUTION_STATUS_UNSPECIFIED" => Ok(ToolExecutionStatus::Unspecified),
                    "TOOL_EXECUTION_STATUS_CALLING" => Ok(ToolExecutionStatus::Calling),
                    "TOOL_EXECUTION_STATUS_RUNNING" => Ok(ToolExecutionStatus::Running),
                    "TOOL_EXECUTION_STATUS_COMPLETED" => Ok(ToolExecutionStatus::Completed),
                    "TOOL_EXECUTION_STATUS_FAILED" => Ok(ToolExecutionStatus::Failed),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ToolResult {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tool_call_id.is_empty() {
            len += 1;
        }
        if !self.result.is_empty() {
            len += 1;
        }
        if self.status != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.ToolResult", len)?;
        if !self.tool_call_id.is_empty() {
            struct_ser.serialize_field("toolCallId", &self.tool_call_id)?;
        }
        if !self.result.is_empty() {
            struct_ser.serialize_field("result", &self.result)?;
        }
        if self.status != 0 {
            let v = ToolExecutionStatus::try_from(self.status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ToolResult {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tool_call_id",
            "toolCallId",
            "result",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ToolCallId,
            Result,
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
                            "toolCallId" | "tool_call_id" => Ok(GeneratedField::ToolCallId),
                            "result" => Ok(GeneratedField::Result),
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
            type Value = ToolResult;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.ToolResult")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ToolResult, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tool_call_id__ = None;
                let mut result__ = None;
                let mut status__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ToolCallId => {
                            if tool_call_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("toolCallId"));
                            }
                            tool_call_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Result => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("result"));
                            }
                            result__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value::<ToolExecutionStatus>()? as i32);
                        }
                    }
                }
                Ok(ToolResult {
                    tool_call_id: tool_call_id__.unwrap_or_default(),
                    result: result__.unwrap_or_default(),
                    status: status__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.ToolResult", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ToolStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tool_call_id.is_empty() {
            len += 1;
        }
        if self.status != 0 {
            len += 1;
        }
        if self.error.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.ToolStatus", len)?;
        if !self.tool_call_id.is_empty() {
            struct_ser.serialize_field("toolCallId", &self.tool_call_id)?;
        }
        if self.status != 0 {
            let v = ToolExecutionStatus::try_from(self.status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        if let Some(v) = self.error.as_ref() {
            struct_ser.serialize_field("error", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ToolStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tool_call_id",
            "toolCallId",
            "status",
            "error",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ToolCallId,
            Status,
            Error,
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
                            "toolCallId" | "tool_call_id" => Ok(GeneratedField::ToolCallId),
                            "status" => Ok(GeneratedField::Status),
                            "error" => Ok(GeneratedField::Error),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ToolStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.ToolStatus")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ToolStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tool_call_id__ = None;
                let mut status__ = None;
                let mut error__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ToolCallId => {
                            if tool_call_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("toolCallId"));
                            }
                            tool_call_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value::<ToolExecutionStatus>()? as i32);
                        }
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ToolStatus {
                    tool_call_id: tool_call_id__.unwrap_or_default(),
                    status: status__.unwrap_or_default(),
                    error: error__,
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.ToolStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UnshareAgentRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.agent_pid.is_empty() {
            len += 1;
        }
        if !self.user_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.UnshareAgentRequest", len)?;
        if !self.agent_pid.is_empty() {
            struct_ser.serialize_field("agentPid", &self.agent_pid)?;
        }
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UnshareAgentRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "agent_pid",
            "agentPid",
            "user_id",
            "userId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AgentPid,
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
                            "agentPid" | "agent_pid" => Ok(GeneratedField::AgentPid),
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
            type Value = UnshareAgentRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.UnshareAgentRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UnshareAgentRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut agent_pid__ = None;
                let mut user_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AgentPid => {
                            if agent_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("agentPid"));
                            }
                            agent_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UnshareAgentRequest {
                    agent_pid: agent_pid__.unwrap_or_default(),
                    user_id: user_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.UnshareAgentRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UnshareAgentResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("rig.v1.UnshareAgentResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UnshareAgentResponse {
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
            type Value = UnshareAgentResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.UnshareAgentResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UnshareAgentResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(UnshareAgentResponse {
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.UnshareAgentResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UnshareConversationRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.conversation_pid.is_empty() {
            len += 1;
        }
        if !self.user_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.UnshareConversationRequest", len)?;
        if !self.conversation_pid.is_empty() {
            struct_ser.serialize_field("conversationPid", &self.conversation_pid)?;
        }
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UnshareConversationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "conversation_pid",
            "conversationPid",
            "user_id",
            "userId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ConversationPid,
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
                            "conversationPid" | "conversation_pid" => Ok(GeneratedField::ConversationPid),
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
            type Value = UnshareConversationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.UnshareConversationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UnshareConversationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut conversation_pid__ = None;
                let mut user_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ConversationPid => {
                            if conversation_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("conversationPid"));
                            }
                            conversation_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UnshareConversationRequest {
                    conversation_pid: conversation_pid__.unwrap_or_default(),
                    user_id: user_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.UnshareConversationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UnshareConversationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("rig.v1.UnshareConversationResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UnshareConversationResponse {
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
            type Value = UnshareConversationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.UnshareConversationResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UnshareConversationResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(UnshareConversationResponse {
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.UnshareConversationResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateAgentRequest {
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
        if self.name.is_some() {
            len += 1;
        }
        if self.slug.is_some() {
            len += 1;
        }
        if self.model_pid.is_some() {
            len += 1;
        }
        if self.temperature.is_some() {
            len += 1;
        }
        if self.system_prompt.is_some() {
            len += 1;
        }
        if self.config.is_some() {
            len += 1;
        }
        if self.provider_pid.is_some() {
            len += 1;
        }
        if !self.tools.is_empty() {
            len += 1;
        }
        if self.is_active.is_some() {
            len += 1;
        }
        if self.kind.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.UpdateAgentRequest", len)?;
        if !self.pid.is_empty() {
            struct_ser.serialize_field("pid", &self.pid)?;
        }
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if let Some(v) = self.slug.as_ref() {
            struct_ser.serialize_field("slug", v)?;
        }
        if let Some(v) = self.model_pid.as_ref() {
            struct_ser.serialize_field("modelPid", v)?;
        }
        if let Some(v) = self.temperature.as_ref() {
            struct_ser.serialize_field("temperature", v)?;
        }
        if let Some(v) = self.system_prompt.as_ref() {
            struct_ser.serialize_field("systemPrompt", v)?;
        }
        if let Some(v) = self.config.as_ref() {
            struct_ser.serialize_field("config", v)?;
        }
        if let Some(v) = self.provider_pid.as_ref() {
            struct_ser.serialize_field("providerPid", v)?;
        }
        if !self.tools.is_empty() {
            struct_ser.serialize_field("tools", &self.tools)?;
        }
        if let Some(v) = self.is_active.as_ref() {
            struct_ser.serialize_field("isActive", v)?;
        }
        if let Some(v) = self.kind.as_ref() {
            let v = AgentKind::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("kind", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateAgentRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pid",
            "name",
            "slug",
            "model_pid",
            "modelPid",
            "temperature",
            "system_prompt",
            "systemPrompt",
            "config",
            "provider_pid",
            "providerPid",
            "tools",
            "is_active",
            "isActive",
            "kind",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pid,
            Name,
            Slug,
            ModelPid,
            Temperature,
            SystemPrompt,
            Config,
            ProviderPid,
            Tools,
            IsActive,
            Kind,
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
                            "modelPid" | "model_pid" => Ok(GeneratedField::ModelPid),
                            "temperature" => Ok(GeneratedField::Temperature),
                            "systemPrompt" | "system_prompt" => Ok(GeneratedField::SystemPrompt),
                            "config" => Ok(GeneratedField::Config),
                            "providerPid" | "provider_pid" => Ok(GeneratedField::ProviderPid),
                            "tools" => Ok(GeneratedField::Tools),
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
                            "kind" => Ok(GeneratedField::Kind),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateAgentRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.UpdateAgentRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateAgentRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pid__ = None;
                let mut name__ = None;
                let mut slug__ = None;
                let mut model_pid__ = None;
                let mut temperature__ = None;
                let mut system_prompt__ = None;
                let mut config__ = None;
                let mut provider_pid__ = None;
                let mut tools__ = None;
                let mut is_active__ = None;
                let mut kind__ = None;
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
                            name__ = map_.next_value()?;
                        }
                        GeneratedField::Slug => {
                            if slug__.is_some() {
                                return Err(serde::de::Error::duplicate_field("slug"));
                            }
                            slug__ = map_.next_value()?;
                        }
                        GeneratedField::ModelPid => {
                            if model_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("modelPid"));
                            }
                            model_pid__ = map_.next_value()?;
                        }
                        GeneratedField::Temperature => {
                            if temperature__.is_some() {
                                return Err(serde::de::Error::duplicate_field("temperature"));
                            }
                            temperature__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::SystemPrompt => {
                            if system_prompt__.is_some() {
                                return Err(serde::de::Error::duplicate_field("systemPrompt"));
                            }
                            system_prompt__ = map_.next_value()?;
                        }
                        GeneratedField::Config => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config__ = map_.next_value()?;
                        }
                        GeneratedField::ProviderPid => {
                            if provider_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("providerPid"));
                            }
                            provider_pid__ = map_.next_value()?;
                        }
                        GeneratedField::Tools => {
                            if tools__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tools"));
                            }
                            tools__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsActive => {
                            if is_active__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isActive"));
                            }
                            is_active__ = map_.next_value()?;
                        }
                        GeneratedField::Kind => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kind"));
                            }
                            kind__ = map_.next_value::<::std::option::Option<AgentKind>>()?.map(|x| x as i32);
                        }
                    }
                }
                Ok(UpdateAgentRequest {
                    pid: pid__.unwrap_or_default(),
                    name: name__,
                    slug: slug__,
                    model_pid: model_pid__,
                    temperature: temperature__,
                    system_prompt: system_prompt__,
                    config: config__,
                    provider_pid: provider_pid__,
                    tools: tools__.unwrap_or_default(),
                    is_active: is_active__,
                    kind: kind__,
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.UpdateAgentRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateAgentResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.agent.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.UpdateAgentResponse", len)?;
        if let Some(v) = self.agent.as_ref() {
            struct_ser.serialize_field("agent", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateAgentResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "agent",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Agent,
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
                            "agent" => Ok(GeneratedField::Agent),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateAgentResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.UpdateAgentResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateAgentResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut agent__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Agent => {
                            if agent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("agent"));
                            }
                            agent__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateAgentResponse {
                    agent: agent__,
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.UpdateAgentResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateAgentToolRequest {
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
        if self.is_active {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.UpdateAgentToolRequest", len)?;
        if !self.pid.is_empty() {
            struct_ser.serialize_field("pid", &self.pid)?;
        }
        if self.is_active {
            struct_ser.serialize_field("isActive", &self.is_active)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateAgentToolRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pid",
            "is_active",
            "isActive",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pid,
            IsActive,
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
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateAgentToolRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.UpdateAgentToolRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateAgentToolRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pid__ = None;
                let mut is_active__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Pid => {
                            if pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pid"));
                            }
                            pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsActive => {
                            if is_active__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isActive"));
                            }
                            is_active__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UpdateAgentToolRequest {
                    pid: pid__.unwrap_or_default(),
                    is_active: is_active__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.UpdateAgentToolRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateDocumentRequest {
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
        if self.filename.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.UpdateDocumentRequest", len)?;
        if !self.pid.is_empty() {
            struct_ser.serialize_field("pid", &self.pid)?;
        }
        if let Some(v) = self.filename.as_ref() {
            struct_ser.serialize_field("filename", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateDocumentRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pid",
            "filename",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pid,
            Filename,
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
                            "filename" => Ok(GeneratedField::Filename),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateDocumentRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.UpdateDocumentRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateDocumentRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pid__ = None;
                let mut filename__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Pid => {
                            if pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pid"));
                            }
                            pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Filename => {
                            if filename__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filename"));
                            }
                            filename__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateDocumentRequest {
                    pid: pid__.unwrap_or_default(),
                    filename: filename__,
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.UpdateDocumentRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateDocumentResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.document.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.UpdateDocumentResponse", len)?;
        if let Some(v) = self.document.as_ref() {
            struct_ser.serialize_field("document", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateDocumentResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "document",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Document,
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
                            "document" => Ok(GeneratedField::Document),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateDocumentResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.UpdateDocumentResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateDocumentResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut document__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Document => {
                            if document__.is_some() {
                                return Err(serde::de::Error::duplicate_field("document"));
                            }
                            document__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateDocumentResponse {
                    document: document__,
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.UpdateDocumentResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateGroupRequest {
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
        if self.name.is_some() {
            len += 1;
        }
        if self.description.is_some() {
            len += 1;
        }
        if self.is_org_shared.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.UpdateGroupRequest", len)?;
        if !self.pid.is_empty() {
            struct_ser.serialize_field("pid", &self.pid)?;
        }
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        if let Some(v) = self.is_org_shared.as_ref() {
            struct_ser.serialize_field("isOrgShared", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateGroupRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pid",
            "name",
            "description",
            "is_org_shared",
            "isOrgShared",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pid,
            Name,
            Description,
            IsOrgShared,
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
                            "description" => Ok(GeneratedField::Description),
                            "isOrgShared" | "is_org_shared" => Ok(GeneratedField::IsOrgShared),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateGroupRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.UpdateGroupRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateGroupRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pid__ = None;
                let mut name__ = None;
                let mut description__ = None;
                let mut is_org_shared__ = None;
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
                            name__ = map_.next_value()?;
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = map_.next_value()?;
                        }
                        GeneratedField::IsOrgShared => {
                            if is_org_shared__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isOrgShared"));
                            }
                            is_org_shared__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateGroupRequest {
                    pid: pid__.unwrap_or_default(),
                    name: name__,
                    description: description__,
                    is_org_shared: is_org_shared__,
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.UpdateGroupRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateGroupResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.group.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.UpdateGroupResponse", len)?;
        if let Some(v) = self.group.as_ref() {
            struct_ser.serialize_field("group", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateGroupResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "group",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Group,
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
                            "group" => Ok(GeneratedField::Group),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateGroupResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.UpdateGroupResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateGroupResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut group__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Group => {
                            if group__.is_some() {
                                return Err(serde::de::Error::duplicate_field("group"));
                            }
                            group__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateGroupResponse {
                    group: group__,
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.UpdateGroupResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateProviderRequest {
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
        if self.name.is_some() {
            len += 1;
        }
        if self.provider_type.is_some() {
            len += 1;
        }
        if self.config.is_some() {
            len += 1;
        }
        if self.credentials.is_some() {
            len += 1;
        }
        if self.endpoint_url.is_some() {
            len += 1;
        }
        if self.is_active.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.UpdateProviderRequest", len)?;
        if !self.pid.is_empty() {
            struct_ser.serialize_field("pid", &self.pid)?;
        }
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if let Some(v) = self.provider_type.as_ref() {
            struct_ser.serialize_field("providerType", v)?;
        }
        if let Some(v) = self.config.as_ref() {
            struct_ser.serialize_field("config", v)?;
        }
        if let Some(v) = self.credentials.as_ref() {
            struct_ser.serialize_field("credentials", v)?;
        }
        if let Some(v) = self.endpoint_url.as_ref() {
            struct_ser.serialize_field("endpointUrl", v)?;
        }
        if let Some(v) = self.is_active.as_ref() {
            struct_ser.serialize_field("isActive", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateProviderRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pid",
            "name",
            "provider_type",
            "providerType",
            "config",
            "credentials",
            "endpoint_url",
            "endpointUrl",
            "is_active",
            "isActive",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pid,
            Name,
            ProviderType,
            Config,
            Credentials,
            EndpointUrl,
            IsActive,
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
                            "providerType" | "provider_type" => Ok(GeneratedField::ProviderType),
                            "config" => Ok(GeneratedField::Config),
                            "credentials" => Ok(GeneratedField::Credentials),
                            "endpointUrl" | "endpoint_url" => Ok(GeneratedField::EndpointUrl),
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateProviderRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.UpdateProviderRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateProviderRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pid__ = None;
                let mut name__ = None;
                let mut provider_type__ = None;
                let mut config__ = None;
                let mut credentials__ = None;
                let mut endpoint_url__ = None;
                let mut is_active__ = None;
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
                            name__ = map_.next_value()?;
                        }
                        GeneratedField::ProviderType => {
                            if provider_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("providerType"));
                            }
                            provider_type__ = map_.next_value()?;
                        }
                        GeneratedField::Config => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config__ = map_.next_value()?;
                        }
                        GeneratedField::Credentials => {
                            if credentials__.is_some() {
                                return Err(serde::de::Error::duplicate_field("credentials"));
                            }
                            credentials__ = map_.next_value()?;
                        }
                        GeneratedField::EndpointUrl => {
                            if endpoint_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endpointUrl"));
                            }
                            endpoint_url__ = map_.next_value()?;
                        }
                        GeneratedField::IsActive => {
                            if is_active__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isActive"));
                            }
                            is_active__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateProviderRequest {
                    pid: pid__.unwrap_or_default(),
                    name: name__,
                    provider_type: provider_type__,
                    config: config__,
                    credentials: credentials__,
                    endpoint_url: endpoint_url__,
                    is_active: is_active__,
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.UpdateProviderRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateProviderResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.provider.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.UpdateProviderResponse", len)?;
        if let Some(v) = self.provider.as_ref() {
            struct_ser.serialize_field("provider", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateProviderResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "provider",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Provider,
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
                            "provider" => Ok(GeneratedField::Provider),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateProviderResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.UpdateProviderResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateProviderResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut provider__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Provider => {
                            if provider__.is_some() {
                                return Err(serde::de::Error::duplicate_field("provider"));
                            }
                            provider__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateProviderResponse {
                    provider: provider__,
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.UpdateProviderResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UploadDocumentRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.filename.is_empty() {
            len += 1;
        }
        if !self.content_type.is_empty() {
            len += 1;
        }
        if !self.content.is_empty() {
            len += 1;
        }
        if self.conversation_pid.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.UploadDocumentRequest", len)?;
        if !self.filename.is_empty() {
            struct_ser.serialize_field("filename", &self.filename)?;
        }
        if !self.content_type.is_empty() {
            struct_ser.serialize_field("contentType", &self.content_type)?;
        }
        if !self.content.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("content", pbjson::private::base64::encode(&self.content).as_str())?;
        }
        if let Some(v) = self.conversation_pid.as_ref() {
            struct_ser.serialize_field("conversationPid", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UploadDocumentRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "filename",
            "content_type",
            "contentType",
            "content",
            "conversation_pid",
            "conversationPid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Filename,
            ContentType,
            Content,
            ConversationPid,
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
                            "filename" => Ok(GeneratedField::Filename),
                            "contentType" | "content_type" => Ok(GeneratedField::ContentType),
                            "content" => Ok(GeneratedField::Content),
                            "conversationPid" | "conversation_pid" => Ok(GeneratedField::ConversationPid),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UploadDocumentRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.UploadDocumentRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UploadDocumentRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut filename__ = None;
                let mut content_type__ = None;
                let mut content__ = None;
                let mut conversation_pid__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Filename => {
                            if filename__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filename"));
                            }
                            filename__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ContentType => {
                            if content_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contentType"));
                            }
                            content_type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Content => {
                            if content__.is_some() {
                                return Err(serde::de::Error::duplicate_field("content"));
                            }
                            content__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ConversationPid => {
                            if conversation_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("conversationPid"));
                            }
                            conversation_pid__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UploadDocumentRequest {
                    filename: filename__.unwrap_or_default(),
                    content_type: content_type__.unwrap_or_default(),
                    content: content__.unwrap_or_default(),
                    conversation_pid: conversation_pid__,
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.UploadDocumentRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UploadDocumentResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.document.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rig.v1.UploadDocumentResponse", len)?;
        if let Some(v) = self.document.as_ref() {
            struct_ser.serialize_field("document", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UploadDocumentResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "document",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Document,
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
                            "document" => Ok(GeneratedField::Document),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UploadDocumentResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rig.v1.UploadDocumentResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UploadDocumentResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut document__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Document => {
                            if document__.is_some() {
                                return Err(serde::de::Error::duplicate_field("document"));
                            }
                            document__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UploadDocumentResponse {
                    document: document__,
                })
            }
        }
        deserializer.deserialize_struct("rig.v1.UploadDocumentResponse", FIELDS, GeneratedVisitor)
    }
}
