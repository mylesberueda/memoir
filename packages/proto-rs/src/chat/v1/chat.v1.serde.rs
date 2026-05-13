// @generated
impl serde::Serialize for AgentSender {
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
        if !self.agent_slug.is_empty() {
            len += 1;
        }
        if self.avatar_url.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.AgentSender", len)?;
        if !self.agent_pid.is_empty() {
            struct_ser.serialize_field("agentPid", &self.agent_pid)?;
        }
        if !self.agent_slug.is_empty() {
            struct_ser.serialize_field("agentSlug", &self.agent_slug)?;
        }
        if let Some(v) = self.avatar_url.as_ref() {
            struct_ser.serialize_field("avatarUrl", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AgentSender {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "agent_pid",
            "agentPid",
            "agent_slug",
            "agentSlug",
            "avatar_url",
            "avatarUrl",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AgentPid,
            AgentSlug,
            AvatarUrl,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "agentSlug" | "agent_slug" => Ok(GeneratedField::AgentSlug),
                            "avatarUrl" | "avatar_url" => Ok(GeneratedField::AvatarUrl),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AgentSender;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.AgentSender")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AgentSender, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut agent_pid__ = None;
                let mut agent_slug__ = None;
                let mut avatar_url__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AgentPid => {
                            if agent_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("agentPid"));
                            }
                            agent_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AgentSlug => {
                            if agent_slug__.is_some() {
                                return Err(serde::de::Error::duplicate_field("agentSlug"));
                            }
                            agent_slug__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AvatarUrl => {
                            if avatar_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("avatarUrl"));
                            }
                            avatar_url__ = map_.next_value()?;
                        }
                    }
                }
                Ok(AgentSender {
                    agent_pid: agent_pid__.unwrap_or_default(),
                    agent_slug: agent_slug__.unwrap_or_default(),
                    avatar_url: avatar_url__,
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.AgentSender", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AgentStreaming {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.agent_slug.is_empty() {
            len += 1;
        }
        if !self.content.is_empty() {
            len += 1;
        }
        if self.done {
            len += 1;
        }
        if !self.message_pid.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.AgentStreaming", len)?;
        if !self.agent_slug.is_empty() {
            struct_ser.serialize_field("agentSlug", &self.agent_slug)?;
        }
        if !self.content.is_empty() {
            struct_ser.serialize_field("content", &self.content)?;
        }
        if self.done {
            struct_ser.serialize_field("done", &self.done)?;
        }
        if !self.message_pid.is_empty() {
            struct_ser.serialize_field("messagePid", &self.message_pid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AgentStreaming {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "agent_slug",
            "agentSlug",
            "content",
            "done",
            "message_pid",
            "messagePid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AgentSlug,
            Content,
            Done,
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
                            "agentSlug" | "agent_slug" => Ok(GeneratedField::AgentSlug),
                            "content" => Ok(GeneratedField::Content),
                            "done" => Ok(GeneratedField::Done),
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
            type Value = AgentStreaming;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.AgentStreaming")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AgentStreaming, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut agent_slug__ = None;
                let mut content__ = None;
                let mut done__ = None;
                let mut message_pid__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AgentSlug => {
                            if agent_slug__.is_some() {
                                return Err(serde::de::Error::duplicate_field("agentSlug"));
                            }
                            agent_slug__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Content => {
                            if content__.is_some() {
                                return Err(serde::de::Error::duplicate_field("content"));
                            }
                            content__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Done => {
                            if done__.is_some() {
                                return Err(serde::de::Error::duplicate_field("done"));
                            }
                            done__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MessagePid => {
                            if message_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messagePid"));
                            }
                            message_pid__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AgentStreaming {
                    agent_slug: agent_slug__.unwrap_or_default(),
                    content: content__.unwrap_or_default(),
                    done: done__.unwrap_or_default(),
                    message_pid: message_pid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.AgentStreaming", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Attachment {
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
        if !self.url.is_empty() {
            len += 1;
        }
        if self.thumbnail_url.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.Attachment", len)?;
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
        if !self.url.is_empty() {
            struct_ser.serialize_field("url", &self.url)?;
        }
        if let Some(v) = self.thumbnail_url.as_ref() {
            struct_ser.serialize_field("thumbnailUrl", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Attachment {
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
            "url",
            "thumbnail_url",
            "thumbnailUrl",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pid,
            Filename,
            ContentType,
            SizeBytes,
            Url,
            ThumbnailUrl,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "url" => Ok(GeneratedField::Url),
                            "thumbnailUrl" | "thumbnail_url" => Ok(GeneratedField::ThumbnailUrl),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Attachment;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.Attachment")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Attachment, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pid__ = None;
                let mut filename__ = None;
                let mut content_type__ = None;
                let mut size_bytes__ = None;
                let mut url__ = None;
                let mut thumbnail_url__ = None;
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
                        GeneratedField::Url => {
                            if url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("url"));
                            }
                            url__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ThumbnailUrl => {
                            if thumbnail_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("thumbnailUrl"));
                            }
                            thumbnail_url__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Attachment {
                    pid: pid__.unwrap_or_default(),
                    filename: filename__.unwrap_or_default(),
                    content_type: content_type__.unwrap_or_default(),
                    size_bytes: size_bytes__.unwrap_or_default(),
                    url: url__.unwrap_or_default(),
                    thumbnail_url: thumbnail_url__,
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.Attachment", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AutoModAction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "AUTO_MOD_ACTION_UNSPECIFIED",
            Self::Flag => "AUTO_MOD_ACTION_FLAG",
            Self::Delete => "AUTO_MOD_ACTION_DELETE",
            Self::Mute => "AUTO_MOD_ACTION_MUTE",
            Self::Warn => "AUTO_MOD_ACTION_WARN",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for AutoModAction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "AUTO_MOD_ACTION_UNSPECIFIED",
            "AUTO_MOD_ACTION_FLAG",
            "AUTO_MOD_ACTION_DELETE",
            "AUTO_MOD_ACTION_MUTE",
            "AUTO_MOD_ACTION_WARN",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AutoModAction;

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
                    "AUTO_MOD_ACTION_UNSPECIFIED" => Ok(AutoModAction::Unspecified),
                    "AUTO_MOD_ACTION_FLAG" => Ok(AutoModAction::Flag),
                    "AUTO_MOD_ACTION_DELETE" => Ok(AutoModAction::Delete),
                    "AUTO_MOD_ACTION_MUTE" => Ok(AutoModAction::Mute),
                    "AUTO_MOD_ACTION_WARN" => Ok(AutoModAction::Warn),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for AutoModSettings {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.enabled {
            len += 1;
        }
        if self.spam_filter_enabled {
            len += 1;
        }
        if self.max_messages_per_minute != 0 {
            len += 1;
        }
        if self.max_duplicate_messages != 0 {
            len += 1;
        }
        if self.profanity_filter_enabled {
            len += 1;
        }
        if self.profanity_filter_level != 0 {
            len += 1;
        }
        if !self.custom_blocked_words.is_empty() {
            len += 1;
        }
        if !self.custom_allowed_words.is_empty() {
            len += 1;
        }
        if self.link_filter_enabled {
            len += 1;
        }
        if !self.allowed_domains.is_empty() {
            len += 1;
        }
        if self.block_all_links {
            len += 1;
        }
        if self.max_mentions_per_message != 0 {
            len += 1;
        }
        if self.block_everyone_mentions {
            len += 1;
        }
        if self.block_role_mentions {
            len += 1;
        }
        if self.default_action != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.AutoModSettings", len)?;
        if self.enabled {
            struct_ser.serialize_field("enabled", &self.enabled)?;
        }
        if self.spam_filter_enabled {
            struct_ser.serialize_field("spamFilterEnabled", &self.spam_filter_enabled)?;
        }
        if self.max_messages_per_minute != 0 {
            struct_ser.serialize_field("maxMessagesPerMinute", &self.max_messages_per_minute)?;
        }
        if self.max_duplicate_messages != 0 {
            struct_ser.serialize_field("maxDuplicateMessages", &self.max_duplicate_messages)?;
        }
        if self.profanity_filter_enabled {
            struct_ser.serialize_field("profanityFilterEnabled", &self.profanity_filter_enabled)?;
        }
        if self.profanity_filter_level != 0 {
            let v = ProfanityFilterLevel::try_from(self.profanity_filter_level)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.profanity_filter_level)))?;
            struct_ser.serialize_field("profanityFilterLevel", &v)?;
        }
        if !self.custom_blocked_words.is_empty() {
            struct_ser.serialize_field("customBlockedWords", &self.custom_blocked_words)?;
        }
        if !self.custom_allowed_words.is_empty() {
            struct_ser.serialize_field("customAllowedWords", &self.custom_allowed_words)?;
        }
        if self.link_filter_enabled {
            struct_ser.serialize_field("linkFilterEnabled", &self.link_filter_enabled)?;
        }
        if !self.allowed_domains.is_empty() {
            struct_ser.serialize_field("allowedDomains", &self.allowed_domains)?;
        }
        if self.block_all_links {
            struct_ser.serialize_field("blockAllLinks", &self.block_all_links)?;
        }
        if self.max_mentions_per_message != 0 {
            struct_ser.serialize_field("maxMentionsPerMessage", &self.max_mentions_per_message)?;
        }
        if self.block_everyone_mentions {
            struct_ser.serialize_field("blockEveryoneMentions", &self.block_everyone_mentions)?;
        }
        if self.block_role_mentions {
            struct_ser.serialize_field("blockRoleMentions", &self.block_role_mentions)?;
        }
        if self.default_action != 0 {
            let v = AutoModAction::try_from(self.default_action)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.default_action)))?;
            struct_ser.serialize_field("defaultAction", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AutoModSettings {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "enabled",
            "spam_filter_enabled",
            "spamFilterEnabled",
            "max_messages_per_minute",
            "maxMessagesPerMinute",
            "max_duplicate_messages",
            "maxDuplicateMessages",
            "profanity_filter_enabled",
            "profanityFilterEnabled",
            "profanity_filter_level",
            "profanityFilterLevel",
            "custom_blocked_words",
            "customBlockedWords",
            "custom_allowed_words",
            "customAllowedWords",
            "link_filter_enabled",
            "linkFilterEnabled",
            "allowed_domains",
            "allowedDomains",
            "block_all_links",
            "blockAllLinks",
            "max_mentions_per_message",
            "maxMentionsPerMessage",
            "block_everyone_mentions",
            "blockEveryoneMentions",
            "block_role_mentions",
            "blockRoleMentions",
            "default_action",
            "defaultAction",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Enabled,
            SpamFilterEnabled,
            MaxMessagesPerMinute,
            MaxDuplicateMessages,
            ProfanityFilterEnabled,
            ProfanityFilterLevel,
            CustomBlockedWords,
            CustomAllowedWords,
            LinkFilterEnabled,
            AllowedDomains,
            BlockAllLinks,
            MaxMentionsPerMessage,
            BlockEveryoneMentions,
            BlockRoleMentions,
            DefaultAction,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "enabled" => Ok(GeneratedField::Enabled),
                            "spamFilterEnabled" | "spam_filter_enabled" => Ok(GeneratedField::SpamFilterEnabled),
                            "maxMessagesPerMinute" | "max_messages_per_minute" => Ok(GeneratedField::MaxMessagesPerMinute),
                            "maxDuplicateMessages" | "max_duplicate_messages" => Ok(GeneratedField::MaxDuplicateMessages),
                            "profanityFilterEnabled" | "profanity_filter_enabled" => Ok(GeneratedField::ProfanityFilterEnabled),
                            "profanityFilterLevel" | "profanity_filter_level" => Ok(GeneratedField::ProfanityFilterLevel),
                            "customBlockedWords" | "custom_blocked_words" => Ok(GeneratedField::CustomBlockedWords),
                            "customAllowedWords" | "custom_allowed_words" => Ok(GeneratedField::CustomAllowedWords),
                            "linkFilterEnabled" | "link_filter_enabled" => Ok(GeneratedField::LinkFilterEnabled),
                            "allowedDomains" | "allowed_domains" => Ok(GeneratedField::AllowedDomains),
                            "blockAllLinks" | "block_all_links" => Ok(GeneratedField::BlockAllLinks),
                            "maxMentionsPerMessage" | "max_mentions_per_message" => Ok(GeneratedField::MaxMentionsPerMessage),
                            "blockEveryoneMentions" | "block_everyone_mentions" => Ok(GeneratedField::BlockEveryoneMentions),
                            "blockRoleMentions" | "block_role_mentions" => Ok(GeneratedField::BlockRoleMentions),
                            "defaultAction" | "default_action" => Ok(GeneratedField::DefaultAction),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AutoModSettings;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.AutoModSettings")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AutoModSettings, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut enabled__ = None;
                let mut spam_filter_enabled__ = None;
                let mut max_messages_per_minute__ = None;
                let mut max_duplicate_messages__ = None;
                let mut profanity_filter_enabled__ = None;
                let mut profanity_filter_level__ = None;
                let mut custom_blocked_words__ = None;
                let mut custom_allowed_words__ = None;
                let mut link_filter_enabled__ = None;
                let mut allowed_domains__ = None;
                let mut block_all_links__ = None;
                let mut max_mentions_per_message__ = None;
                let mut block_everyone_mentions__ = None;
                let mut block_role_mentions__ = None;
                let mut default_action__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Enabled => {
                            if enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enabled"));
                            }
                            enabled__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SpamFilterEnabled => {
                            if spam_filter_enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spamFilterEnabled"));
                            }
                            spam_filter_enabled__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MaxMessagesPerMinute => {
                            if max_messages_per_minute__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxMessagesPerMinute"));
                            }
                            max_messages_per_minute__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MaxDuplicateMessages => {
                            if max_duplicate_messages__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxDuplicateMessages"));
                            }
                            max_duplicate_messages__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ProfanityFilterEnabled => {
                            if profanity_filter_enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("profanityFilterEnabled"));
                            }
                            profanity_filter_enabled__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ProfanityFilterLevel => {
                            if profanity_filter_level__.is_some() {
                                return Err(serde::de::Error::duplicate_field("profanityFilterLevel"));
                            }
                            profanity_filter_level__ = Some(map_.next_value::<ProfanityFilterLevel>()? as i32);
                        }
                        GeneratedField::CustomBlockedWords => {
                            if custom_blocked_words__.is_some() {
                                return Err(serde::de::Error::duplicate_field("customBlockedWords"));
                            }
                            custom_blocked_words__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CustomAllowedWords => {
                            if custom_allowed_words__.is_some() {
                                return Err(serde::de::Error::duplicate_field("customAllowedWords"));
                            }
                            custom_allowed_words__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LinkFilterEnabled => {
                            if link_filter_enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("linkFilterEnabled"));
                            }
                            link_filter_enabled__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AllowedDomains => {
                            if allowed_domains__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowedDomains"));
                            }
                            allowed_domains__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BlockAllLinks => {
                            if block_all_links__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockAllLinks"));
                            }
                            block_all_links__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MaxMentionsPerMessage => {
                            if max_mentions_per_message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxMentionsPerMessage"));
                            }
                            max_mentions_per_message__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::BlockEveryoneMentions => {
                            if block_everyone_mentions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockEveryoneMentions"));
                            }
                            block_everyone_mentions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BlockRoleMentions => {
                            if block_role_mentions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockRoleMentions"));
                            }
                            block_role_mentions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DefaultAction => {
                            if default_action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultAction"));
                            }
                            default_action__ = Some(map_.next_value::<AutoModAction>()? as i32);
                        }
                    }
                }
                Ok(AutoModSettings {
                    enabled: enabled__.unwrap_or_default(),
                    spam_filter_enabled: spam_filter_enabled__.unwrap_or_default(),
                    max_messages_per_minute: max_messages_per_minute__.unwrap_or_default(),
                    max_duplicate_messages: max_duplicate_messages__.unwrap_or_default(),
                    profanity_filter_enabled: profanity_filter_enabled__.unwrap_or_default(),
                    profanity_filter_level: profanity_filter_level__.unwrap_or_default(),
                    custom_blocked_words: custom_blocked_words__.unwrap_or_default(),
                    custom_allowed_words: custom_allowed_words__.unwrap_or_default(),
                    link_filter_enabled: link_filter_enabled__.unwrap_or_default(),
                    allowed_domains: allowed_domains__.unwrap_or_default(),
                    block_all_links: block_all_links__.unwrap_or_default(),
                    max_mentions_per_message: max_mentions_per_message__.unwrap_or_default(),
                    block_everyone_mentions: block_everyone_mentions__.unwrap_or_default(),
                    block_role_mentions: block_role_mentions__.unwrap_or_default(),
                    default_action: default_action__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.AutoModSettings", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BanUserAction {
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
        if self.delete_messages.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.BanUserAction", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if let Some(v) = self.delete_messages.as_ref() {
            struct_ser.serialize_field("deleteMessages", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BanUserAction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
            "delete_messages",
            "deleteMessages",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
            DeleteMessages,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "deleteMessages" | "delete_messages" => Ok(GeneratedField::DeleteMessages),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BanUserAction;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.BanUserAction")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BanUserAction, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut delete_messages__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DeleteMessages => {
                            if delete_messages__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deleteMessages"));
                            }
                            delete_messages__ = map_.next_value()?;
                        }
                    }
                }
                Ok(BanUserAction {
                    user_id: user_id__.unwrap_or_default(),
                    delete_messages: delete_messages__,
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.BanUserAction", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BanUserRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.channel_pid.is_empty() {
            len += 1;
        }
        if !self.user_id.is_empty() {
            len += 1;
        }
        if self.reason.is_some() {
            len += 1;
        }
        if self.delete_messages.is_some() {
            len += 1;
        }
        if self.delete_messages_seconds.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.BanUserRequest", len)?;
        if !self.channel_pid.is_empty() {
            struct_ser.serialize_field("channelPid", &self.channel_pid)?;
        }
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if let Some(v) = self.reason.as_ref() {
            struct_ser.serialize_field("reason", v)?;
        }
        if let Some(v) = self.delete_messages.as_ref() {
            struct_ser.serialize_field("deleteMessages", v)?;
        }
        if let Some(v) = self.delete_messages_seconds.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("deleteMessagesSeconds", ToString::to_string(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BanUserRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "channel_pid",
            "channelPid",
            "user_id",
            "userId",
            "reason",
            "delete_messages",
            "deleteMessages",
            "delete_messages_seconds",
            "deleteMessagesSeconds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChannelPid,
            UserId,
            Reason,
            DeleteMessages,
            DeleteMessagesSeconds,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "channelPid" | "channel_pid" => Ok(GeneratedField::ChannelPid),
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
                            "reason" => Ok(GeneratedField::Reason),
                            "deleteMessages" | "delete_messages" => Ok(GeneratedField::DeleteMessages),
                            "deleteMessagesSeconds" | "delete_messages_seconds" => Ok(GeneratedField::DeleteMessagesSeconds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BanUserRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.BanUserRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BanUserRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut channel_pid__ = None;
                let mut user_id__ = None;
                let mut reason__ = None;
                let mut delete_messages__ = None;
                let mut delete_messages_seconds__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ChannelPid => {
                            if channel_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelPid"));
                            }
                            channel_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Reason => {
                            if reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            reason__ = map_.next_value()?;
                        }
                        GeneratedField::DeleteMessages => {
                            if delete_messages__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deleteMessages"));
                            }
                            delete_messages__ = map_.next_value()?;
                        }
                        GeneratedField::DeleteMessagesSeconds => {
                            if delete_messages_seconds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deleteMessagesSeconds"));
                            }
                            delete_messages_seconds__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(BanUserRequest {
                    channel_pid: channel_pid__.unwrap_or_default(),
                    user_id: user_id__.unwrap_or_default(),
                    reason: reason__,
                    delete_messages: delete_messages__,
                    delete_messages_seconds: delete_messages_seconds__,
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.BanUserRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BanUserResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("chat.v1.BanUserResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BanUserResponse {
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
            type Value = BanUserResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.BanUserResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BanUserResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(BanUserResponse {
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.BanUserResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BannedUser {
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
        if !self.user_name.is_empty() {
            len += 1;
        }
        if self.avatar_url.is_some() {
            len += 1;
        }
        if !self.banned_by_id.is_empty() {
            len += 1;
        }
        if self.reason.is_some() {
            len += 1;
        }
        if !self.banned_at.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.BannedUser", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if !self.user_name.is_empty() {
            struct_ser.serialize_field("userName", &self.user_name)?;
        }
        if let Some(v) = self.avatar_url.as_ref() {
            struct_ser.serialize_field("avatarUrl", v)?;
        }
        if !self.banned_by_id.is_empty() {
            struct_ser.serialize_field("bannedById", &self.banned_by_id)?;
        }
        if let Some(v) = self.reason.as_ref() {
            struct_ser.serialize_field("reason", v)?;
        }
        if !self.banned_at.is_empty() {
            struct_ser.serialize_field("bannedAt", &self.banned_at)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BannedUser {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
            "user_name",
            "userName",
            "avatar_url",
            "avatarUrl",
            "banned_by_id",
            "bannedById",
            "reason",
            "banned_at",
            "bannedAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
            UserName,
            AvatarUrl,
            BannedById,
            Reason,
            BannedAt,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "userName" | "user_name" => Ok(GeneratedField::UserName),
                            "avatarUrl" | "avatar_url" => Ok(GeneratedField::AvatarUrl),
                            "bannedById" | "banned_by_id" => Ok(GeneratedField::BannedById),
                            "reason" => Ok(GeneratedField::Reason),
                            "bannedAt" | "banned_at" => Ok(GeneratedField::BannedAt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BannedUser;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.BannedUser")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BannedUser, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut user_name__ = None;
                let mut avatar_url__ = None;
                let mut banned_by_id__ = None;
                let mut reason__ = None;
                let mut banned_at__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserName => {
                            if user_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userName"));
                            }
                            user_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AvatarUrl => {
                            if avatar_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("avatarUrl"));
                            }
                            avatar_url__ = map_.next_value()?;
                        }
                        GeneratedField::BannedById => {
                            if banned_by_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bannedById"));
                            }
                            banned_by_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Reason => {
                            if reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            reason__ = map_.next_value()?;
                        }
                        GeneratedField::BannedAt => {
                            if banned_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bannedAt"));
                            }
                            banned_at__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BannedUser {
                    user_id: user_id__.unwrap_or_default(),
                    user_name: user_name__.unwrap_or_default(),
                    avatar_url: avatar_url__,
                    banned_by_id: banned_by_id__.unwrap_or_default(),
                    reason: reason__,
                    banned_at: banned_at__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.BannedUser", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Channel {
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
        if self.channel_type != 0 {
            len += 1;
        }
        if self.description.is_some() {
            len += 1;
        }
        if self.organization_pid.is_some() {
            len += 1;
        }
        if self.icon_url.is_some() {
            len += 1;
        }
        if !self.created_at.is_empty() {
            len += 1;
        }
        if !self.updated_at.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.Channel", len)?;
        if !self.pid.is_empty() {
            struct_ser.serialize_field("pid", &self.pid)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.channel_type != 0 {
            let v = ChannelType::try_from(self.channel_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.channel_type)))?;
            struct_ser.serialize_field("channelType", &v)?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        if let Some(v) = self.organization_pid.as_ref() {
            struct_ser.serialize_field("organizationPid", v)?;
        }
        if let Some(v) = self.icon_url.as_ref() {
            struct_ser.serialize_field("iconUrl", v)?;
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
impl<'de> serde::Deserialize<'de> for Channel {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pid",
            "name",
            "channel_type",
            "channelType",
            "description",
            "organization_pid",
            "organizationPid",
            "icon_url",
            "iconUrl",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pid,
            Name,
            ChannelType,
            Description,
            OrganizationPid,
            IconUrl,
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
                            "channelType" | "channel_type" => Ok(GeneratedField::ChannelType),
                            "description" => Ok(GeneratedField::Description),
                            "organizationPid" | "organization_pid" => Ok(GeneratedField::OrganizationPid),
                            "iconUrl" | "icon_url" => Ok(GeneratedField::IconUrl),
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
            type Value = Channel;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.Channel")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Channel, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pid__ = None;
                let mut name__ = None;
                let mut channel_type__ = None;
                let mut description__ = None;
                let mut organization_pid__ = None;
                let mut icon_url__ = None;
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
                        GeneratedField::ChannelType => {
                            if channel_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelType"));
                            }
                            channel_type__ = Some(map_.next_value::<ChannelType>()? as i32);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = map_.next_value()?;
                        }
                        GeneratedField::OrganizationPid => {
                            if organization_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationPid"));
                            }
                            organization_pid__ = map_.next_value()?;
                        }
                        GeneratedField::IconUrl => {
                            if icon_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("iconUrl"));
                            }
                            icon_url__ = map_.next_value()?;
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
                Ok(Channel {
                    pid: pid__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    channel_type: channel_type__.unwrap_or_default(),
                    description: description__,
                    organization_pid: organization_pid__,
                    icon_url: icon_url__,
                    created_at: created_at__.unwrap_or_default(),
                    updated_at: updated_at__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.Channel", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ChannelMember {
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
        if !self.user_name.is_empty() {
            len += 1;
        }
        if self.avatar_url.is_some() {
            len += 1;
        }
        if self.role != 0 {
            len += 1;
        }
        if !self.joined_at.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.ChannelMember", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if !self.user_name.is_empty() {
            struct_ser.serialize_field("userName", &self.user_name)?;
        }
        if let Some(v) = self.avatar_url.as_ref() {
            struct_ser.serialize_field("avatarUrl", v)?;
        }
        if self.role != 0 {
            let v = ChannelRole::try_from(self.role)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.role)))?;
            struct_ser.serialize_field("role", &v)?;
        }
        if !self.joined_at.is_empty() {
            struct_ser.serialize_field("joinedAt", &self.joined_at)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ChannelMember {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
            "user_name",
            "userName",
            "avatar_url",
            "avatarUrl",
            "role",
            "joined_at",
            "joinedAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
            UserName,
            AvatarUrl,
            Role,
            JoinedAt,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "userName" | "user_name" => Ok(GeneratedField::UserName),
                            "avatarUrl" | "avatar_url" => Ok(GeneratedField::AvatarUrl),
                            "role" => Ok(GeneratedField::Role),
                            "joinedAt" | "joined_at" => Ok(GeneratedField::JoinedAt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ChannelMember;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.ChannelMember")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ChannelMember, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut user_name__ = None;
                let mut avatar_url__ = None;
                let mut role__ = None;
                let mut joined_at__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserName => {
                            if user_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userName"));
                            }
                            user_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AvatarUrl => {
                            if avatar_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("avatarUrl"));
                            }
                            avatar_url__ = map_.next_value()?;
                        }
                        GeneratedField::Role => {
                            if role__.is_some() {
                                return Err(serde::de::Error::duplicate_field("role"));
                            }
                            role__ = Some(map_.next_value::<ChannelRole>()? as i32);
                        }
                        GeneratedField::JoinedAt => {
                            if joined_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("joinedAt"));
                            }
                            joined_at__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ChannelMember {
                    user_id: user_id__.unwrap_or_default(),
                    user_name: user_name__.unwrap_or_default(),
                    avatar_url: avatar_url__,
                    role: role__.unwrap_or_default(),
                    joined_at: joined_at__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.ChannelMember", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ChannelRole {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "CHANNEL_ROLE_UNSPECIFIED",
            Self::Member => "CHANNEL_ROLE_MEMBER",
            Self::Moderator => "CHANNEL_ROLE_MODERATOR",
            Self::Admin => "CHANNEL_ROLE_ADMIN",
            Self::Owner => "CHANNEL_ROLE_OWNER",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ChannelRole {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "CHANNEL_ROLE_UNSPECIFIED",
            "CHANNEL_ROLE_MEMBER",
            "CHANNEL_ROLE_MODERATOR",
            "CHANNEL_ROLE_ADMIN",
            "CHANNEL_ROLE_OWNER",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ChannelRole;

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
                    "CHANNEL_ROLE_UNSPECIFIED" => Ok(ChannelRole::Unspecified),
                    "CHANNEL_ROLE_MEMBER" => Ok(ChannelRole::Member),
                    "CHANNEL_ROLE_MODERATOR" => Ok(ChannelRole::Moderator),
                    "CHANNEL_ROLE_ADMIN" => Ok(ChannelRole::Admin),
                    "CHANNEL_ROLE_OWNER" => Ok(ChannelRole::Owner),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ChannelSummary {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.channel_pid.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if self.channel_type != 0 {
            len += 1;
        }
        if self.has_unread {
            len += 1;
        }
        if self.unread_count != 0 {
            len += 1;
        }
        if self.last_message.is_some() {
            len += 1;
        }
        if self.last_read_at.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.ChannelSummary", len)?;
        if !self.channel_pid.is_empty() {
            struct_ser.serialize_field("channelPid", &self.channel_pid)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.channel_type != 0 {
            let v = ChannelType::try_from(self.channel_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.channel_type)))?;
            struct_ser.serialize_field("channelType", &v)?;
        }
        if self.has_unread {
            struct_ser.serialize_field("hasUnread", &self.has_unread)?;
        }
        if self.unread_count != 0 {
            struct_ser.serialize_field("unreadCount", &self.unread_count)?;
        }
        if let Some(v) = self.last_message.as_ref() {
            struct_ser.serialize_field("lastMessage", v)?;
        }
        if let Some(v) = self.last_read_at.as_ref() {
            struct_ser.serialize_field("lastReadAt", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ChannelSummary {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "channel_pid",
            "channelPid",
            "name",
            "channel_type",
            "channelType",
            "has_unread",
            "hasUnread",
            "unread_count",
            "unreadCount",
            "last_message",
            "lastMessage",
            "last_read_at",
            "lastReadAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChannelPid,
            Name,
            ChannelType,
            HasUnread,
            UnreadCount,
            LastMessage,
            LastReadAt,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "channelPid" | "channel_pid" => Ok(GeneratedField::ChannelPid),
                            "name" => Ok(GeneratedField::Name),
                            "channelType" | "channel_type" => Ok(GeneratedField::ChannelType),
                            "hasUnread" | "has_unread" => Ok(GeneratedField::HasUnread),
                            "unreadCount" | "unread_count" => Ok(GeneratedField::UnreadCount),
                            "lastMessage" | "last_message" => Ok(GeneratedField::LastMessage),
                            "lastReadAt" | "last_read_at" => Ok(GeneratedField::LastReadAt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ChannelSummary;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.ChannelSummary")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ChannelSummary, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut channel_pid__ = None;
                let mut name__ = None;
                let mut channel_type__ = None;
                let mut has_unread__ = None;
                let mut unread_count__ = None;
                let mut last_message__ = None;
                let mut last_read_at__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ChannelPid => {
                            if channel_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelPid"));
                            }
                            channel_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ChannelType => {
                            if channel_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelType"));
                            }
                            channel_type__ = Some(map_.next_value::<ChannelType>()? as i32);
                        }
                        GeneratedField::HasUnread => {
                            if has_unread__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hasUnread"));
                            }
                            has_unread__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UnreadCount => {
                            if unread_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unreadCount"));
                            }
                            unread_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::LastMessage => {
                            if last_message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastMessage"));
                            }
                            last_message__ = map_.next_value()?;
                        }
                        GeneratedField::LastReadAt => {
                            if last_read_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastReadAt"));
                            }
                            last_read_at__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ChannelSummary {
                    channel_pid: channel_pid__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    channel_type: channel_type__.unwrap_or_default(),
                    has_unread: has_unread__.unwrap_or_default(),
                    unread_count: unread_count__.unwrap_or_default(),
                    last_message: last_message__,
                    last_read_at: last_read_at__,
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.ChannelSummary", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ChannelType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "CHANNEL_TYPE_UNSPECIFIED",
            Self::Global => "CHANNEL_TYPE_GLOBAL",
            Self::Organization => "CHANNEL_TYPE_ORGANIZATION",
            Self::Private => "CHANNEL_TYPE_PRIVATE",
            Self::System => "CHANNEL_TYPE_SYSTEM",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ChannelType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "CHANNEL_TYPE_UNSPECIFIED",
            "CHANNEL_TYPE_GLOBAL",
            "CHANNEL_TYPE_ORGANIZATION",
            "CHANNEL_TYPE_PRIVATE",
            "CHANNEL_TYPE_SYSTEM",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ChannelType;

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
                    "CHANNEL_TYPE_UNSPECIFIED" => Ok(ChannelType::Unspecified),
                    "CHANNEL_TYPE_GLOBAL" => Ok(ChannelType::Global),
                    "CHANNEL_TYPE_ORGANIZATION" => Ok(ChannelType::Organization),
                    "CHANNEL_TYPE_PRIVATE" => Ok(ChannelType::Private),
                    "CHANNEL_TYPE_SYSTEM" => Ok(ChannelType::System),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ChannelUpdated {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.channel.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.ChannelUpdated", len)?;
        if let Some(v) = self.channel.as_ref() {
            struct_ser.serialize_field("channel", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ChannelUpdated {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "channel",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Channel,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "channel" => Ok(GeneratedField::Channel),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ChannelUpdated;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.ChannelUpdated")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ChannelUpdated, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut channel__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Channel => {
                            if channel__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channel"));
                            }
                            channel__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ChannelUpdated {
                    channel: channel__,
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.ChannelUpdated", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ChatMessage {
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
        if !self.channel_pid.is_empty() {
            len += 1;
        }
        if !self.sender_name.is_empty() {
            len += 1;
        }
        if !self.content.is_empty() {
            len += 1;
        }
        if self.parent_pid.is_some() {
            len += 1;
        }
        if !self.timestamp.is_empty() {
            len += 1;
        }
        if self.is_deleted {
            len += 1;
        }
        if self.is_edited {
            len += 1;
        }
        if self.edited_at.is_some() {
            len += 1;
        }
        if !self.reactions.is_empty() {
            len += 1;
        }
        if !self.attachments.is_empty() {
            len += 1;
        }
        if !self.mentions.is_empty() {
            len += 1;
        }
        if self.reply_count.is_some() {
            len += 1;
        }
        if self.is_pinned {
            len += 1;
        }
        if self.sender.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.ChatMessage", len)?;
        if !self.pid.is_empty() {
            struct_ser.serialize_field("pid", &self.pid)?;
        }
        if !self.channel_pid.is_empty() {
            struct_ser.serialize_field("channelPid", &self.channel_pid)?;
        }
        if !self.sender_name.is_empty() {
            struct_ser.serialize_field("senderName", &self.sender_name)?;
        }
        if !self.content.is_empty() {
            struct_ser.serialize_field("content", &self.content)?;
        }
        if let Some(v) = self.parent_pid.as_ref() {
            struct_ser.serialize_field("parentPid", v)?;
        }
        if !self.timestamp.is_empty() {
            struct_ser.serialize_field("timestamp", &self.timestamp)?;
        }
        if self.is_deleted {
            struct_ser.serialize_field("isDeleted", &self.is_deleted)?;
        }
        if self.is_edited {
            struct_ser.serialize_field("isEdited", &self.is_edited)?;
        }
        if let Some(v) = self.edited_at.as_ref() {
            struct_ser.serialize_field("editedAt", v)?;
        }
        if !self.reactions.is_empty() {
            struct_ser.serialize_field("reactions", &self.reactions)?;
        }
        if !self.attachments.is_empty() {
            struct_ser.serialize_field("attachments", &self.attachments)?;
        }
        if !self.mentions.is_empty() {
            struct_ser.serialize_field("mentions", &self.mentions)?;
        }
        if let Some(v) = self.reply_count.as_ref() {
            struct_ser.serialize_field("replyCount", v)?;
        }
        if self.is_pinned {
            struct_ser.serialize_field("isPinned", &self.is_pinned)?;
        }
        if let Some(v) = self.sender.as_ref() {
            match v {
                chat_message::Sender::User(v) => {
                    struct_ser.serialize_field("user", v)?;
                }
                chat_message::Sender::Agent(v) => {
                    struct_ser.serialize_field("agent", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ChatMessage {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pid",
            "channel_pid",
            "channelPid",
            "sender_name",
            "senderName",
            "content",
            "parent_pid",
            "parentPid",
            "timestamp",
            "is_deleted",
            "isDeleted",
            "is_edited",
            "isEdited",
            "edited_at",
            "editedAt",
            "reactions",
            "attachments",
            "mentions",
            "reply_count",
            "replyCount",
            "is_pinned",
            "isPinned",
            "user",
            "agent",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pid,
            ChannelPid,
            SenderName,
            Content,
            ParentPid,
            Timestamp,
            IsDeleted,
            IsEdited,
            EditedAt,
            Reactions,
            Attachments,
            Mentions,
            ReplyCount,
            IsPinned,
            User,
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
                            "pid" => Ok(GeneratedField::Pid),
                            "channelPid" | "channel_pid" => Ok(GeneratedField::ChannelPid),
                            "senderName" | "sender_name" => Ok(GeneratedField::SenderName),
                            "content" => Ok(GeneratedField::Content),
                            "parentPid" | "parent_pid" => Ok(GeneratedField::ParentPid),
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            "isDeleted" | "is_deleted" => Ok(GeneratedField::IsDeleted),
                            "isEdited" | "is_edited" => Ok(GeneratedField::IsEdited),
                            "editedAt" | "edited_at" => Ok(GeneratedField::EditedAt),
                            "reactions" => Ok(GeneratedField::Reactions),
                            "attachments" => Ok(GeneratedField::Attachments),
                            "mentions" => Ok(GeneratedField::Mentions),
                            "replyCount" | "reply_count" => Ok(GeneratedField::ReplyCount),
                            "isPinned" | "is_pinned" => Ok(GeneratedField::IsPinned),
                            "user" => Ok(GeneratedField::User),
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
            type Value = ChatMessage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.ChatMessage")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ChatMessage, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pid__ = None;
                let mut channel_pid__ = None;
                let mut sender_name__ = None;
                let mut content__ = None;
                let mut parent_pid__ = None;
                let mut timestamp__ = None;
                let mut is_deleted__ = None;
                let mut is_edited__ = None;
                let mut edited_at__ = None;
                let mut reactions__ = None;
                let mut attachments__ = None;
                let mut mentions__ = None;
                let mut reply_count__ = None;
                let mut is_pinned__ = None;
                let mut sender__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Pid => {
                            if pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pid"));
                            }
                            pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ChannelPid => {
                            if channel_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelPid"));
                            }
                            channel_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SenderName => {
                            if sender_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("senderName"));
                            }
                            sender_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Content => {
                            if content__.is_some() {
                                return Err(serde::de::Error::duplicate_field("content"));
                            }
                            content__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ParentPid => {
                            if parent_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parentPid"));
                            }
                            parent_pid__ = map_.next_value()?;
                        }
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsDeleted => {
                            if is_deleted__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isDeleted"));
                            }
                            is_deleted__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsEdited => {
                            if is_edited__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isEdited"));
                            }
                            is_edited__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EditedAt => {
                            if edited_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("editedAt"));
                            }
                            edited_at__ = map_.next_value()?;
                        }
                        GeneratedField::Reactions => {
                            if reactions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reactions"));
                            }
                            reactions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Attachments => {
                            if attachments__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attachments"));
                            }
                            attachments__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Mentions => {
                            if mentions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mentions"));
                            }
                            mentions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ReplyCount => {
                            if reply_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("replyCount"));
                            }
                            reply_count__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::IsPinned => {
                            if is_pinned__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isPinned"));
                            }
                            is_pinned__ = Some(map_.next_value()?);
                        }
                        GeneratedField::User => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            sender__ = map_.next_value::<::std::option::Option<_>>()?.map(chat_message::Sender::User)
;
                        }
                        GeneratedField::Agent => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("agent"));
                            }
                            sender__ = map_.next_value::<::std::option::Option<_>>()?.map(chat_message::Sender::Agent)
;
                        }
                    }
                }
                Ok(ChatMessage {
                    pid: pid__.unwrap_or_default(),
                    channel_pid: channel_pid__.unwrap_or_default(),
                    sender_name: sender_name__.unwrap_or_default(),
                    content: content__.unwrap_or_default(),
                    parent_pid: parent_pid__,
                    timestamp: timestamp__.unwrap_or_default(),
                    is_deleted: is_deleted__.unwrap_or_default(),
                    is_edited: is_edited__.unwrap_or_default(),
                    edited_at: edited_at__,
                    reactions: reactions__.unwrap_or_default(),
                    attachments: attachments__.unwrap_or_default(),
                    mentions: mentions__.unwrap_or_default(),
                    reply_count: reply_count__,
                    is_pinned: is_pinned__.unwrap_or_default(),
                    sender: sender__,
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.ChatMessage", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ClientEvent {
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
        let mut struct_ser = serializer.serialize_struct("chat.v1.ClientEvent", len)?;
        if let Some(v) = self.event.as_ref() {
            match v {
                client_event::Event::Subscribe(v) => {
                    struct_ser.serialize_field("subscribe", v)?;
                }
                client_event::Event::Unsubscribe(v) => {
                    struct_ser.serialize_field("unsubscribe", v)?;
                }
                client_event::Event::Typing(v) => {
                    struct_ser.serialize_field("typing", v)?;
                }
                client_event::Event::Presence(v) => {
                    struct_ser.serialize_field("presence", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ClientEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "subscribe",
            "unsubscribe",
            "typing",
            "presence",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Subscribe,
            Unsubscribe,
            Typing,
            Presence,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "subscribe" => Ok(GeneratedField::Subscribe),
                            "unsubscribe" => Ok(GeneratedField::Unsubscribe),
                            "typing" => Ok(GeneratedField::Typing),
                            "presence" => Ok(GeneratedField::Presence),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ClientEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.ClientEvent")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ClientEvent, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut event__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Subscribe => {
                            if event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subscribe"));
                            }
                            event__ = map_.next_value::<::std::option::Option<_>>()?.map(client_event::Event::Subscribe)
;
                        }
                        GeneratedField::Unsubscribe => {
                            if event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unsubscribe"));
                            }
                            event__ = map_.next_value::<::std::option::Option<_>>()?.map(client_event::Event::Unsubscribe)
;
                        }
                        GeneratedField::Typing => {
                            if event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typing"));
                            }
                            event__ = map_.next_value::<::std::option::Option<_>>()?.map(client_event::Event::Typing)
;
                        }
                        GeneratedField::Presence => {
                            if event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("presence"));
                            }
                            event__ = map_.next_value::<::std::option::Option<_>>()?.map(client_event::Event::Presence)
;
                        }
                    }
                }
                Ok(ClientEvent {
                    event: event__,
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.ClientEvent", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ConversationSummary {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.channel.is_some() {
            len += 1;
        }
        if !self.participants.is_empty() {
            len += 1;
        }
        if self.last_message.is_some() {
            len += 1;
        }
        if self.unread_count != 0 {
            len += 1;
        }
        if self.last_read_at.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.ConversationSummary", len)?;
        if let Some(v) = self.channel.as_ref() {
            struct_ser.serialize_field("channel", v)?;
        }
        if !self.participants.is_empty() {
            struct_ser.serialize_field("participants", &self.participants)?;
        }
        if let Some(v) = self.last_message.as_ref() {
            struct_ser.serialize_field("lastMessage", v)?;
        }
        if self.unread_count != 0 {
            struct_ser.serialize_field("unreadCount", &self.unread_count)?;
        }
        if let Some(v) = self.last_read_at.as_ref() {
            struct_ser.serialize_field("lastReadAt", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ConversationSummary {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "channel",
            "participants",
            "last_message",
            "lastMessage",
            "unread_count",
            "unreadCount",
            "last_read_at",
            "lastReadAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Channel,
            Participants,
            LastMessage,
            UnreadCount,
            LastReadAt,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "channel" => Ok(GeneratedField::Channel),
                            "participants" => Ok(GeneratedField::Participants),
                            "lastMessage" | "last_message" => Ok(GeneratedField::LastMessage),
                            "unreadCount" | "unread_count" => Ok(GeneratedField::UnreadCount),
                            "lastReadAt" | "last_read_at" => Ok(GeneratedField::LastReadAt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ConversationSummary;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.ConversationSummary")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ConversationSummary, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut channel__ = None;
                let mut participants__ = None;
                let mut last_message__ = None;
                let mut unread_count__ = None;
                let mut last_read_at__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Channel => {
                            if channel__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channel"));
                            }
                            channel__ = map_.next_value()?;
                        }
                        GeneratedField::Participants => {
                            if participants__.is_some() {
                                return Err(serde::de::Error::duplicate_field("participants"));
                            }
                            participants__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LastMessage => {
                            if last_message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastMessage"));
                            }
                            last_message__ = map_.next_value()?;
                        }
                        GeneratedField::UnreadCount => {
                            if unread_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unreadCount"));
                            }
                            unread_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::LastReadAt => {
                            if last_read_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastReadAt"));
                            }
                            last_read_at__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ConversationSummary {
                    channel: channel__,
                    participants: participants__.unwrap_or_default(),
                    last_message: last_message__,
                    unread_count: unread_count__.unwrap_or_default(),
                    last_read_at: last_read_at__,
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.ConversationSummary", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateChannelRequest {
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
        if self.channel_type != 0 {
            len += 1;
        }
        if self.description.is_some() {
            len += 1;
        }
        if self.organization_pid.is_some() {
            len += 1;
        }
        if !self.participant_ids.is_empty() {
            len += 1;
        }
        if self.icon_url.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.CreateChannelRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.channel_type != 0 {
            let v = ChannelType::try_from(self.channel_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.channel_type)))?;
            struct_ser.serialize_field("channelType", &v)?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        if let Some(v) = self.organization_pid.as_ref() {
            struct_ser.serialize_field("organizationPid", v)?;
        }
        if !self.participant_ids.is_empty() {
            struct_ser.serialize_field("participantIds", &self.participant_ids)?;
        }
        if let Some(v) = self.icon_url.as_ref() {
            struct_ser.serialize_field("iconUrl", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateChannelRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "channel_type",
            "channelType",
            "description",
            "organization_pid",
            "organizationPid",
            "participant_ids",
            "participantIds",
            "icon_url",
            "iconUrl",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            ChannelType,
            Description,
            OrganizationPid,
            ParticipantIds,
            IconUrl,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "channelType" | "channel_type" => Ok(GeneratedField::ChannelType),
                            "description" => Ok(GeneratedField::Description),
                            "organizationPid" | "organization_pid" => Ok(GeneratedField::OrganizationPid),
                            "participantIds" | "participant_ids" => Ok(GeneratedField::ParticipantIds),
                            "iconUrl" | "icon_url" => Ok(GeneratedField::IconUrl),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateChannelRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.CreateChannelRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateChannelRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut channel_type__ = None;
                let mut description__ = None;
                let mut organization_pid__ = None;
                let mut participant_ids__ = None;
                let mut icon_url__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ChannelType => {
                            if channel_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelType"));
                            }
                            channel_type__ = Some(map_.next_value::<ChannelType>()? as i32);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = map_.next_value()?;
                        }
                        GeneratedField::OrganizationPid => {
                            if organization_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationPid"));
                            }
                            organization_pid__ = map_.next_value()?;
                        }
                        GeneratedField::ParticipantIds => {
                            if participant_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("participantIds"));
                            }
                            participant_ids__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IconUrl => {
                            if icon_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("iconUrl"));
                            }
                            icon_url__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateChannelRequest {
                    name: name__.unwrap_or_default(),
                    channel_type: channel_type__.unwrap_or_default(),
                    description: description__,
                    organization_pid: organization_pid__,
                    participant_ids: participant_ids__.unwrap_or_default(),
                    icon_url: icon_url__,
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.CreateChannelRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateChannelResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.channel.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.CreateChannelResponse", len)?;
        if let Some(v) = self.channel.as_ref() {
            struct_ser.serialize_field("channel", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateChannelResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "channel",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Channel,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "channel" => Ok(GeneratedField::Channel),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateChannelResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.CreateChannelResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateChannelResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut channel__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Channel => {
                            if channel__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channel"));
                            }
                            channel__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateChannelResponse {
                    channel: channel__,
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.CreateChannelResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteChannelRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.channel_pid.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.DeleteChannelRequest", len)?;
        if !self.channel_pid.is_empty() {
            struct_ser.serialize_field("channelPid", &self.channel_pid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteChannelRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "channel_pid",
            "channelPid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChannelPid,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "channelPid" | "channel_pid" => Ok(GeneratedField::ChannelPid),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteChannelRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.DeleteChannelRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteChannelRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut channel_pid__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ChannelPid => {
                            if channel_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelPid"));
                            }
                            channel_pid__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeleteChannelRequest {
                    channel_pid: channel_pid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.DeleteChannelRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteChannelResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("chat.v1.DeleteChannelResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteChannelResponse {
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
            type Value = DeleteChannelResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.DeleteChannelResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteChannelResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(DeleteChannelResponse {
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.DeleteChannelResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteMessageAction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.message_pid.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.DeleteMessageAction", len)?;
        if !self.message_pid.is_empty() {
            struct_ser.serialize_field("messagePid", &self.message_pid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteMessageAction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "message_pid",
            "messagePid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = DeleteMessageAction;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.DeleteMessageAction")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteMessageAction, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut message_pid__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MessagePid => {
                            if message_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messagePid"));
                            }
                            message_pid__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeleteMessageAction {
                    message_pid: message_pid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.DeleteMessageAction", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteMessageRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.message_pid.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.DeleteMessageRequest", len)?;
        if !self.message_pid.is_empty() {
            struct_ser.serialize_field("messagePid", &self.message_pid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteMessageRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "message_pid",
            "messagePid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = DeleteMessageRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.DeleteMessageRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteMessageRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut message_pid__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MessagePid => {
                            if message_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messagePid"));
                            }
                            message_pid__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeleteMessageRequest {
                    message_pid: message_pid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.DeleteMessageRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteMessageResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("chat.v1.DeleteMessageResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteMessageResponse {
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
            type Value = DeleteMessageResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.DeleteMessageResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteMessageResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(DeleteMessageResponse {
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.DeleteMessageResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EditMessageRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.message_pid.is_empty() {
            len += 1;
        }
        if !self.new_content.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.EditMessageRequest", len)?;
        if !self.message_pid.is_empty() {
            struct_ser.serialize_field("messagePid", &self.message_pid)?;
        }
        if !self.new_content.is_empty() {
            struct_ser.serialize_field("newContent", &self.new_content)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EditMessageRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "message_pid",
            "messagePid",
            "new_content",
            "newContent",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MessagePid,
            NewContent,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "messagePid" | "message_pid" => Ok(GeneratedField::MessagePid),
                            "newContent" | "new_content" => Ok(GeneratedField::NewContent),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EditMessageRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.EditMessageRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EditMessageRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut message_pid__ = None;
                let mut new_content__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MessagePid => {
                            if message_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messagePid"));
                            }
                            message_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NewContent => {
                            if new_content__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newContent"));
                            }
                            new_content__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(EditMessageRequest {
                    message_pid: message_pid__.unwrap_or_default(),
                    new_content: new_content__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.EditMessageRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EditMessageResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.message.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.EditMessageResponse", len)?;
        if let Some(v) = self.message.as_ref() {
            struct_ser.serialize_field("message", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EditMessageResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "message",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Message,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "message" => Ok(GeneratedField::Message),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EditMessageResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.EditMessageResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EditMessageResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut message__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = map_.next_value()?;
                        }
                    }
                }
                Ok(EditMessageResponse {
                    message: message__,
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.EditMessageResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetAutoModSettingsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.channel_pid.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.GetAutoModSettingsRequest", len)?;
        if !self.channel_pid.is_empty() {
            struct_ser.serialize_field("channelPid", &self.channel_pid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetAutoModSettingsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "channel_pid",
            "channelPid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChannelPid,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "channelPid" | "channel_pid" => Ok(GeneratedField::ChannelPid),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetAutoModSettingsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.GetAutoModSettingsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetAutoModSettingsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut channel_pid__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ChannelPid => {
                            if channel_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelPid"));
                            }
                            channel_pid__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetAutoModSettingsRequest {
                    channel_pid: channel_pid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.GetAutoModSettingsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetAutoModSettingsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.settings.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.GetAutoModSettingsResponse", len)?;
        if let Some(v) = self.settings.as_ref() {
            struct_ser.serialize_field("settings", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetAutoModSettingsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "settings",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = GetAutoModSettingsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.GetAutoModSettingsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetAutoModSettingsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut settings__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Settings => {
                            if settings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("settings"));
                            }
                            settings__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetAutoModSettingsResponse {
                    settings: settings__,
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.GetAutoModSettingsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetBannedUsersRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.channel_pid.is_empty() {
            len += 1;
        }
        if self.limit.is_some() {
            len += 1;
        }
        if self.cursor.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.GetBannedUsersRequest", len)?;
        if !self.channel_pid.is_empty() {
            struct_ser.serialize_field("channelPid", &self.channel_pid)?;
        }
        if let Some(v) = self.limit.as_ref() {
            struct_ser.serialize_field("limit", v)?;
        }
        if let Some(v) = self.cursor.as_ref() {
            struct_ser.serialize_field("cursor", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetBannedUsersRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "channel_pid",
            "channelPid",
            "limit",
            "cursor",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChannelPid,
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
                            "channelPid" | "channel_pid" => Ok(GeneratedField::ChannelPid),
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
            type Value = GetBannedUsersRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.GetBannedUsersRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetBannedUsersRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut channel_pid__ = None;
                let mut limit__ = None;
                let mut cursor__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ChannelPid => {
                            if channel_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelPid"));
                            }
                            channel_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Limit => {
                            if limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("limit"));
                            }
                            limit__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
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
                Ok(GetBannedUsersRequest {
                    channel_pid: channel_pid__.unwrap_or_default(),
                    limit: limit__,
                    cursor: cursor__,
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.GetBannedUsersRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetBannedUsersResponse {
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
        let mut struct_ser = serializer.serialize_struct("chat.v1.GetBannedUsersResponse", len)?;
        if !self.users.is_empty() {
            struct_ser.serialize_field("users", &self.users)?;
        }
        if let Some(v) = self.next_cursor.as_ref() {
            struct_ser.serialize_field("nextCursor", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetBannedUsersResponse {
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
            type Value = GetBannedUsersResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.GetBannedUsersResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetBannedUsersResponse, V::Error>
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
                Ok(GetBannedUsersResponse {
                    users: users__.unwrap_or_default(),
                    next_cursor: next_cursor__,
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.GetBannedUsersResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetChannelMembersRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.channel_pid.is_empty() {
            len += 1;
        }
        if self.limit.is_some() {
            len += 1;
        }
        if self.cursor.is_some() {
            len += 1;
        }
        if self.role_filter.is_some() {
            len += 1;
        }
        if self.online_only.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.GetChannelMembersRequest", len)?;
        if !self.channel_pid.is_empty() {
            struct_ser.serialize_field("channelPid", &self.channel_pid)?;
        }
        if let Some(v) = self.limit.as_ref() {
            struct_ser.serialize_field("limit", v)?;
        }
        if let Some(v) = self.cursor.as_ref() {
            struct_ser.serialize_field("cursor", v)?;
        }
        if let Some(v) = self.role_filter.as_ref() {
            let v = ChannelRole::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("roleFilter", &v)?;
        }
        if let Some(v) = self.online_only.as_ref() {
            struct_ser.serialize_field("onlineOnly", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetChannelMembersRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "channel_pid",
            "channelPid",
            "limit",
            "cursor",
            "role_filter",
            "roleFilter",
            "online_only",
            "onlineOnly",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChannelPid,
            Limit,
            Cursor,
            RoleFilter,
            OnlineOnly,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "channelPid" | "channel_pid" => Ok(GeneratedField::ChannelPid),
                            "limit" => Ok(GeneratedField::Limit),
                            "cursor" => Ok(GeneratedField::Cursor),
                            "roleFilter" | "role_filter" => Ok(GeneratedField::RoleFilter),
                            "onlineOnly" | "online_only" => Ok(GeneratedField::OnlineOnly),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetChannelMembersRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.GetChannelMembersRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetChannelMembersRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut channel_pid__ = None;
                let mut limit__ = None;
                let mut cursor__ = None;
                let mut role_filter__ = None;
                let mut online_only__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ChannelPid => {
                            if channel_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelPid"));
                            }
                            channel_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Limit => {
                            if limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("limit"));
                            }
                            limit__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Cursor => {
                            if cursor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cursor"));
                            }
                            cursor__ = map_.next_value()?;
                        }
                        GeneratedField::RoleFilter => {
                            if role_filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roleFilter"));
                            }
                            role_filter__ = map_.next_value::<::std::option::Option<ChannelRole>>()?.map(|x| x as i32);
                        }
                        GeneratedField::OnlineOnly => {
                            if online_only__.is_some() {
                                return Err(serde::de::Error::duplicate_field("onlineOnly"));
                            }
                            online_only__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetChannelMembersRequest {
                    channel_pid: channel_pid__.unwrap_or_default(),
                    limit: limit__,
                    cursor: cursor__,
                    role_filter: role_filter__,
                    online_only: online_only__,
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.GetChannelMembersRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetChannelMembersResponse {
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
        if self.total_count != 0 {
            len += 1;
        }
        if self.online_count != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.GetChannelMembersResponse", len)?;
        if !self.members.is_empty() {
            struct_ser.serialize_field("members", &self.members)?;
        }
        if let Some(v) = self.next_cursor.as_ref() {
            struct_ser.serialize_field("nextCursor", v)?;
        }
        if self.total_count != 0 {
            struct_ser.serialize_field("totalCount", &self.total_count)?;
        }
        if self.online_count != 0 {
            struct_ser.serialize_field("onlineCount", &self.online_count)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetChannelMembersResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "members",
            "next_cursor",
            "nextCursor",
            "total_count",
            "totalCount",
            "online_count",
            "onlineCount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Members,
            NextCursor,
            TotalCount,
            OnlineCount,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "totalCount" | "total_count" => Ok(GeneratedField::TotalCount),
                            "onlineCount" | "online_count" => Ok(GeneratedField::OnlineCount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetChannelMembersResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.GetChannelMembersResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetChannelMembersResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut members__ = None;
                let mut next_cursor__ = None;
                let mut total_count__ = None;
                let mut online_count__ = None;
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
                        GeneratedField::TotalCount => {
                            if total_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalCount"));
                            }
                            total_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::OnlineCount => {
                            if online_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("onlineCount"));
                            }
                            online_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(GetChannelMembersResponse {
                    members: members__.unwrap_or_default(),
                    next_cursor: next_cursor__,
                    total_count: total_count__.unwrap_or_default(),
                    online_count: online_count__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.GetChannelMembersResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetChannelRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.channel_pid.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.GetChannelRequest", len)?;
        if !self.channel_pid.is_empty() {
            struct_ser.serialize_field("channelPid", &self.channel_pid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetChannelRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "channel_pid",
            "channelPid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChannelPid,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "channelPid" | "channel_pid" => Ok(GeneratedField::ChannelPid),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetChannelRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.GetChannelRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetChannelRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut channel_pid__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ChannelPid => {
                            if channel_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelPid"));
                            }
                            channel_pid__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetChannelRequest {
                    channel_pid: channel_pid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.GetChannelRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetChannelResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.channel.is_some() {
            len += 1;
        }
        if self.my_membership.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.GetChannelResponse", len)?;
        if let Some(v) = self.channel.as_ref() {
            struct_ser.serialize_field("channel", v)?;
        }
        if let Some(v) = self.my_membership.as_ref() {
            struct_ser.serialize_field("myMembership", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetChannelResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "channel",
            "my_membership",
            "myMembership",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Channel,
            MyMembership,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "channel" => Ok(GeneratedField::Channel),
                            "myMembership" | "my_membership" => Ok(GeneratedField::MyMembership),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetChannelResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.GetChannelResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetChannelResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut channel__ = None;
                let mut my_membership__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Channel => {
                            if channel__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channel"));
                            }
                            channel__ = map_.next_value()?;
                        }
                        GeneratedField::MyMembership => {
                            if my_membership__.is_some() {
                                return Err(serde::de::Error::duplicate_field("myMembership"));
                            }
                            my_membership__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetChannelResponse {
                    channel: channel__,
                    my_membership: my_membership__,
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.GetChannelResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetChannelsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.channel_type.is_some() {
            len += 1;
        }
        if self.organization_pid.is_some() {
            len += 1;
        }
        if self.limit.is_some() {
            len += 1;
        }
        if self.cursor.is_some() {
            len += 1;
        }
        if self.include_archived.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.GetChannelsRequest", len)?;
        if let Some(v) = self.channel_type.as_ref() {
            let v = ChannelType::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("channelType", &v)?;
        }
        if let Some(v) = self.organization_pid.as_ref() {
            struct_ser.serialize_field("organizationPid", v)?;
        }
        if let Some(v) = self.limit.as_ref() {
            struct_ser.serialize_field("limit", v)?;
        }
        if let Some(v) = self.cursor.as_ref() {
            struct_ser.serialize_field("cursor", v)?;
        }
        if let Some(v) = self.include_archived.as_ref() {
            struct_ser.serialize_field("includeArchived", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetChannelsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "channel_type",
            "channelType",
            "organization_pid",
            "organizationPid",
            "limit",
            "cursor",
            "include_archived",
            "includeArchived",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChannelType,
            OrganizationPid,
            Limit,
            Cursor,
            IncludeArchived,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "channelType" | "channel_type" => Ok(GeneratedField::ChannelType),
                            "organizationPid" | "organization_pid" => Ok(GeneratedField::OrganizationPid),
                            "limit" => Ok(GeneratedField::Limit),
                            "cursor" => Ok(GeneratedField::Cursor),
                            "includeArchived" | "include_archived" => Ok(GeneratedField::IncludeArchived),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetChannelsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.GetChannelsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetChannelsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut channel_type__ = None;
                let mut organization_pid__ = None;
                let mut limit__ = None;
                let mut cursor__ = None;
                let mut include_archived__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ChannelType => {
                            if channel_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelType"));
                            }
                            channel_type__ = map_.next_value::<::std::option::Option<ChannelType>>()?.map(|x| x as i32);
                        }
                        GeneratedField::OrganizationPid => {
                            if organization_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationPid"));
                            }
                            organization_pid__ = map_.next_value()?;
                        }
                        GeneratedField::Limit => {
                            if limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("limit"));
                            }
                            limit__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Cursor => {
                            if cursor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cursor"));
                            }
                            cursor__ = map_.next_value()?;
                        }
                        GeneratedField::IncludeArchived => {
                            if include_archived__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includeArchived"));
                            }
                            include_archived__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetChannelsRequest {
                    channel_type: channel_type__,
                    organization_pid: organization_pid__,
                    limit: limit__,
                    cursor: cursor__,
                    include_archived: include_archived__,
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.GetChannelsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetChannelsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.channels.is_empty() {
            len += 1;
        }
        if self.next_cursor.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.GetChannelsResponse", len)?;
        if !self.channels.is_empty() {
            struct_ser.serialize_field("channels", &self.channels)?;
        }
        if let Some(v) = self.next_cursor.as_ref() {
            struct_ser.serialize_field("nextCursor", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetChannelsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "channels",
            "next_cursor",
            "nextCursor",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Channels,
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
                            "channels" => Ok(GeneratedField::Channels),
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
            type Value = GetChannelsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.GetChannelsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetChannelsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut channels__ = None;
                let mut next_cursor__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Channels => {
                            if channels__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channels"));
                            }
                            channels__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextCursor => {
                            if next_cursor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextCursor"));
                            }
                            next_cursor__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetChannelsResponse {
                    channels: channels__.unwrap_or_default(),
                    next_cursor: next_cursor__,
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.GetChannelsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetConversationsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.limit.is_some() {
            len += 1;
        }
        if self.cursor.is_some() {
            len += 1;
        }
        if self.unread_only.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.GetConversationsRequest", len)?;
        if let Some(v) = self.limit.as_ref() {
            struct_ser.serialize_field("limit", v)?;
        }
        if let Some(v) = self.cursor.as_ref() {
            struct_ser.serialize_field("cursor", v)?;
        }
        if let Some(v) = self.unread_only.as_ref() {
            struct_ser.serialize_field("unreadOnly", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetConversationsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "limit",
            "cursor",
            "unread_only",
            "unreadOnly",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Limit,
            Cursor,
            UnreadOnly,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "unreadOnly" | "unread_only" => Ok(GeneratedField::UnreadOnly),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetConversationsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.GetConversationsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetConversationsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut limit__ = None;
                let mut cursor__ = None;
                let mut unread_only__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Limit => {
                            if limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("limit"));
                            }
                            limit__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Cursor => {
                            if cursor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cursor"));
                            }
                            cursor__ = map_.next_value()?;
                        }
                        GeneratedField::UnreadOnly => {
                            if unread_only__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unreadOnly"));
                            }
                            unread_only__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetConversationsRequest {
                    limit: limit__,
                    cursor: cursor__,
                    unread_only: unread_only__,
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.GetConversationsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetConversationsResponse {
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
        if self.next_cursor.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.GetConversationsResponse", len)?;
        if !self.conversations.is_empty() {
            struct_ser.serialize_field("conversations", &self.conversations)?;
        }
        if let Some(v) = self.next_cursor.as_ref() {
            struct_ser.serialize_field("nextCursor", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetConversationsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "conversations",
            "next_cursor",
            "nextCursor",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Conversations,
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
                            "conversations" => Ok(GeneratedField::Conversations),
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
            type Value = GetConversationsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.GetConversationsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetConversationsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut conversations__ = None;
                let mut next_cursor__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Conversations => {
                            if conversations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("conversations"));
                            }
                            conversations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextCursor => {
                            if next_cursor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextCursor"));
                            }
                            next_cursor__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetConversationsResponse {
                    conversations: conversations__.unwrap_or_default(),
                    next_cursor: next_cursor__,
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.GetConversationsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetMessageHistoryRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.channel_pid.is_empty() {
            len += 1;
        }
        if self.limit != 0 {
            len += 1;
        }
        if self.before_timestamp.is_some() {
            len += 1;
        }
        if self.after_timestamp.is_some() {
            len += 1;
        }
        if self.around_message_pid.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.GetMessageHistoryRequest", len)?;
        if !self.channel_pid.is_empty() {
            struct_ser.serialize_field("channelPid", &self.channel_pid)?;
        }
        if self.limit != 0 {
            struct_ser.serialize_field("limit", &self.limit)?;
        }
        if let Some(v) = self.before_timestamp.as_ref() {
            struct_ser.serialize_field("beforeTimestamp", v)?;
        }
        if let Some(v) = self.after_timestamp.as_ref() {
            struct_ser.serialize_field("afterTimestamp", v)?;
        }
        if let Some(v) = self.around_message_pid.as_ref() {
            struct_ser.serialize_field("aroundMessagePid", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetMessageHistoryRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "channel_pid",
            "channelPid",
            "limit",
            "before_timestamp",
            "beforeTimestamp",
            "after_timestamp",
            "afterTimestamp",
            "around_message_pid",
            "aroundMessagePid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChannelPid,
            Limit,
            BeforeTimestamp,
            AfterTimestamp,
            AroundMessagePid,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "channelPid" | "channel_pid" => Ok(GeneratedField::ChannelPid),
                            "limit" => Ok(GeneratedField::Limit),
                            "beforeTimestamp" | "before_timestamp" => Ok(GeneratedField::BeforeTimestamp),
                            "afterTimestamp" | "after_timestamp" => Ok(GeneratedField::AfterTimestamp),
                            "aroundMessagePid" | "around_message_pid" => Ok(GeneratedField::AroundMessagePid),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetMessageHistoryRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.GetMessageHistoryRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetMessageHistoryRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut channel_pid__ = None;
                let mut limit__ = None;
                let mut before_timestamp__ = None;
                let mut after_timestamp__ = None;
                let mut around_message_pid__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ChannelPid => {
                            if channel_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelPid"));
                            }
                            channel_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Limit => {
                            if limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("limit"));
                            }
                            limit__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::BeforeTimestamp => {
                            if before_timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("beforeTimestamp"));
                            }
                            before_timestamp__ = map_.next_value()?;
                        }
                        GeneratedField::AfterTimestamp => {
                            if after_timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("afterTimestamp"));
                            }
                            after_timestamp__ = map_.next_value()?;
                        }
                        GeneratedField::AroundMessagePid => {
                            if around_message_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("aroundMessagePid"));
                            }
                            around_message_pid__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetMessageHistoryRequest {
                    channel_pid: channel_pid__.unwrap_or_default(),
                    limit: limit__.unwrap_or_default(),
                    before_timestamp: before_timestamp__,
                    after_timestamp: after_timestamp__,
                    around_message_pid: around_message_pid__,
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.GetMessageHistoryRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetMessageHistoryResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.messages.is_empty() {
            len += 1;
        }
        if self.has_more_before {
            len += 1;
        }
        if self.has_more_after {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.GetMessageHistoryResponse", len)?;
        if !self.messages.is_empty() {
            struct_ser.serialize_field("messages", &self.messages)?;
        }
        if self.has_more_before {
            struct_ser.serialize_field("hasMoreBefore", &self.has_more_before)?;
        }
        if self.has_more_after {
            struct_ser.serialize_field("hasMoreAfter", &self.has_more_after)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetMessageHistoryResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "messages",
            "has_more_before",
            "hasMoreBefore",
            "has_more_after",
            "hasMoreAfter",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Messages,
            HasMoreBefore,
            HasMoreAfter,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "messages" => Ok(GeneratedField::Messages),
                            "hasMoreBefore" | "has_more_before" => Ok(GeneratedField::HasMoreBefore),
                            "hasMoreAfter" | "has_more_after" => Ok(GeneratedField::HasMoreAfter),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetMessageHistoryResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.GetMessageHistoryResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetMessageHistoryResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut messages__ = None;
                let mut has_more_before__ = None;
                let mut has_more_after__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Messages => {
                            if messages__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messages"));
                            }
                            messages__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HasMoreBefore => {
                            if has_more_before__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hasMoreBefore"));
                            }
                            has_more_before__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HasMoreAfter => {
                            if has_more_after__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hasMoreAfter"));
                            }
                            has_more_after__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetMessageHistoryResponse {
                    messages: messages__.unwrap_or_default(),
                    has_more_before: has_more_before__.unwrap_or_default(),
                    has_more_after: has_more_after__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.GetMessageHistoryResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetMessageRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.message_pid.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.GetMessageRequest", len)?;
        if !self.message_pid.is_empty() {
            struct_ser.serialize_field("messagePid", &self.message_pid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetMessageRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "message_pid",
            "messagePid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = GetMessageRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.GetMessageRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetMessageRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut message_pid__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MessagePid => {
                            if message_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messagePid"));
                            }
                            message_pid__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetMessageRequest {
                    message_pid: message_pid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.GetMessageRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetMessageResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.message.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.GetMessageResponse", len)?;
        if let Some(v) = self.message.as_ref() {
            struct_ser.serialize_field("message", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetMessageResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "message",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Message,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "message" => Ok(GeneratedField::Message),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetMessageResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.GetMessageResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetMessageResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut message__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetMessageResponse {
                    message: message__,
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.GetMessageResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetModerationLogRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.channel_pid.is_empty() {
            len += 1;
        }
        if self.moderator_id.is_some() {
            len += 1;
        }
        if self.target_user_id.is_some() {
            len += 1;
        }
        if self.action_type.is_some() {
            len += 1;
        }
        if self.limit.is_some() {
            len += 1;
        }
        if self.cursor.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.GetModerationLogRequest", len)?;
        if !self.channel_pid.is_empty() {
            struct_ser.serialize_field("channelPid", &self.channel_pid)?;
        }
        if let Some(v) = self.moderator_id.as_ref() {
            struct_ser.serialize_field("moderatorId", v)?;
        }
        if let Some(v) = self.target_user_id.as_ref() {
            struct_ser.serialize_field("targetUserId", v)?;
        }
        if let Some(v) = self.action_type.as_ref() {
            let v = ModerationActionType::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("actionType", &v)?;
        }
        if let Some(v) = self.limit.as_ref() {
            struct_ser.serialize_field("limit", v)?;
        }
        if let Some(v) = self.cursor.as_ref() {
            struct_ser.serialize_field("cursor", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetModerationLogRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "channel_pid",
            "channelPid",
            "moderator_id",
            "moderatorId",
            "target_user_id",
            "targetUserId",
            "action_type",
            "actionType",
            "limit",
            "cursor",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChannelPid,
            ModeratorId,
            TargetUserId,
            ActionType,
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
                            "channelPid" | "channel_pid" => Ok(GeneratedField::ChannelPid),
                            "moderatorId" | "moderator_id" => Ok(GeneratedField::ModeratorId),
                            "targetUserId" | "target_user_id" => Ok(GeneratedField::TargetUserId),
                            "actionType" | "action_type" => Ok(GeneratedField::ActionType),
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
            type Value = GetModerationLogRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.GetModerationLogRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetModerationLogRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut channel_pid__ = None;
                let mut moderator_id__ = None;
                let mut target_user_id__ = None;
                let mut action_type__ = None;
                let mut limit__ = None;
                let mut cursor__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ChannelPid => {
                            if channel_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelPid"));
                            }
                            channel_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ModeratorId => {
                            if moderator_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("moderatorId"));
                            }
                            moderator_id__ = map_.next_value()?;
                        }
                        GeneratedField::TargetUserId => {
                            if target_user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetUserId"));
                            }
                            target_user_id__ = map_.next_value()?;
                        }
                        GeneratedField::ActionType => {
                            if action_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actionType"));
                            }
                            action_type__ = map_.next_value::<::std::option::Option<ModerationActionType>>()?.map(|x| x as i32);
                        }
                        GeneratedField::Limit => {
                            if limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("limit"));
                            }
                            limit__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
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
                Ok(GetModerationLogRequest {
                    channel_pid: channel_pid__.unwrap_or_default(),
                    moderator_id: moderator_id__,
                    target_user_id: target_user_id__,
                    action_type: action_type__,
                    limit: limit__,
                    cursor: cursor__,
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.GetModerationLogRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetModerationLogResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.entries.is_empty() {
            len += 1;
        }
        if self.next_cursor.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.GetModerationLogResponse", len)?;
        if !self.entries.is_empty() {
            struct_ser.serialize_field("entries", &self.entries)?;
        }
        if let Some(v) = self.next_cursor.as_ref() {
            struct_ser.serialize_field("nextCursor", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetModerationLogResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "entries",
            "next_cursor",
            "nextCursor",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Entries,
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
                            "entries" => Ok(GeneratedField::Entries),
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
            type Value = GetModerationLogResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.GetModerationLogResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetModerationLogResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut entries__ = None;
                let mut next_cursor__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Entries => {
                            if entries__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entries"));
                            }
                            entries__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextCursor => {
                            if next_cursor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextCursor"));
                            }
                            next_cursor__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetModerationLogResponse {
                    entries: entries__.unwrap_or_default(),
                    next_cursor: next_cursor__,
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.GetModerationLogResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetMutedUsersRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.channel_pid.is_empty() {
            len += 1;
        }
        if self.limit.is_some() {
            len += 1;
        }
        if self.cursor.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.GetMutedUsersRequest", len)?;
        if !self.channel_pid.is_empty() {
            struct_ser.serialize_field("channelPid", &self.channel_pid)?;
        }
        if let Some(v) = self.limit.as_ref() {
            struct_ser.serialize_field("limit", v)?;
        }
        if let Some(v) = self.cursor.as_ref() {
            struct_ser.serialize_field("cursor", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetMutedUsersRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "channel_pid",
            "channelPid",
            "limit",
            "cursor",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChannelPid,
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
                            "channelPid" | "channel_pid" => Ok(GeneratedField::ChannelPid),
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
            type Value = GetMutedUsersRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.GetMutedUsersRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetMutedUsersRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut channel_pid__ = None;
                let mut limit__ = None;
                let mut cursor__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ChannelPid => {
                            if channel_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelPid"));
                            }
                            channel_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Limit => {
                            if limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("limit"));
                            }
                            limit__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
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
                Ok(GetMutedUsersRequest {
                    channel_pid: channel_pid__.unwrap_or_default(),
                    limit: limit__,
                    cursor: cursor__,
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.GetMutedUsersRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetMutedUsersResponse {
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
        let mut struct_ser = serializer.serialize_struct("chat.v1.GetMutedUsersResponse", len)?;
        if !self.users.is_empty() {
            struct_ser.serialize_field("users", &self.users)?;
        }
        if let Some(v) = self.next_cursor.as_ref() {
            struct_ser.serialize_field("nextCursor", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetMutedUsersResponse {
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
            type Value = GetMutedUsersResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.GetMutedUsersResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetMutedUsersResponse, V::Error>
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
                Ok(GetMutedUsersResponse {
                    users: users__.unwrap_or_default(),
                    next_cursor: next_cursor__,
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.GetMutedUsersResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetPinnedMessagesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.channel_pid.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.GetPinnedMessagesRequest", len)?;
        if !self.channel_pid.is_empty() {
            struct_ser.serialize_field("channelPid", &self.channel_pid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetPinnedMessagesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "channel_pid",
            "channelPid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChannelPid,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "channelPid" | "channel_pid" => Ok(GeneratedField::ChannelPid),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetPinnedMessagesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.GetPinnedMessagesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetPinnedMessagesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut channel_pid__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ChannelPid => {
                            if channel_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelPid"));
                            }
                            channel_pid__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetPinnedMessagesRequest {
                    channel_pid: channel_pid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.GetPinnedMessagesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetPinnedMessagesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.messages.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.GetPinnedMessagesResponse", len)?;
        if !self.messages.is_empty() {
            struct_ser.serialize_field("messages", &self.messages)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetPinnedMessagesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "messages",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Messages,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "messages" => Ok(GeneratedField::Messages),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetPinnedMessagesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.GetPinnedMessagesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetPinnedMessagesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut messages__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Messages => {
                            if messages__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messages"));
                            }
                            messages__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetPinnedMessagesResponse {
                    messages: messages__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.GetPinnedMessagesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetReportsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.channel_pid.is_some() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        if self.reason.is_some() {
            len += 1;
        }
        if self.limit.is_some() {
            len += 1;
        }
        if self.cursor.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.GetReportsRequest", len)?;
        if let Some(v) = self.channel_pid.as_ref() {
            struct_ser.serialize_field("channelPid", v)?;
        }
        if let Some(v) = self.status.as_ref() {
            let v = ReportStatus::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        if let Some(v) = self.reason.as_ref() {
            let v = ReportReason::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("reason", &v)?;
        }
        if let Some(v) = self.limit.as_ref() {
            struct_ser.serialize_field("limit", v)?;
        }
        if let Some(v) = self.cursor.as_ref() {
            struct_ser.serialize_field("cursor", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetReportsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "channel_pid",
            "channelPid",
            "status",
            "reason",
            "limit",
            "cursor",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChannelPid,
            Status,
            Reason,
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
                            "channelPid" | "channel_pid" => Ok(GeneratedField::ChannelPid),
                            "status" => Ok(GeneratedField::Status),
                            "reason" => Ok(GeneratedField::Reason),
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
            type Value = GetReportsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.GetReportsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetReportsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut channel_pid__ = None;
                let mut status__ = None;
                let mut reason__ = None;
                let mut limit__ = None;
                let mut cursor__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ChannelPid => {
                            if channel_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelPid"));
                            }
                            channel_pid__ = map_.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map_.next_value::<::std::option::Option<ReportStatus>>()?.map(|x| x as i32);
                        }
                        GeneratedField::Reason => {
                            if reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            reason__ = map_.next_value::<::std::option::Option<ReportReason>>()?.map(|x| x as i32);
                        }
                        GeneratedField::Limit => {
                            if limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("limit"));
                            }
                            limit__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
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
                Ok(GetReportsRequest {
                    channel_pid: channel_pid__,
                    status: status__,
                    reason: reason__,
                    limit: limit__,
                    cursor: cursor__,
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.GetReportsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetReportsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.reports.is_empty() {
            len += 1;
        }
        if self.next_cursor.is_some() {
            len += 1;
        }
        if self.total_pending != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.GetReportsResponse", len)?;
        if !self.reports.is_empty() {
            struct_ser.serialize_field("reports", &self.reports)?;
        }
        if let Some(v) = self.next_cursor.as_ref() {
            struct_ser.serialize_field("nextCursor", v)?;
        }
        if self.total_pending != 0 {
            struct_ser.serialize_field("totalPending", &self.total_pending)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetReportsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "reports",
            "next_cursor",
            "nextCursor",
            "total_pending",
            "totalPending",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Reports,
            NextCursor,
            TotalPending,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "reports" => Ok(GeneratedField::Reports),
                            "nextCursor" | "next_cursor" => Ok(GeneratedField::NextCursor),
                            "totalPending" | "total_pending" => Ok(GeneratedField::TotalPending),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetReportsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.GetReportsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetReportsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut reports__ = None;
                let mut next_cursor__ = None;
                let mut total_pending__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Reports => {
                            if reports__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reports"));
                            }
                            reports__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextCursor => {
                            if next_cursor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextCursor"));
                            }
                            next_cursor__ = map_.next_value()?;
                        }
                        GeneratedField::TotalPending => {
                            if total_pending__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalPending"));
                            }
                            total_pending__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(GetReportsResponse {
                    reports: reports__.unwrap_or_default(),
                    next_cursor: next_cursor__,
                    total_pending: total_pending__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.GetReportsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetThreadRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.parent_message_pid.is_empty() {
            len += 1;
        }
        if self.limit.is_some() {
            len += 1;
        }
        if self.before_timestamp.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.GetThreadRequest", len)?;
        if !self.parent_message_pid.is_empty() {
            struct_ser.serialize_field("parentMessagePid", &self.parent_message_pid)?;
        }
        if let Some(v) = self.limit.as_ref() {
            struct_ser.serialize_field("limit", v)?;
        }
        if let Some(v) = self.before_timestamp.as_ref() {
            struct_ser.serialize_field("beforeTimestamp", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetThreadRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "parent_message_pid",
            "parentMessagePid",
            "limit",
            "before_timestamp",
            "beforeTimestamp",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ParentMessagePid,
            Limit,
            BeforeTimestamp,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "parentMessagePid" | "parent_message_pid" => Ok(GeneratedField::ParentMessagePid),
                            "limit" => Ok(GeneratedField::Limit),
                            "beforeTimestamp" | "before_timestamp" => Ok(GeneratedField::BeforeTimestamp),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetThreadRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.GetThreadRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetThreadRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut parent_message_pid__ = None;
                let mut limit__ = None;
                let mut before_timestamp__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ParentMessagePid => {
                            if parent_message_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parentMessagePid"));
                            }
                            parent_message_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Limit => {
                            if limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("limit"));
                            }
                            limit__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::BeforeTimestamp => {
                            if before_timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("beforeTimestamp"));
                            }
                            before_timestamp__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetThreadRequest {
                    parent_message_pid: parent_message_pid__.unwrap_or_default(),
                    limit: limit__,
                    before_timestamp: before_timestamp__,
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.GetThreadRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetThreadResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.parent.is_some() {
            len += 1;
        }
        if !self.replies.is_empty() {
            len += 1;
        }
        if self.has_more {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.GetThreadResponse", len)?;
        if let Some(v) = self.parent.as_ref() {
            struct_ser.serialize_field("parent", v)?;
        }
        if !self.replies.is_empty() {
            struct_ser.serialize_field("replies", &self.replies)?;
        }
        if self.has_more {
            struct_ser.serialize_field("hasMore", &self.has_more)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetThreadResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "parent",
            "replies",
            "has_more",
            "hasMore",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Parent,
            Replies,
            HasMore,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "parent" => Ok(GeneratedField::Parent),
                            "replies" => Ok(GeneratedField::Replies),
                            "hasMore" | "has_more" => Ok(GeneratedField::HasMore),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetThreadResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.GetThreadResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetThreadResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut parent__ = None;
                let mut replies__ = None;
                let mut has_more__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Parent => {
                            if parent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parent"));
                            }
                            parent__ = map_.next_value()?;
                        }
                        GeneratedField::Replies => {
                            if replies__.is_some() {
                                return Err(serde::de::Error::duplicate_field("replies"));
                            }
                            replies__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HasMore => {
                            if has_more__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hasMore"));
                            }
                            has_more__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetThreadResponse {
                    parent: parent__,
                    replies: replies__.unwrap_or_default(),
                    has_more: has_more__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.GetThreadResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetUnreadCountsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.channel_pids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.GetUnreadCountsRequest", len)?;
        if !self.channel_pids.is_empty() {
            struct_ser.serialize_field("channelPids", &self.channel_pids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetUnreadCountsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "channel_pids",
            "channelPids",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChannelPids,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "channelPids" | "channel_pids" => Ok(GeneratedField::ChannelPids),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetUnreadCountsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.GetUnreadCountsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetUnreadCountsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut channel_pids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ChannelPids => {
                            if channel_pids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelPids"));
                            }
                            channel_pids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetUnreadCountsRequest {
                    channel_pids: channel_pids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.GetUnreadCountsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetUnreadCountsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.channels.is_empty() {
            len += 1;
        }
        if self.total_unread != 0 {
            len += 1;
        }
        if self.total_mentions != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.GetUnreadCountsResponse", len)?;
        if !self.channels.is_empty() {
            struct_ser.serialize_field("channels", &self.channels)?;
        }
        if self.total_unread != 0 {
            struct_ser.serialize_field("totalUnread", &self.total_unread)?;
        }
        if self.total_mentions != 0 {
            struct_ser.serialize_field("totalMentions", &self.total_mentions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetUnreadCountsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "channels",
            "total_unread",
            "totalUnread",
            "total_mentions",
            "totalMentions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Channels,
            TotalUnread,
            TotalMentions,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "channels" => Ok(GeneratedField::Channels),
                            "totalUnread" | "total_unread" => Ok(GeneratedField::TotalUnread),
                            "totalMentions" | "total_mentions" => Ok(GeneratedField::TotalMentions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetUnreadCountsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.GetUnreadCountsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetUnreadCountsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut channels__ = None;
                let mut total_unread__ = None;
                let mut total_mentions__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Channels => {
                            if channels__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channels"));
                            }
                            channels__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::TotalUnread => {
                            if total_unread__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalUnread"));
                            }
                            total_unread__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TotalMentions => {
                            if total_mentions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalMentions"));
                            }
                            total_mentions__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(GetUnreadCountsResponse {
                    channels: channels__.unwrap_or_default(),
                    total_unread: total_unread__.unwrap_or_default(),
                    total_mentions: total_mentions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.GetUnreadCountsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InviteResult {
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
        if self.success {
            len += 1;
        }
        if self.error.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.InviteResult", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if self.success {
            struct_ser.serialize_field("success", &self.success)?;
        }
        if let Some(v) = self.error.as_ref() {
            struct_ser.serialize_field("error", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InviteResult {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
            "success",
            "error",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
            Success,
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
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
                            "success" => Ok(GeneratedField::Success),
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
            type Value = InviteResult;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.InviteResult")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<InviteResult, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut success__ = None;
                let mut error__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Success => {
                            if success__.is_some() {
                                return Err(serde::de::Error::duplicate_field("success"));
                            }
                            success__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = map_.next_value()?;
                        }
                    }
                }
                Ok(InviteResult {
                    user_id: user_id__.unwrap_or_default(),
                    success: success__.unwrap_or_default(),
                    error: error__,
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.InviteResult", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InviteToChannelRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.channel_pid.is_empty() {
            len += 1;
        }
        if !self.user_ids.is_empty() {
            len += 1;
        }
        if self.invite_message.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.InviteToChannelRequest", len)?;
        if !self.channel_pid.is_empty() {
            struct_ser.serialize_field("channelPid", &self.channel_pid)?;
        }
        if !self.user_ids.is_empty() {
            struct_ser.serialize_field("userIds", &self.user_ids)?;
        }
        if let Some(v) = self.invite_message.as_ref() {
            struct_ser.serialize_field("inviteMessage", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InviteToChannelRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "channel_pid",
            "channelPid",
            "user_ids",
            "userIds",
            "invite_message",
            "inviteMessage",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChannelPid,
            UserIds,
            InviteMessage,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "channelPid" | "channel_pid" => Ok(GeneratedField::ChannelPid),
                            "userIds" | "user_ids" => Ok(GeneratedField::UserIds),
                            "inviteMessage" | "invite_message" => Ok(GeneratedField::InviteMessage),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InviteToChannelRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.InviteToChannelRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<InviteToChannelRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut channel_pid__ = None;
                let mut user_ids__ = None;
                let mut invite_message__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ChannelPid => {
                            if channel_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelPid"));
                            }
                            channel_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserIds => {
                            if user_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userIds"));
                            }
                            user_ids__ = Some(map_.next_value()?);
                        }
                        GeneratedField::InviteMessage => {
                            if invite_message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inviteMessage"));
                            }
                            invite_message__ = map_.next_value()?;
                        }
                    }
                }
                Ok(InviteToChannelRequest {
                    channel_pid: channel_pid__.unwrap_or_default(),
                    user_ids: user_ids__.unwrap_or_default(),
                    invite_message: invite_message__,
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.InviteToChannelRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InviteToChannelResponse {
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
        let mut struct_ser = serializer.serialize_struct("chat.v1.InviteToChannelResponse", len)?;
        if !self.results.is_empty() {
            struct_ser.serialize_field("results", &self.results)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InviteToChannelResponse {
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
            type Value = InviteToChannelResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.InviteToChannelResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<InviteToChannelResponse, V::Error>
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
                Ok(InviteToChannelResponse {
                    results: results__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.InviteToChannelResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for KickFromChannelRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.channel_pid.is_empty() {
            len += 1;
        }
        if !self.user_id.is_empty() {
            len += 1;
        }
        if self.reason.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.KickFromChannelRequest", len)?;
        if !self.channel_pid.is_empty() {
            struct_ser.serialize_field("channelPid", &self.channel_pid)?;
        }
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if let Some(v) = self.reason.as_ref() {
            struct_ser.serialize_field("reason", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for KickFromChannelRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "channel_pid",
            "channelPid",
            "user_id",
            "userId",
            "reason",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChannelPid,
            UserId,
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
                            "channelPid" | "channel_pid" => Ok(GeneratedField::ChannelPid),
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
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
            type Value = KickFromChannelRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.KickFromChannelRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<KickFromChannelRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut channel_pid__ = None;
                let mut user_id__ = None;
                let mut reason__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ChannelPid => {
                            if channel_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelPid"));
                            }
                            channel_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Reason => {
                            if reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            reason__ = map_.next_value()?;
                        }
                    }
                }
                Ok(KickFromChannelRequest {
                    channel_pid: channel_pid__.unwrap_or_default(),
                    user_id: user_id__.unwrap_or_default(),
                    reason: reason__,
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.KickFromChannelRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for KickFromChannelResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("chat.v1.KickFromChannelResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for KickFromChannelResponse {
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
            type Value = KickFromChannelResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.KickFromChannelResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<KickFromChannelResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(KickFromChannelResponse {
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.KickFromChannelResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LeaveChannelRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.channel_pid.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.LeaveChannelRequest", len)?;
        if !self.channel_pid.is_empty() {
            struct_ser.serialize_field("channelPid", &self.channel_pid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LeaveChannelRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "channel_pid",
            "channelPid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChannelPid,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "channelPid" | "channel_pid" => Ok(GeneratedField::ChannelPid),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LeaveChannelRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.LeaveChannelRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LeaveChannelRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut channel_pid__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ChannelPid => {
                            if channel_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelPid"));
                            }
                            channel_pid__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(LeaveChannelRequest {
                    channel_pid: channel_pid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.LeaveChannelRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LeaveChannelResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("chat.v1.LeaveChannelResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LeaveChannelResponse {
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
            type Value = LeaveChannelResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.LeaveChannelResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LeaveChannelResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(LeaveChannelResponse {
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.LeaveChannelResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MarkAsReadRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.channel_pid.is_empty() {
            len += 1;
        }
        if self.last_read_message_pid.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.MarkAsReadRequest", len)?;
        if !self.channel_pid.is_empty() {
            struct_ser.serialize_field("channelPid", &self.channel_pid)?;
        }
        if let Some(v) = self.last_read_message_pid.as_ref() {
            struct_ser.serialize_field("lastReadMessagePid", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MarkAsReadRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "channel_pid",
            "channelPid",
            "last_read_message_pid",
            "lastReadMessagePid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChannelPid,
            LastReadMessagePid,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "channelPid" | "channel_pid" => Ok(GeneratedField::ChannelPid),
                            "lastReadMessagePid" | "last_read_message_pid" => Ok(GeneratedField::LastReadMessagePid),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MarkAsReadRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.MarkAsReadRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MarkAsReadRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut channel_pid__ = None;
                let mut last_read_message_pid__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ChannelPid => {
                            if channel_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelPid"));
                            }
                            channel_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LastReadMessagePid => {
                            if last_read_message_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastReadMessagePid"));
                            }
                            last_read_message_pid__ = map_.next_value()?;
                        }
                    }
                }
                Ok(MarkAsReadRequest {
                    channel_pid: channel_pid__.unwrap_or_default(),
                    last_read_message_pid: last_read_message_pid__,
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.MarkAsReadRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MarkAsReadResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.last_read_at.is_empty() {
            len += 1;
        }
        if self.remaining_unread != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.MarkAsReadResponse", len)?;
        if !self.last_read_at.is_empty() {
            struct_ser.serialize_field("lastReadAt", &self.last_read_at)?;
        }
        if self.remaining_unread != 0 {
            struct_ser.serialize_field("remainingUnread", &self.remaining_unread)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MarkAsReadResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "last_read_at",
            "lastReadAt",
            "remaining_unread",
            "remainingUnread",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LastReadAt,
            RemainingUnread,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "lastReadAt" | "last_read_at" => Ok(GeneratedField::LastReadAt),
                            "remainingUnread" | "remaining_unread" => Ok(GeneratedField::RemainingUnread),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MarkAsReadResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.MarkAsReadResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MarkAsReadResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut last_read_at__ = None;
                let mut remaining_unread__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LastReadAt => {
                            if last_read_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastReadAt"));
                            }
                            last_read_at__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RemainingUnread => {
                            if remaining_unread__.is_some() {
                                return Err(serde::de::Error::duplicate_field("remainingUnread"));
                            }
                            remaining_unread__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(MarkAsReadResponse {
                    last_read_at: last_read_at__.unwrap_or_default(),
                    remaining_unread: remaining_unread__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.MarkAsReadResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Mention {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.mention_type != 0 {
            len += 1;
        }
        if !self.target_id.is_empty() {
            len += 1;
        }
        if !self.display_text.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.Mention", len)?;
        if self.mention_type != 0 {
            let v = MentionType::try_from(self.mention_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.mention_type)))?;
            struct_ser.serialize_field("mentionType", &v)?;
        }
        if !self.target_id.is_empty() {
            struct_ser.serialize_field("targetId", &self.target_id)?;
        }
        if !self.display_text.is_empty() {
            struct_ser.serialize_field("displayText", &self.display_text)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Mention {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "mention_type",
            "mentionType",
            "target_id",
            "targetId",
            "display_text",
            "displayText",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MentionType,
            TargetId,
            DisplayText,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "mentionType" | "mention_type" => Ok(GeneratedField::MentionType),
                            "targetId" | "target_id" => Ok(GeneratedField::TargetId),
                            "displayText" | "display_text" => Ok(GeneratedField::DisplayText),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Mention;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.Mention")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Mention, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut mention_type__ = None;
                let mut target_id__ = None;
                let mut display_text__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MentionType => {
                            if mention_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mentionType"));
                            }
                            mention_type__ = Some(map_.next_value::<MentionType>()? as i32);
                        }
                        GeneratedField::TargetId => {
                            if target_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetId"));
                            }
                            target_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DisplayText => {
                            if display_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayText"));
                            }
                            display_text__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Mention {
                    mention_type: mention_type__.unwrap_or_default(),
                    target_id: target_id__.unwrap_or_default(),
                    display_text: display_text__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.Mention", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MentionType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "MENTION_TYPE_UNSPECIFIED",
            Self::User => "MENTION_TYPE_USER",
            Self::Channel => "MENTION_TYPE_CHANNEL",
            Self::Role => "MENTION_TYPE_ROLE",
            Self::Everyone => "MENTION_TYPE_EVERYONE",
            Self::Here => "MENTION_TYPE_HERE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for MentionType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "MENTION_TYPE_UNSPECIFIED",
            "MENTION_TYPE_USER",
            "MENTION_TYPE_CHANNEL",
            "MENTION_TYPE_ROLE",
            "MENTION_TYPE_EVERYONE",
            "MENTION_TYPE_HERE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MentionType;

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
                    "MENTION_TYPE_UNSPECIFIED" => Ok(MentionType::Unspecified),
                    "MENTION_TYPE_USER" => Ok(MentionType::User),
                    "MENTION_TYPE_CHANNEL" => Ok(MentionType::Channel),
                    "MENTION_TYPE_ROLE" => Ok(MentionType::Role),
                    "MENTION_TYPE_EVERYONE" => Ok(MentionType::Everyone),
                    "MENTION_TYPE_HERE" => Ok(MentionType::Here),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for MessageDeleted {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.message_pid.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.MessageDeleted", len)?;
        if !self.message_pid.is_empty() {
            struct_ser.serialize_field("messagePid", &self.message_pid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MessageDeleted {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "message_pid",
            "messagePid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = MessageDeleted;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.MessageDeleted")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MessageDeleted, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut message_pid__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MessagePid => {
                            if message_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messagePid"));
                            }
                            message_pid__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MessageDeleted {
                    message_pid: message_pid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.MessageDeleted", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MessageEdited {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.message_pid.is_empty() {
            len += 1;
        }
        if !self.new_content.is_empty() {
            len += 1;
        }
        if !self.edited_at.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.MessageEdited", len)?;
        if !self.message_pid.is_empty() {
            struct_ser.serialize_field("messagePid", &self.message_pid)?;
        }
        if !self.new_content.is_empty() {
            struct_ser.serialize_field("newContent", &self.new_content)?;
        }
        if !self.edited_at.is_empty() {
            struct_ser.serialize_field("editedAt", &self.edited_at)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MessageEdited {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "message_pid",
            "messagePid",
            "new_content",
            "newContent",
            "edited_at",
            "editedAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MessagePid,
            NewContent,
            EditedAt,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "messagePid" | "message_pid" => Ok(GeneratedField::MessagePid),
                            "newContent" | "new_content" => Ok(GeneratedField::NewContent),
                            "editedAt" | "edited_at" => Ok(GeneratedField::EditedAt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MessageEdited;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.MessageEdited")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MessageEdited, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut message_pid__ = None;
                let mut new_content__ = None;
                let mut edited_at__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MessagePid => {
                            if message_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messagePid"));
                            }
                            message_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NewContent => {
                            if new_content__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newContent"));
                            }
                            new_content__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EditedAt => {
                            if edited_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("editedAt"));
                            }
                            edited_at__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MessageEdited {
                    message_pid: message_pid__.unwrap_or_default(),
                    new_content: new_content__.unwrap_or_default(),
                    edited_at: edited_at__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.MessageEdited", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MessagePinned {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.message_pid.is_empty() {
            len += 1;
        }
        if !self.pinned_by_user_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.MessagePinned", len)?;
        if !self.message_pid.is_empty() {
            struct_ser.serialize_field("messagePid", &self.message_pid)?;
        }
        if !self.pinned_by_user_id.is_empty() {
            struct_ser.serialize_field("pinnedByUserId", &self.pinned_by_user_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MessagePinned {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "message_pid",
            "messagePid",
            "pinned_by_user_id",
            "pinnedByUserId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MessagePid,
            PinnedByUserId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "messagePid" | "message_pid" => Ok(GeneratedField::MessagePid),
                            "pinnedByUserId" | "pinned_by_user_id" => Ok(GeneratedField::PinnedByUserId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MessagePinned;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.MessagePinned")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MessagePinned, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut message_pid__ = None;
                let mut pinned_by_user_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MessagePid => {
                            if message_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messagePid"));
                            }
                            message_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PinnedByUserId => {
                            if pinned_by_user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pinnedByUserId"));
                            }
                            pinned_by_user_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MessagePinned {
                    message_pid: message_pid__.unwrap_or_default(),
                    pinned_by_user_id: pinned_by_user_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.MessagePinned", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MessageUnpinned {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.message_pid.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.MessageUnpinned", len)?;
        if !self.message_pid.is_empty() {
            struct_ser.serialize_field("messagePid", &self.message_pid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MessageUnpinned {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "message_pid",
            "messagePid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = MessageUnpinned;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.MessageUnpinned")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MessageUnpinned, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut message_pid__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MessagePid => {
                            if message_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messagePid"));
                            }
                            message_pid__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MessageUnpinned {
                    message_pid: message_pid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.MessageUnpinned", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ModerationAction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.action.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.ModerationAction", len)?;
        if let Some(v) = self.action.as_ref() {
            match v {
                moderation_action::Action::DeleteMessage(v) => {
                    struct_ser.serialize_field("deleteMessage", v)?;
                }
                moderation_action::Action::MuteUser(v) => {
                    struct_ser.serialize_field("muteUser", v)?;
                }
                moderation_action::Action::BanUser(v) => {
                    struct_ser.serialize_field("banUser", v)?;
                }
                moderation_action::Action::WarnUser(v) => {
                    struct_ser.serialize_field("warnUser", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ModerationAction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "delete_message",
            "deleteMessage",
            "mute_user",
            "muteUser",
            "ban_user",
            "banUser",
            "warn_user",
            "warnUser",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DeleteMessage,
            MuteUser,
            BanUser,
            WarnUser,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "deleteMessage" | "delete_message" => Ok(GeneratedField::DeleteMessage),
                            "muteUser" | "mute_user" => Ok(GeneratedField::MuteUser),
                            "banUser" | "ban_user" => Ok(GeneratedField::BanUser),
                            "warnUser" | "warn_user" => Ok(GeneratedField::WarnUser),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ModerationAction;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.ModerationAction")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ModerationAction, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut action__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DeleteMessage => {
                            if action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deleteMessage"));
                            }
                            action__ = map_.next_value::<::std::option::Option<_>>()?.map(moderation_action::Action::DeleteMessage)
;
                        }
                        GeneratedField::MuteUser => {
                            if action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("muteUser"));
                            }
                            action__ = map_.next_value::<::std::option::Option<_>>()?.map(moderation_action::Action::MuteUser)
;
                        }
                        GeneratedField::BanUser => {
                            if action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("banUser"));
                            }
                            action__ = map_.next_value::<::std::option::Option<_>>()?.map(moderation_action::Action::BanUser)
;
                        }
                        GeneratedField::WarnUser => {
                            if action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("warnUser"));
                            }
                            action__ = map_.next_value::<::std::option::Option<_>>()?.map(moderation_action::Action::WarnUser)
;
                        }
                    }
                }
                Ok(ModerationAction {
                    action: action__,
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.ModerationAction", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ModerationActionType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "MODERATION_ACTION_TYPE_UNSPECIFIED",
            Self::Mute => "MODERATION_ACTION_TYPE_MUTE",
            Self::Unmute => "MODERATION_ACTION_TYPE_UNMUTE",
            Self::Ban => "MODERATION_ACTION_TYPE_BAN",
            Self::Unban => "MODERATION_ACTION_TYPE_UNBAN",
            Self::Kick => "MODERATION_ACTION_TYPE_KICK",
            Self::Warn => "MODERATION_ACTION_TYPE_WARN",
            Self::DeleteMessage => "MODERATION_ACTION_TYPE_DELETE_MESSAGE",
            Self::RoleChange => "MODERATION_ACTION_TYPE_ROLE_CHANGE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ModerationActionType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "MODERATION_ACTION_TYPE_UNSPECIFIED",
            "MODERATION_ACTION_TYPE_MUTE",
            "MODERATION_ACTION_TYPE_UNMUTE",
            "MODERATION_ACTION_TYPE_BAN",
            "MODERATION_ACTION_TYPE_UNBAN",
            "MODERATION_ACTION_TYPE_KICK",
            "MODERATION_ACTION_TYPE_WARN",
            "MODERATION_ACTION_TYPE_DELETE_MESSAGE",
            "MODERATION_ACTION_TYPE_ROLE_CHANGE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ModerationActionType;

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
                    "MODERATION_ACTION_TYPE_UNSPECIFIED" => Ok(ModerationActionType::Unspecified),
                    "MODERATION_ACTION_TYPE_MUTE" => Ok(ModerationActionType::Mute),
                    "MODERATION_ACTION_TYPE_UNMUTE" => Ok(ModerationActionType::Unmute),
                    "MODERATION_ACTION_TYPE_BAN" => Ok(ModerationActionType::Ban),
                    "MODERATION_ACTION_TYPE_UNBAN" => Ok(ModerationActionType::Unban),
                    "MODERATION_ACTION_TYPE_KICK" => Ok(ModerationActionType::Kick),
                    "MODERATION_ACTION_TYPE_WARN" => Ok(ModerationActionType::Warn),
                    "MODERATION_ACTION_TYPE_DELETE_MESSAGE" => Ok(ModerationActionType::DeleteMessage),
                    "MODERATION_ACTION_TYPE_ROLE_CHANGE" => Ok(ModerationActionType::RoleChange),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ModerationLogEntry {
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
        if !self.channel_pid.is_empty() {
            len += 1;
        }
        if !self.moderator_id.is_empty() {
            len += 1;
        }
        if !self.moderator_name.is_empty() {
            len += 1;
        }
        if !self.target_user_id.is_empty() {
            len += 1;
        }
        if !self.target_user_name.is_empty() {
            len += 1;
        }
        if self.action_type != 0 {
            len += 1;
        }
        if self.reason.is_some() {
            len += 1;
        }
        if !self.metadata.is_empty() {
            len += 1;
        }
        if !self.created_at.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.ModerationLogEntry", len)?;
        if !self.pid.is_empty() {
            struct_ser.serialize_field("pid", &self.pid)?;
        }
        if !self.channel_pid.is_empty() {
            struct_ser.serialize_field("channelPid", &self.channel_pid)?;
        }
        if !self.moderator_id.is_empty() {
            struct_ser.serialize_field("moderatorId", &self.moderator_id)?;
        }
        if !self.moderator_name.is_empty() {
            struct_ser.serialize_field("moderatorName", &self.moderator_name)?;
        }
        if !self.target_user_id.is_empty() {
            struct_ser.serialize_field("targetUserId", &self.target_user_id)?;
        }
        if !self.target_user_name.is_empty() {
            struct_ser.serialize_field("targetUserName", &self.target_user_name)?;
        }
        if self.action_type != 0 {
            let v = ModerationActionType::try_from(self.action_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.action_type)))?;
            struct_ser.serialize_field("actionType", &v)?;
        }
        if let Some(v) = self.reason.as_ref() {
            struct_ser.serialize_field("reason", v)?;
        }
        if !self.metadata.is_empty() {
            struct_ser.serialize_field("metadata", &self.metadata)?;
        }
        if !self.created_at.is_empty() {
            struct_ser.serialize_field("createdAt", &self.created_at)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ModerationLogEntry {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pid",
            "channel_pid",
            "channelPid",
            "moderator_id",
            "moderatorId",
            "moderator_name",
            "moderatorName",
            "target_user_id",
            "targetUserId",
            "target_user_name",
            "targetUserName",
            "action_type",
            "actionType",
            "reason",
            "metadata",
            "created_at",
            "createdAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pid,
            ChannelPid,
            ModeratorId,
            ModeratorName,
            TargetUserId,
            TargetUserName,
            ActionType,
            Reason,
            Metadata,
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
                            "channelPid" | "channel_pid" => Ok(GeneratedField::ChannelPid),
                            "moderatorId" | "moderator_id" => Ok(GeneratedField::ModeratorId),
                            "moderatorName" | "moderator_name" => Ok(GeneratedField::ModeratorName),
                            "targetUserId" | "target_user_id" => Ok(GeneratedField::TargetUserId),
                            "targetUserName" | "target_user_name" => Ok(GeneratedField::TargetUserName),
                            "actionType" | "action_type" => Ok(GeneratedField::ActionType),
                            "reason" => Ok(GeneratedField::Reason),
                            "metadata" => Ok(GeneratedField::Metadata),
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
            type Value = ModerationLogEntry;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.ModerationLogEntry")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ModerationLogEntry, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pid__ = None;
                let mut channel_pid__ = None;
                let mut moderator_id__ = None;
                let mut moderator_name__ = None;
                let mut target_user_id__ = None;
                let mut target_user_name__ = None;
                let mut action_type__ = None;
                let mut reason__ = None;
                let mut metadata__ = None;
                let mut created_at__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Pid => {
                            if pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pid"));
                            }
                            pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ChannelPid => {
                            if channel_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelPid"));
                            }
                            channel_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ModeratorId => {
                            if moderator_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("moderatorId"));
                            }
                            moderator_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ModeratorName => {
                            if moderator_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("moderatorName"));
                            }
                            moderator_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TargetUserId => {
                            if target_user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetUserId"));
                            }
                            target_user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TargetUserName => {
                            if target_user_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetUserName"));
                            }
                            target_user_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ActionType => {
                            if action_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actionType"));
                            }
                            action_type__ = Some(map_.next_value::<ModerationActionType>()? as i32);
                        }
                        GeneratedField::Reason => {
                            if reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            reason__ = map_.next_value()?;
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdAt"));
                            }
                            created_at__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ModerationLogEntry {
                    pid: pid__.unwrap_or_default(),
                    channel_pid: channel_pid__.unwrap_or_default(),
                    moderator_id: moderator_id__.unwrap_or_default(),
                    moderator_name: moderator_name__.unwrap_or_default(),
                    target_user_id: target_user_id__.unwrap_or_default(),
                    target_user_name: target_user_name__.unwrap_or_default(),
                    action_type: action_type__.unwrap_or_default(),
                    reason: reason__,
                    metadata: metadata__.unwrap_or_default(),
                    created_at: created_at__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.ModerationLogEntry", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MuteUserAction {
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
        if self.duration_seconds.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.MuteUserAction", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if let Some(v) = self.duration_seconds.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("durationSeconds", ToString::to_string(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MuteUserAction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
            "duration_seconds",
            "durationSeconds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
            DurationSeconds,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "durationSeconds" | "duration_seconds" => Ok(GeneratedField::DurationSeconds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MuteUserAction;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.MuteUserAction")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MuteUserAction, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut duration_seconds__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DurationSeconds => {
                            if duration_seconds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("durationSeconds"));
                            }
                            duration_seconds__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(MuteUserAction {
                    user_id: user_id__.unwrap_or_default(),
                    duration_seconds: duration_seconds__,
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.MuteUserAction", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MuteUserRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.channel_pid.is_empty() {
            len += 1;
        }
        if !self.user_id.is_empty() {
            len += 1;
        }
        if self.duration_seconds.is_some() {
            len += 1;
        }
        if self.reason.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.MuteUserRequest", len)?;
        if !self.channel_pid.is_empty() {
            struct_ser.serialize_field("channelPid", &self.channel_pid)?;
        }
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if let Some(v) = self.duration_seconds.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("durationSeconds", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.reason.as_ref() {
            struct_ser.serialize_field("reason", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MuteUserRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "channel_pid",
            "channelPid",
            "user_id",
            "userId",
            "duration_seconds",
            "durationSeconds",
            "reason",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChannelPid,
            UserId,
            DurationSeconds,
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
                            "channelPid" | "channel_pid" => Ok(GeneratedField::ChannelPid),
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
                            "durationSeconds" | "duration_seconds" => Ok(GeneratedField::DurationSeconds),
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
            type Value = MuteUserRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.MuteUserRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MuteUserRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut channel_pid__ = None;
                let mut user_id__ = None;
                let mut duration_seconds__ = None;
                let mut reason__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ChannelPid => {
                            if channel_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelPid"));
                            }
                            channel_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DurationSeconds => {
                            if duration_seconds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("durationSeconds"));
                            }
                            duration_seconds__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Reason => {
                            if reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            reason__ = map_.next_value()?;
                        }
                    }
                }
                Ok(MuteUserRequest {
                    channel_pid: channel_pid__.unwrap_or_default(),
                    user_id: user_id__.unwrap_or_default(),
                    duration_seconds: duration_seconds__,
                    reason: reason__,
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.MuteUserRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MuteUserResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.muted_until.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.MuteUserResponse", len)?;
        if !self.muted_until.is_empty() {
            struct_ser.serialize_field("mutedUntil", &self.muted_until)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MuteUserResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "muted_until",
            "mutedUntil",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MutedUntil,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "mutedUntil" | "muted_until" => Ok(GeneratedField::MutedUntil),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MuteUserResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.MuteUserResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MuteUserResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut muted_until__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MutedUntil => {
                            if muted_until__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mutedUntil"));
                            }
                            muted_until__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MuteUserResponse {
                    muted_until: muted_until__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.MuteUserResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MutedUser {
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
        if !self.user_name.is_empty() {
            len += 1;
        }
        if self.avatar_url.is_some() {
            len += 1;
        }
        if !self.muted_by_id.is_empty() {
            len += 1;
        }
        if self.reason.is_some() {
            len += 1;
        }
        if !self.muted_at.is_empty() {
            len += 1;
        }
        if !self.muted_until.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.MutedUser", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if !self.user_name.is_empty() {
            struct_ser.serialize_field("userName", &self.user_name)?;
        }
        if let Some(v) = self.avatar_url.as_ref() {
            struct_ser.serialize_field("avatarUrl", v)?;
        }
        if !self.muted_by_id.is_empty() {
            struct_ser.serialize_field("mutedById", &self.muted_by_id)?;
        }
        if let Some(v) = self.reason.as_ref() {
            struct_ser.serialize_field("reason", v)?;
        }
        if !self.muted_at.is_empty() {
            struct_ser.serialize_field("mutedAt", &self.muted_at)?;
        }
        if !self.muted_until.is_empty() {
            struct_ser.serialize_field("mutedUntil", &self.muted_until)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MutedUser {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
            "user_name",
            "userName",
            "avatar_url",
            "avatarUrl",
            "muted_by_id",
            "mutedById",
            "reason",
            "muted_at",
            "mutedAt",
            "muted_until",
            "mutedUntil",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
            UserName,
            AvatarUrl,
            MutedById,
            Reason,
            MutedAt,
            MutedUntil,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "userName" | "user_name" => Ok(GeneratedField::UserName),
                            "avatarUrl" | "avatar_url" => Ok(GeneratedField::AvatarUrl),
                            "mutedById" | "muted_by_id" => Ok(GeneratedField::MutedById),
                            "reason" => Ok(GeneratedField::Reason),
                            "mutedAt" | "muted_at" => Ok(GeneratedField::MutedAt),
                            "mutedUntil" | "muted_until" => Ok(GeneratedField::MutedUntil),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MutedUser;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.MutedUser")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MutedUser, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut user_name__ = None;
                let mut avatar_url__ = None;
                let mut muted_by_id__ = None;
                let mut reason__ = None;
                let mut muted_at__ = None;
                let mut muted_until__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserName => {
                            if user_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userName"));
                            }
                            user_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AvatarUrl => {
                            if avatar_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("avatarUrl"));
                            }
                            avatar_url__ = map_.next_value()?;
                        }
                        GeneratedField::MutedById => {
                            if muted_by_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mutedById"));
                            }
                            muted_by_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Reason => {
                            if reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            reason__ = map_.next_value()?;
                        }
                        GeneratedField::MutedAt => {
                            if muted_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mutedAt"));
                            }
                            muted_at__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MutedUntil => {
                            if muted_until__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mutedUntil"));
                            }
                            muted_until__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MutedUser {
                    user_id: user_id__.unwrap_or_default(),
                    user_name: user_name__.unwrap_or_default(),
                    avatar_url: avatar_url__,
                    muted_by_id: muted_by_id__.unwrap_or_default(),
                    reason: reason__,
                    muted_at: muted_at__.unwrap_or_default(),
                    muted_until: muted_until__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.MutedUser", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ParticipantInfo {
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
        if !self.user_name.is_empty() {
            len += 1;
        }
        if self.avatar_url.is_some() {
            len += 1;
        }
        if self.presence != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.ParticipantInfo", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if !self.user_name.is_empty() {
            struct_ser.serialize_field("userName", &self.user_name)?;
        }
        if let Some(v) = self.avatar_url.as_ref() {
            struct_ser.serialize_field("avatarUrl", v)?;
        }
        if self.presence != 0 {
            let v = PresenceStatus::try_from(self.presence)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.presence)))?;
            struct_ser.serialize_field("presence", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ParticipantInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
            "user_name",
            "userName",
            "avatar_url",
            "avatarUrl",
            "presence",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
            UserName,
            AvatarUrl,
            Presence,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "userName" | "user_name" => Ok(GeneratedField::UserName),
                            "avatarUrl" | "avatar_url" => Ok(GeneratedField::AvatarUrl),
                            "presence" => Ok(GeneratedField::Presence),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ParticipantInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.ParticipantInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ParticipantInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut user_name__ = None;
                let mut avatar_url__ = None;
                let mut presence__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserName => {
                            if user_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userName"));
                            }
                            user_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AvatarUrl => {
                            if avatar_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("avatarUrl"));
                            }
                            avatar_url__ = map_.next_value()?;
                        }
                        GeneratedField::Presence => {
                            if presence__.is_some() {
                                return Err(serde::de::Error::duplicate_field("presence"));
                            }
                            presence__ = Some(map_.next_value::<PresenceStatus>()? as i32);
                        }
                    }
                }
                Ok(ParticipantInfo {
                    user_id: user_id__.unwrap_or_default(),
                    user_name: user_name__.unwrap_or_default(),
                    avatar_url: avatar_url__,
                    presence: presence__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.ParticipantInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PinMessageRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.message_pid.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.PinMessageRequest", len)?;
        if !self.message_pid.is_empty() {
            struct_ser.serialize_field("messagePid", &self.message_pid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PinMessageRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "message_pid",
            "messagePid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = PinMessageRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.PinMessageRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PinMessageRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut message_pid__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MessagePid => {
                            if message_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messagePid"));
                            }
                            message_pid__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PinMessageRequest {
                    message_pid: message_pid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.PinMessageRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PinMessageResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("chat.v1.PinMessageResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PinMessageResponse {
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
            type Value = PinMessageResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.PinMessageResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PinMessageResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(PinMessageResponse {
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.PinMessageResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PresenceStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "PRESENCE_STATUS_UNSPECIFIED",
            Self::Online => "PRESENCE_STATUS_ONLINE",
            Self::Away => "PRESENCE_STATUS_AWAY",
            Self::Dnd => "PRESENCE_STATUS_DND",
            Self::Offline => "PRESENCE_STATUS_OFFLINE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for PresenceStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "PRESENCE_STATUS_UNSPECIFIED",
            "PRESENCE_STATUS_ONLINE",
            "PRESENCE_STATUS_AWAY",
            "PRESENCE_STATUS_DND",
            "PRESENCE_STATUS_OFFLINE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PresenceStatus;

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
                    "PRESENCE_STATUS_UNSPECIFIED" => Ok(PresenceStatus::Unspecified),
                    "PRESENCE_STATUS_ONLINE" => Ok(PresenceStatus::Online),
                    "PRESENCE_STATUS_AWAY" => Ok(PresenceStatus::Away),
                    "PRESENCE_STATUS_DND" => Ok(PresenceStatus::Dnd),
                    "PRESENCE_STATUS_OFFLINE" => Ok(PresenceStatus::Offline),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for PresenceUpdate {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.status != 0 {
            len += 1;
        }
        if self.status_text.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.PresenceUpdate", len)?;
        if self.status != 0 {
            let v = PresenceStatus::try_from(self.status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        if let Some(v) = self.status_text.as_ref() {
            struct_ser.serialize_field("statusText", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PresenceUpdate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "status",
            "status_text",
            "statusText",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Status,
            StatusText,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "status" => Ok(GeneratedField::Status),
                            "statusText" | "status_text" => Ok(GeneratedField::StatusText),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PresenceUpdate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.PresenceUpdate")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PresenceUpdate, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut status__ = None;
                let mut status_text__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value::<PresenceStatus>()? as i32);
                        }
                        GeneratedField::StatusText => {
                            if status_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statusText"));
                            }
                            status_text__ = map_.next_value()?;
                        }
                    }
                }
                Ok(PresenceUpdate {
                    status: status__.unwrap_or_default(),
                    status_text: status_text__,
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.PresenceUpdate", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ProfanityFilterLevel {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "PROFANITY_FILTER_LEVEL_UNSPECIFIED",
            Self::Low => "PROFANITY_FILTER_LEVEL_LOW",
            Self::Medium => "PROFANITY_FILTER_LEVEL_MEDIUM",
            Self::High => "PROFANITY_FILTER_LEVEL_HIGH",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ProfanityFilterLevel {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "PROFANITY_FILTER_LEVEL_UNSPECIFIED",
            "PROFANITY_FILTER_LEVEL_LOW",
            "PROFANITY_FILTER_LEVEL_MEDIUM",
            "PROFANITY_FILTER_LEVEL_HIGH",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProfanityFilterLevel;

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
                    "PROFANITY_FILTER_LEVEL_UNSPECIFIED" => Ok(ProfanityFilterLevel::Unspecified),
                    "PROFANITY_FILTER_LEVEL_LOW" => Ok(ProfanityFilterLevel::Low),
                    "PROFANITY_FILTER_LEVEL_MEDIUM" => Ok(ProfanityFilterLevel::Medium),
                    "PROFANITY_FILTER_LEVEL_HIGH" => Ok(ProfanityFilterLevel::High),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ReactToMessageRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.message_pid.is_empty() {
            len += 1;
        }
        if !self.emoji.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.ReactToMessageRequest", len)?;
        if !self.message_pid.is_empty() {
            struct_ser.serialize_field("messagePid", &self.message_pid)?;
        }
        if !self.emoji.is_empty() {
            struct_ser.serialize_field("emoji", &self.emoji)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReactToMessageRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "message_pid",
            "messagePid",
            "emoji",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MessagePid,
            Emoji,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "messagePid" | "message_pid" => Ok(GeneratedField::MessagePid),
                            "emoji" => Ok(GeneratedField::Emoji),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReactToMessageRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.ReactToMessageRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReactToMessageRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut message_pid__ = None;
                let mut emoji__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MessagePid => {
                            if message_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messagePid"));
                            }
                            message_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Emoji => {
                            if emoji__.is_some() {
                                return Err(serde::de::Error::duplicate_field("emoji"));
                            }
                            emoji__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ReactToMessageRequest {
                    message_pid: message_pid__.unwrap_or_default(),
                    emoji: emoji__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.ReactToMessageRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReactToMessageResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("chat.v1.ReactToMessageResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReactToMessageResponse {
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
            type Value = ReactToMessageResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.ReactToMessageResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReactToMessageResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ReactToMessageResponse {
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.ReactToMessageResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Reaction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.emoji.is_empty() {
            len += 1;
        }
        if self.count != 0 {
            len += 1;
        }
        if !self.user_ids.is_empty() {
            len += 1;
        }
        if self.includes_me {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.Reaction", len)?;
        if !self.emoji.is_empty() {
            struct_ser.serialize_field("emoji", &self.emoji)?;
        }
        if self.count != 0 {
            struct_ser.serialize_field("count", &self.count)?;
        }
        if !self.user_ids.is_empty() {
            struct_ser.serialize_field("userIds", &self.user_ids)?;
        }
        if self.includes_me {
            struct_ser.serialize_field("includesMe", &self.includes_me)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Reaction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "emoji",
            "count",
            "user_ids",
            "userIds",
            "includes_me",
            "includesMe",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Emoji,
            Count,
            UserIds,
            IncludesMe,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "emoji" => Ok(GeneratedField::Emoji),
                            "count" => Ok(GeneratedField::Count),
                            "userIds" | "user_ids" => Ok(GeneratedField::UserIds),
                            "includesMe" | "includes_me" => Ok(GeneratedField::IncludesMe),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Reaction;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.Reaction")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Reaction, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut emoji__ = None;
                let mut count__ = None;
                let mut user_ids__ = None;
                let mut includes_me__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Emoji => {
                            if emoji__.is_some() {
                                return Err(serde::de::Error::duplicate_field("emoji"));
                            }
                            emoji__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Count => {
                            if count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("count"));
                            }
                            count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::UserIds => {
                            if user_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userIds"));
                            }
                            user_ids__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IncludesMe => {
                            if includes_me__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includesMe"));
                            }
                            includes_me__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Reaction {
                    emoji: emoji__.unwrap_or_default(),
                    count: count__.unwrap_or_default(),
                    user_ids: user_ids__.unwrap_or_default(),
                    includes_me: includes_me__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.Reaction", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReactionAdded {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.message_pid.is_empty() {
            len += 1;
        }
        if !self.user_id.is_empty() {
            len += 1;
        }
        if !self.emoji.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.ReactionAdded", len)?;
        if !self.message_pid.is_empty() {
            struct_ser.serialize_field("messagePid", &self.message_pid)?;
        }
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if !self.emoji.is_empty() {
            struct_ser.serialize_field("emoji", &self.emoji)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReactionAdded {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "message_pid",
            "messagePid",
            "user_id",
            "userId",
            "emoji",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MessagePid,
            UserId,
            Emoji,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "messagePid" | "message_pid" => Ok(GeneratedField::MessagePid),
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
                            "emoji" => Ok(GeneratedField::Emoji),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReactionAdded;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.ReactionAdded")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReactionAdded, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut message_pid__ = None;
                let mut user_id__ = None;
                let mut emoji__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MessagePid => {
                            if message_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messagePid"));
                            }
                            message_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Emoji => {
                            if emoji__.is_some() {
                                return Err(serde::de::Error::duplicate_field("emoji"));
                            }
                            emoji__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ReactionAdded {
                    message_pid: message_pid__.unwrap_or_default(),
                    user_id: user_id__.unwrap_or_default(),
                    emoji: emoji__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.ReactionAdded", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReactionRemoved {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.message_pid.is_empty() {
            len += 1;
        }
        if !self.user_id.is_empty() {
            len += 1;
        }
        if !self.emoji.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.ReactionRemoved", len)?;
        if !self.message_pid.is_empty() {
            struct_ser.serialize_field("messagePid", &self.message_pid)?;
        }
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if !self.emoji.is_empty() {
            struct_ser.serialize_field("emoji", &self.emoji)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReactionRemoved {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "message_pid",
            "messagePid",
            "user_id",
            "userId",
            "emoji",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MessagePid,
            UserId,
            Emoji,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "messagePid" | "message_pid" => Ok(GeneratedField::MessagePid),
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
                            "emoji" => Ok(GeneratedField::Emoji),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReactionRemoved;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.ReactionRemoved")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReactionRemoved, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut message_pid__ = None;
                let mut user_id__ = None;
                let mut emoji__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MessagePid => {
                            if message_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messagePid"));
                            }
                            message_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Emoji => {
                            if emoji__.is_some() {
                                return Err(serde::de::Error::duplicate_field("emoji"));
                            }
                            emoji__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ReactionRemoved {
                    message_pid: message_pid__.unwrap_or_default(),
                    user_id: user_id__.unwrap_or_default(),
                    emoji: emoji__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.ReactionRemoved", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemoveReactionRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.message_pid.is_empty() {
            len += 1;
        }
        if !self.emoji.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.RemoveReactionRequest", len)?;
        if !self.message_pid.is_empty() {
            struct_ser.serialize_field("messagePid", &self.message_pid)?;
        }
        if !self.emoji.is_empty() {
            struct_ser.serialize_field("emoji", &self.emoji)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoveReactionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "message_pid",
            "messagePid",
            "emoji",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MessagePid,
            Emoji,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "messagePid" | "message_pid" => Ok(GeneratedField::MessagePid),
                            "emoji" => Ok(GeneratedField::Emoji),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RemoveReactionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.RemoveReactionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemoveReactionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut message_pid__ = None;
                let mut emoji__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MessagePid => {
                            if message_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messagePid"));
                            }
                            message_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Emoji => {
                            if emoji__.is_some() {
                                return Err(serde::de::Error::duplicate_field("emoji"));
                            }
                            emoji__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RemoveReactionRequest {
                    message_pid: message_pid__.unwrap_or_default(),
                    emoji: emoji__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.RemoveReactionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemoveReactionResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("chat.v1.RemoveReactionResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoveReactionResponse {
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
            type Value = RemoveReactionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.RemoveReactionResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemoveReactionResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(RemoveReactionResponse {
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.RemoveReactionResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Report {
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
        if self.report_type != 0 {
            len += 1;
        }
        if !self.reporter_id.is_empty() {
            len += 1;
        }
        if !self.reporter_name.is_empty() {
            len += 1;
        }
        if self.target_message_pid.is_some() {
            len += 1;
        }
        if self.target_user_id.is_some() {
            len += 1;
        }
        if !self.channel_pid.is_empty() {
            len += 1;
        }
        if self.reason != 0 {
            len += 1;
        }
        if self.details.is_some() {
            len += 1;
        }
        if self.status != 0 {
            len += 1;
        }
        if !self.created_at.is_empty() {
            len += 1;
        }
        if self.resolved_at.is_some() {
            len += 1;
        }
        if self.resolved_by_id.is_some() {
            len += 1;
        }
        if self.resolution_notes.is_some() {
            len += 1;
        }
        if self.resolution.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.Report", len)?;
        if !self.pid.is_empty() {
            struct_ser.serialize_field("pid", &self.pid)?;
        }
        if self.report_type != 0 {
            let v = ReportType::try_from(self.report_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.report_type)))?;
            struct_ser.serialize_field("reportType", &v)?;
        }
        if !self.reporter_id.is_empty() {
            struct_ser.serialize_field("reporterId", &self.reporter_id)?;
        }
        if !self.reporter_name.is_empty() {
            struct_ser.serialize_field("reporterName", &self.reporter_name)?;
        }
        if let Some(v) = self.target_message_pid.as_ref() {
            struct_ser.serialize_field("targetMessagePid", v)?;
        }
        if let Some(v) = self.target_user_id.as_ref() {
            struct_ser.serialize_field("targetUserId", v)?;
        }
        if !self.channel_pid.is_empty() {
            struct_ser.serialize_field("channelPid", &self.channel_pid)?;
        }
        if self.reason != 0 {
            let v = ReportReason::try_from(self.reason)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.reason)))?;
            struct_ser.serialize_field("reason", &v)?;
        }
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if self.status != 0 {
            let v = ReportStatus::try_from(self.status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        if !self.created_at.is_empty() {
            struct_ser.serialize_field("createdAt", &self.created_at)?;
        }
        if let Some(v) = self.resolved_at.as_ref() {
            struct_ser.serialize_field("resolvedAt", v)?;
        }
        if let Some(v) = self.resolved_by_id.as_ref() {
            struct_ser.serialize_field("resolvedById", v)?;
        }
        if let Some(v) = self.resolution_notes.as_ref() {
            struct_ser.serialize_field("resolutionNotes", v)?;
        }
        if let Some(v) = self.resolution.as_ref() {
            let v = ReportResolution::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("resolution", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Report {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pid",
            "report_type",
            "reportType",
            "reporter_id",
            "reporterId",
            "reporter_name",
            "reporterName",
            "target_message_pid",
            "targetMessagePid",
            "target_user_id",
            "targetUserId",
            "channel_pid",
            "channelPid",
            "reason",
            "details",
            "status",
            "created_at",
            "createdAt",
            "resolved_at",
            "resolvedAt",
            "resolved_by_id",
            "resolvedById",
            "resolution_notes",
            "resolutionNotes",
            "resolution",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pid,
            ReportType,
            ReporterId,
            ReporterName,
            TargetMessagePid,
            TargetUserId,
            ChannelPid,
            Reason,
            Details,
            Status,
            CreatedAt,
            ResolvedAt,
            ResolvedById,
            ResolutionNotes,
            Resolution,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "reportType" | "report_type" => Ok(GeneratedField::ReportType),
                            "reporterId" | "reporter_id" => Ok(GeneratedField::ReporterId),
                            "reporterName" | "reporter_name" => Ok(GeneratedField::ReporterName),
                            "targetMessagePid" | "target_message_pid" => Ok(GeneratedField::TargetMessagePid),
                            "targetUserId" | "target_user_id" => Ok(GeneratedField::TargetUserId),
                            "channelPid" | "channel_pid" => Ok(GeneratedField::ChannelPid),
                            "reason" => Ok(GeneratedField::Reason),
                            "details" => Ok(GeneratedField::Details),
                            "status" => Ok(GeneratedField::Status),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "resolvedAt" | "resolved_at" => Ok(GeneratedField::ResolvedAt),
                            "resolvedById" | "resolved_by_id" => Ok(GeneratedField::ResolvedById),
                            "resolutionNotes" | "resolution_notes" => Ok(GeneratedField::ResolutionNotes),
                            "resolution" => Ok(GeneratedField::Resolution),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Report;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.Report")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Report, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pid__ = None;
                let mut report_type__ = None;
                let mut reporter_id__ = None;
                let mut reporter_name__ = None;
                let mut target_message_pid__ = None;
                let mut target_user_id__ = None;
                let mut channel_pid__ = None;
                let mut reason__ = None;
                let mut details__ = None;
                let mut status__ = None;
                let mut created_at__ = None;
                let mut resolved_at__ = None;
                let mut resolved_by_id__ = None;
                let mut resolution_notes__ = None;
                let mut resolution__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Pid => {
                            if pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pid"));
                            }
                            pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ReportType => {
                            if report_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reportType"));
                            }
                            report_type__ = Some(map_.next_value::<ReportType>()? as i32);
                        }
                        GeneratedField::ReporterId => {
                            if reporter_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reporterId"));
                            }
                            reporter_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ReporterName => {
                            if reporter_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reporterName"));
                            }
                            reporter_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TargetMessagePid => {
                            if target_message_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetMessagePid"));
                            }
                            target_message_pid__ = map_.next_value()?;
                        }
                        GeneratedField::TargetUserId => {
                            if target_user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetUserId"));
                            }
                            target_user_id__ = map_.next_value()?;
                        }
                        GeneratedField::ChannelPid => {
                            if channel_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelPid"));
                            }
                            channel_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Reason => {
                            if reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            reason__ = Some(map_.next_value::<ReportReason>()? as i32);
                        }
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value::<ReportStatus>()? as i32);
                        }
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdAt"));
                            }
                            created_at__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResolvedAt => {
                            if resolved_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resolvedAt"));
                            }
                            resolved_at__ = map_.next_value()?;
                        }
                        GeneratedField::ResolvedById => {
                            if resolved_by_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resolvedById"));
                            }
                            resolved_by_id__ = map_.next_value()?;
                        }
                        GeneratedField::ResolutionNotes => {
                            if resolution_notes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resolutionNotes"));
                            }
                            resolution_notes__ = map_.next_value()?;
                        }
                        GeneratedField::Resolution => {
                            if resolution__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resolution"));
                            }
                            resolution__ = map_.next_value::<::std::option::Option<ReportResolution>>()?.map(|x| x as i32);
                        }
                    }
                }
                Ok(Report {
                    pid: pid__.unwrap_or_default(),
                    report_type: report_type__.unwrap_or_default(),
                    reporter_id: reporter_id__.unwrap_or_default(),
                    reporter_name: reporter_name__.unwrap_or_default(),
                    target_message_pid: target_message_pid__,
                    target_user_id: target_user_id__,
                    channel_pid: channel_pid__.unwrap_or_default(),
                    reason: reason__.unwrap_or_default(),
                    details: details__,
                    status: status__.unwrap_or_default(),
                    created_at: created_at__.unwrap_or_default(),
                    resolved_at: resolved_at__,
                    resolved_by_id: resolved_by_id__,
                    resolution_notes: resolution_notes__,
                    resolution: resolution__,
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.Report", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReportMessageRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.message_pid.is_empty() {
            len += 1;
        }
        if self.reason != 0 {
            len += 1;
        }
        if self.details.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.ReportMessageRequest", len)?;
        if !self.message_pid.is_empty() {
            struct_ser.serialize_field("messagePid", &self.message_pid)?;
        }
        if self.reason != 0 {
            let v = ReportReason::try_from(self.reason)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.reason)))?;
            struct_ser.serialize_field("reason", &v)?;
        }
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReportMessageRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "message_pid",
            "messagePid",
            "reason",
            "details",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MessagePid,
            Reason,
            Details,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "messagePid" | "message_pid" => Ok(GeneratedField::MessagePid),
                            "reason" => Ok(GeneratedField::Reason),
                            "details" => Ok(GeneratedField::Details),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReportMessageRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.ReportMessageRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReportMessageRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut message_pid__ = None;
                let mut reason__ = None;
                let mut details__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MessagePid => {
                            if message_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messagePid"));
                            }
                            message_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Reason => {
                            if reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            reason__ = Some(map_.next_value::<ReportReason>()? as i32);
                        }
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ReportMessageRequest {
                    message_pid: message_pid__.unwrap_or_default(),
                    reason: reason__.unwrap_or_default(),
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.ReportMessageRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReportMessageResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.report_pid.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.ReportMessageResponse", len)?;
        if !self.report_pid.is_empty() {
            struct_ser.serialize_field("reportPid", &self.report_pid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReportMessageResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "report_pid",
            "reportPid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ReportPid,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "reportPid" | "report_pid" => Ok(GeneratedField::ReportPid),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReportMessageResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.ReportMessageResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReportMessageResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut report_pid__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ReportPid => {
                            if report_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reportPid"));
                            }
                            report_pid__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ReportMessageResponse {
                    report_pid: report_pid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.ReportMessageResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReportReason {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "REPORT_REASON_UNSPECIFIED",
            Self::Spam => "REPORT_REASON_SPAM",
            Self::Harassment => "REPORT_REASON_HARASSMENT",
            Self::HateSpeech => "REPORT_REASON_HATE_SPEECH",
            Self::Violence => "REPORT_REASON_VIOLENCE",
            Self::Nsfw => "REPORT_REASON_NSFW",
            Self::Misinformation => "REPORT_REASON_MISINFORMATION",
            Self::Other => "REPORT_REASON_OTHER",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ReportReason {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "REPORT_REASON_UNSPECIFIED",
            "REPORT_REASON_SPAM",
            "REPORT_REASON_HARASSMENT",
            "REPORT_REASON_HATE_SPEECH",
            "REPORT_REASON_VIOLENCE",
            "REPORT_REASON_NSFW",
            "REPORT_REASON_MISINFORMATION",
            "REPORT_REASON_OTHER",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReportReason;

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
                    "REPORT_REASON_UNSPECIFIED" => Ok(ReportReason::Unspecified),
                    "REPORT_REASON_SPAM" => Ok(ReportReason::Spam),
                    "REPORT_REASON_HARASSMENT" => Ok(ReportReason::Harassment),
                    "REPORT_REASON_HATE_SPEECH" => Ok(ReportReason::HateSpeech),
                    "REPORT_REASON_VIOLENCE" => Ok(ReportReason::Violence),
                    "REPORT_REASON_NSFW" => Ok(ReportReason::Nsfw),
                    "REPORT_REASON_MISINFORMATION" => Ok(ReportReason::Misinformation),
                    "REPORT_REASON_OTHER" => Ok(ReportReason::Other),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ReportResolution {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "REPORT_RESOLUTION_UNSPECIFIED",
            Self::NoAction => "REPORT_RESOLUTION_NO_ACTION",
            Self::WarningIssued => "REPORT_RESOLUTION_WARNING_ISSUED",
            Self::ContentRemoved => "REPORT_RESOLUTION_CONTENT_REMOVED",
            Self::UserMuted => "REPORT_RESOLUTION_USER_MUTED",
            Self::UserBanned => "REPORT_RESOLUTION_USER_BANNED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ReportResolution {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "REPORT_RESOLUTION_UNSPECIFIED",
            "REPORT_RESOLUTION_NO_ACTION",
            "REPORT_RESOLUTION_WARNING_ISSUED",
            "REPORT_RESOLUTION_CONTENT_REMOVED",
            "REPORT_RESOLUTION_USER_MUTED",
            "REPORT_RESOLUTION_USER_BANNED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReportResolution;

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
                    "REPORT_RESOLUTION_UNSPECIFIED" => Ok(ReportResolution::Unspecified),
                    "REPORT_RESOLUTION_NO_ACTION" => Ok(ReportResolution::NoAction),
                    "REPORT_RESOLUTION_WARNING_ISSUED" => Ok(ReportResolution::WarningIssued),
                    "REPORT_RESOLUTION_CONTENT_REMOVED" => Ok(ReportResolution::ContentRemoved),
                    "REPORT_RESOLUTION_USER_MUTED" => Ok(ReportResolution::UserMuted),
                    "REPORT_RESOLUTION_USER_BANNED" => Ok(ReportResolution::UserBanned),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ReportStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "REPORT_STATUS_UNSPECIFIED",
            Self::Pending => "REPORT_STATUS_PENDING",
            Self::Reviewing => "REPORT_STATUS_REVIEWING",
            Self::Resolved => "REPORT_STATUS_RESOLVED",
            Self::Dismissed => "REPORT_STATUS_DISMISSED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ReportStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "REPORT_STATUS_UNSPECIFIED",
            "REPORT_STATUS_PENDING",
            "REPORT_STATUS_REVIEWING",
            "REPORT_STATUS_RESOLVED",
            "REPORT_STATUS_DISMISSED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReportStatus;

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
                    "REPORT_STATUS_UNSPECIFIED" => Ok(ReportStatus::Unspecified),
                    "REPORT_STATUS_PENDING" => Ok(ReportStatus::Pending),
                    "REPORT_STATUS_REVIEWING" => Ok(ReportStatus::Reviewing),
                    "REPORT_STATUS_RESOLVED" => Ok(ReportStatus::Resolved),
                    "REPORT_STATUS_DISMISSED" => Ok(ReportStatus::Dismissed),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ReportType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "REPORT_TYPE_UNSPECIFIED",
            Self::Message => "REPORT_TYPE_MESSAGE",
            Self::User => "REPORT_TYPE_USER",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ReportType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "REPORT_TYPE_UNSPECIFIED",
            "REPORT_TYPE_MESSAGE",
            "REPORT_TYPE_USER",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReportType;

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
                    "REPORT_TYPE_UNSPECIFIED" => Ok(ReportType::Unspecified),
                    "REPORT_TYPE_MESSAGE" => Ok(ReportType::Message),
                    "REPORT_TYPE_USER" => Ok(ReportType::User),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ReportUserRequest {
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
        if !self.channel_pid.is_empty() {
            len += 1;
        }
        if self.reason != 0 {
            len += 1;
        }
        if self.details.is_some() {
            len += 1;
        }
        if !self.evidence_message_pids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.ReportUserRequest", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if !self.channel_pid.is_empty() {
            struct_ser.serialize_field("channelPid", &self.channel_pid)?;
        }
        if self.reason != 0 {
            let v = ReportReason::try_from(self.reason)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.reason)))?;
            struct_ser.serialize_field("reason", &v)?;
        }
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if !self.evidence_message_pids.is_empty() {
            struct_ser.serialize_field("evidenceMessagePids", &self.evidence_message_pids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReportUserRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
            "channel_pid",
            "channelPid",
            "reason",
            "details",
            "evidence_message_pids",
            "evidenceMessagePids",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
            ChannelPid,
            Reason,
            Details,
            EvidenceMessagePids,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "channelPid" | "channel_pid" => Ok(GeneratedField::ChannelPid),
                            "reason" => Ok(GeneratedField::Reason),
                            "details" => Ok(GeneratedField::Details),
                            "evidenceMessagePids" | "evidence_message_pids" => Ok(GeneratedField::EvidenceMessagePids),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReportUserRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.ReportUserRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReportUserRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut channel_pid__ = None;
                let mut reason__ = None;
                let mut details__ = None;
                let mut evidence_message_pids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ChannelPid => {
                            if channel_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelPid"));
                            }
                            channel_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Reason => {
                            if reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            reason__ = Some(map_.next_value::<ReportReason>()? as i32);
                        }
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::EvidenceMessagePids => {
                            if evidence_message_pids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("evidenceMessagePids"));
                            }
                            evidence_message_pids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ReportUserRequest {
                    user_id: user_id__.unwrap_or_default(),
                    channel_pid: channel_pid__.unwrap_or_default(),
                    reason: reason__.unwrap_or_default(),
                    details: details__,
                    evidence_message_pids: evidence_message_pids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.ReportUserRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReportUserResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.report_pid.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.ReportUserResponse", len)?;
        if !self.report_pid.is_empty() {
            struct_ser.serialize_field("reportPid", &self.report_pid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReportUserResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "report_pid",
            "reportPid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ReportPid,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "reportPid" | "report_pid" => Ok(GeneratedField::ReportPid),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReportUserResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.ReportUserResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReportUserResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut report_pid__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ReportPid => {
                            if report_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reportPid"));
                            }
                            report_pid__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ReportUserResponse {
                    report_pid: report_pid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.ReportUserResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResolveReportRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.report_pid.is_empty() {
            len += 1;
        }
        if self.resolution != 0 {
            len += 1;
        }
        if self.notes.is_some() {
            len += 1;
        }
        if self.action.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.ResolveReportRequest", len)?;
        if !self.report_pid.is_empty() {
            struct_ser.serialize_field("reportPid", &self.report_pid)?;
        }
        if self.resolution != 0 {
            let v = ReportResolution::try_from(self.resolution)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.resolution)))?;
            struct_ser.serialize_field("resolution", &v)?;
        }
        if let Some(v) = self.notes.as_ref() {
            struct_ser.serialize_field("notes", v)?;
        }
        if let Some(v) = self.action.as_ref() {
            struct_ser.serialize_field("action", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResolveReportRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "report_pid",
            "reportPid",
            "resolution",
            "notes",
            "action",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ReportPid,
            Resolution,
            Notes,
            Action,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "reportPid" | "report_pid" => Ok(GeneratedField::ReportPid),
                            "resolution" => Ok(GeneratedField::Resolution),
                            "notes" => Ok(GeneratedField::Notes),
                            "action" => Ok(GeneratedField::Action),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResolveReportRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.ResolveReportRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResolveReportRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut report_pid__ = None;
                let mut resolution__ = None;
                let mut notes__ = None;
                let mut action__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ReportPid => {
                            if report_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reportPid"));
                            }
                            report_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Resolution => {
                            if resolution__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resolution"));
                            }
                            resolution__ = Some(map_.next_value::<ReportResolution>()? as i32);
                        }
                        GeneratedField::Notes => {
                            if notes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notes"));
                            }
                            notes__ = map_.next_value()?;
                        }
                        GeneratedField::Action => {
                            if action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("action"));
                            }
                            action__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ResolveReportRequest {
                    report_pid: report_pid__.unwrap_or_default(),
                    resolution: resolution__.unwrap_or_default(),
                    notes: notes__,
                    action: action__,
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.ResolveReportRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResolveReportResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.report.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.ResolveReportResponse", len)?;
        if let Some(v) = self.report.as_ref() {
            struct_ser.serialize_field("report", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResolveReportResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "report",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Report,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "report" => Ok(GeneratedField::Report),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResolveReportResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.ResolveReportResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResolveReportResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut report__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Report => {
                            if report__.is_some() {
                                return Err(serde::de::Error::duplicate_field("report"));
                            }
                            report__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ResolveReportResponse {
                    report: report__,
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.ResolveReportResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SearchMessagesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.channel_pid.is_some() {
            len += 1;
        }
        if !self.query.is_empty() {
            len += 1;
        }
        if self.from_user_id.is_some() {
            len += 1;
        }
        if self.after_timestamp.is_some() {
            len += 1;
        }
        if self.before_timestamp.is_some() {
            len += 1;
        }
        if self.has_attachments.is_some() {
            len += 1;
        }
        if self.is_pinned.is_some() {
            len += 1;
        }
        if self.limit != 0 {
            len += 1;
        }
        if self.cursor.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.SearchMessagesRequest", len)?;
        if let Some(v) = self.channel_pid.as_ref() {
            struct_ser.serialize_field("channelPid", v)?;
        }
        if !self.query.is_empty() {
            struct_ser.serialize_field("query", &self.query)?;
        }
        if let Some(v) = self.from_user_id.as_ref() {
            struct_ser.serialize_field("fromUserId", v)?;
        }
        if let Some(v) = self.after_timestamp.as_ref() {
            struct_ser.serialize_field("afterTimestamp", v)?;
        }
        if let Some(v) = self.before_timestamp.as_ref() {
            struct_ser.serialize_field("beforeTimestamp", v)?;
        }
        if let Some(v) = self.has_attachments.as_ref() {
            struct_ser.serialize_field("hasAttachments", v)?;
        }
        if let Some(v) = self.is_pinned.as_ref() {
            struct_ser.serialize_field("isPinned", v)?;
        }
        if self.limit != 0 {
            struct_ser.serialize_field("limit", &self.limit)?;
        }
        if let Some(v) = self.cursor.as_ref() {
            struct_ser.serialize_field("cursor", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SearchMessagesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "channel_pid",
            "channelPid",
            "query",
            "from_user_id",
            "fromUserId",
            "after_timestamp",
            "afterTimestamp",
            "before_timestamp",
            "beforeTimestamp",
            "has_attachments",
            "hasAttachments",
            "is_pinned",
            "isPinned",
            "limit",
            "cursor",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChannelPid,
            Query,
            FromUserId,
            AfterTimestamp,
            BeforeTimestamp,
            HasAttachments,
            IsPinned,
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
                            "channelPid" | "channel_pid" => Ok(GeneratedField::ChannelPid),
                            "query" => Ok(GeneratedField::Query),
                            "fromUserId" | "from_user_id" => Ok(GeneratedField::FromUserId),
                            "afterTimestamp" | "after_timestamp" => Ok(GeneratedField::AfterTimestamp),
                            "beforeTimestamp" | "before_timestamp" => Ok(GeneratedField::BeforeTimestamp),
                            "hasAttachments" | "has_attachments" => Ok(GeneratedField::HasAttachments),
                            "isPinned" | "is_pinned" => Ok(GeneratedField::IsPinned),
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
            type Value = SearchMessagesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.SearchMessagesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SearchMessagesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut channel_pid__ = None;
                let mut query__ = None;
                let mut from_user_id__ = None;
                let mut after_timestamp__ = None;
                let mut before_timestamp__ = None;
                let mut has_attachments__ = None;
                let mut is_pinned__ = None;
                let mut limit__ = None;
                let mut cursor__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ChannelPid => {
                            if channel_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelPid"));
                            }
                            channel_pid__ = map_.next_value()?;
                        }
                        GeneratedField::Query => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            query__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FromUserId => {
                            if from_user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fromUserId"));
                            }
                            from_user_id__ = map_.next_value()?;
                        }
                        GeneratedField::AfterTimestamp => {
                            if after_timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("afterTimestamp"));
                            }
                            after_timestamp__ = map_.next_value()?;
                        }
                        GeneratedField::BeforeTimestamp => {
                            if before_timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("beforeTimestamp"));
                            }
                            before_timestamp__ = map_.next_value()?;
                        }
                        GeneratedField::HasAttachments => {
                            if has_attachments__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hasAttachments"));
                            }
                            has_attachments__ = map_.next_value()?;
                        }
                        GeneratedField::IsPinned => {
                            if is_pinned__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isPinned"));
                            }
                            is_pinned__ = map_.next_value()?;
                        }
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
                Ok(SearchMessagesRequest {
                    channel_pid: channel_pid__,
                    query: query__.unwrap_or_default(),
                    from_user_id: from_user_id__,
                    after_timestamp: after_timestamp__,
                    before_timestamp: before_timestamp__,
                    has_attachments: has_attachments__,
                    is_pinned: is_pinned__,
                    limit: limit__.unwrap_or_default(),
                    cursor: cursor__,
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.SearchMessagesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SearchMessagesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.messages.is_empty() {
            len += 1;
        }
        if self.total_count != 0 {
            len += 1;
        }
        if self.next_cursor.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.SearchMessagesResponse", len)?;
        if !self.messages.is_empty() {
            struct_ser.serialize_field("messages", &self.messages)?;
        }
        if self.total_count != 0 {
            struct_ser.serialize_field("totalCount", &self.total_count)?;
        }
        if let Some(v) = self.next_cursor.as_ref() {
            struct_ser.serialize_field("nextCursor", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SearchMessagesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "messages",
            "total_count",
            "totalCount",
            "next_cursor",
            "nextCursor",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Messages,
            TotalCount,
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
                            "messages" => Ok(GeneratedField::Messages),
                            "totalCount" | "total_count" => Ok(GeneratedField::TotalCount),
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
            type Value = SearchMessagesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.SearchMessagesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SearchMessagesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut messages__ = None;
                let mut total_count__ = None;
                let mut next_cursor__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Messages => {
                            if messages__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messages"));
                            }
                            messages__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TotalCount => {
                            if total_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalCount"));
                            }
                            total_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::NextCursor => {
                            if next_cursor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextCursor"));
                            }
                            next_cursor__ = map_.next_value()?;
                        }
                    }
                }
                Ok(SearchMessagesResponse {
                    messages: messages__.unwrap_or_default(),
                    total_count: total_count__.unwrap_or_default(),
                    next_cursor: next_cursor__,
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.SearchMessagesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SendMessageRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.channel_pid.is_empty() {
            len += 1;
        }
        if !self.content.is_empty() {
            len += 1;
        }
        if self.parent_pid.is_some() {
            len += 1;
        }
        if !self.attachment_pids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.SendMessageRequest", len)?;
        if !self.channel_pid.is_empty() {
            struct_ser.serialize_field("channelPid", &self.channel_pid)?;
        }
        if !self.content.is_empty() {
            struct_ser.serialize_field("content", &self.content)?;
        }
        if let Some(v) = self.parent_pid.as_ref() {
            struct_ser.serialize_field("parentPid", v)?;
        }
        if !self.attachment_pids.is_empty() {
            struct_ser.serialize_field("attachmentPids", &self.attachment_pids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SendMessageRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "channel_pid",
            "channelPid",
            "content",
            "parent_pid",
            "parentPid",
            "attachment_pids",
            "attachmentPids",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChannelPid,
            Content,
            ParentPid,
            AttachmentPids,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "channelPid" | "channel_pid" => Ok(GeneratedField::ChannelPid),
                            "content" => Ok(GeneratedField::Content),
                            "parentPid" | "parent_pid" => Ok(GeneratedField::ParentPid),
                            "attachmentPids" | "attachment_pids" => Ok(GeneratedField::AttachmentPids),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SendMessageRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.SendMessageRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SendMessageRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut channel_pid__ = None;
                let mut content__ = None;
                let mut parent_pid__ = None;
                let mut attachment_pids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ChannelPid => {
                            if channel_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelPid"));
                            }
                            channel_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Content => {
                            if content__.is_some() {
                                return Err(serde::de::Error::duplicate_field("content"));
                            }
                            content__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ParentPid => {
                            if parent_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parentPid"));
                            }
                            parent_pid__ = map_.next_value()?;
                        }
                        GeneratedField::AttachmentPids => {
                            if attachment_pids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attachmentPids"));
                            }
                            attachment_pids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SendMessageRequest {
                    channel_pid: channel_pid__.unwrap_or_default(),
                    content: content__.unwrap_or_default(),
                    parent_pid: parent_pid__,
                    attachment_pids: attachment_pids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.SendMessageRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SendMessageResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.message_pid.is_empty() {
            len += 1;
        }
        if !self.timestamp.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.SendMessageResponse", len)?;
        if !self.message_pid.is_empty() {
            struct_ser.serialize_field("messagePid", &self.message_pid)?;
        }
        if !self.timestamp.is_empty() {
            struct_ser.serialize_field("timestamp", &self.timestamp)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SendMessageResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "message_pid",
            "messagePid",
            "timestamp",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MessagePid,
            Timestamp,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "messagePid" | "message_pid" => Ok(GeneratedField::MessagePid),
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SendMessageResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.SendMessageResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SendMessageResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut message_pid__ = None;
                let mut timestamp__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MessagePid => {
                            if message_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messagePid"));
                            }
                            message_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SendMessageResponse {
                    message_pid: message_pid__.unwrap_or_default(),
                    timestamp: timestamp__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.SendMessageResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ServerEvent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.channel_pid.is_empty() {
            len += 1;
        }
        if self.event.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.ServerEvent", len)?;
        if !self.channel_pid.is_empty() {
            struct_ser.serialize_field("channelPid", &self.channel_pid)?;
        }
        if let Some(v) = self.event.as_ref() {
            match v {
                server_event::Event::Message(v) => {
                    struct_ser.serialize_field("message", v)?;
                }
                server_event::Event::MessageEdited(v) => {
                    struct_ser.serialize_field("messageEdited", v)?;
                }
                server_event::Event::MessageDeleted(v) => {
                    struct_ser.serialize_field("messageDeleted", v)?;
                }
                server_event::Event::ReactionAdded(v) => {
                    struct_ser.serialize_field("reactionAdded", v)?;
                }
                server_event::Event::ReactionRemoved(v) => {
                    struct_ser.serialize_field("reactionRemoved", v)?;
                }
                server_event::Event::UserTyping(v) => {
                    struct_ser.serialize_field("userTyping", v)?;
                }
                server_event::Event::UserPresence(v) => {
                    struct_ser.serialize_field("userPresence", v)?;
                }
                server_event::Event::UserJoined(v) => {
                    struct_ser.serialize_field("userJoined", v)?;
                }
                server_event::Event::UserLeft(v) => {
                    struct_ser.serialize_field("userLeft", v)?;
                }
                server_event::Event::ChannelUpdated(v) => {
                    struct_ser.serialize_field("channelUpdated", v)?;
                }
                server_event::Event::Notification(v) => {
                    struct_ser.serialize_field("notification", v)?;
                }
                server_event::Event::MessagePinned(v) => {
                    struct_ser.serialize_field("messagePinned", v)?;
                }
                server_event::Event::MessageUnpinned(v) => {
                    struct_ser.serialize_field("messageUnpinned", v)?;
                }
                server_event::Event::AgentStreaming(v) => {
                    struct_ser.serialize_field("agentStreaming", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ServerEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "channel_pid",
            "channelPid",
            "message",
            "message_edited",
            "messageEdited",
            "message_deleted",
            "messageDeleted",
            "reaction_added",
            "reactionAdded",
            "reaction_removed",
            "reactionRemoved",
            "user_typing",
            "userTyping",
            "user_presence",
            "userPresence",
            "user_joined",
            "userJoined",
            "user_left",
            "userLeft",
            "channel_updated",
            "channelUpdated",
            "notification",
            "message_pinned",
            "messagePinned",
            "message_unpinned",
            "messageUnpinned",
            "agent_streaming",
            "agentStreaming",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChannelPid,
            Message,
            MessageEdited,
            MessageDeleted,
            ReactionAdded,
            ReactionRemoved,
            UserTyping,
            UserPresence,
            UserJoined,
            UserLeft,
            ChannelUpdated,
            Notification,
            MessagePinned,
            MessageUnpinned,
            AgentStreaming,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "channelPid" | "channel_pid" => Ok(GeneratedField::ChannelPid),
                            "message" => Ok(GeneratedField::Message),
                            "messageEdited" | "message_edited" => Ok(GeneratedField::MessageEdited),
                            "messageDeleted" | "message_deleted" => Ok(GeneratedField::MessageDeleted),
                            "reactionAdded" | "reaction_added" => Ok(GeneratedField::ReactionAdded),
                            "reactionRemoved" | "reaction_removed" => Ok(GeneratedField::ReactionRemoved),
                            "userTyping" | "user_typing" => Ok(GeneratedField::UserTyping),
                            "userPresence" | "user_presence" => Ok(GeneratedField::UserPresence),
                            "userJoined" | "user_joined" => Ok(GeneratedField::UserJoined),
                            "userLeft" | "user_left" => Ok(GeneratedField::UserLeft),
                            "channelUpdated" | "channel_updated" => Ok(GeneratedField::ChannelUpdated),
                            "notification" => Ok(GeneratedField::Notification),
                            "messagePinned" | "message_pinned" => Ok(GeneratedField::MessagePinned),
                            "messageUnpinned" | "message_unpinned" => Ok(GeneratedField::MessageUnpinned),
                            "agentStreaming" | "agent_streaming" => Ok(GeneratedField::AgentStreaming),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ServerEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.ServerEvent")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ServerEvent, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut channel_pid__ = None;
                let mut event__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ChannelPid => {
                            if channel_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelPid"));
                            }
                            channel_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Message => {
                            if event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            event__ = map_.next_value::<::std::option::Option<_>>()?.map(server_event::Event::Message)
;
                        }
                        GeneratedField::MessageEdited => {
                            if event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messageEdited"));
                            }
                            event__ = map_.next_value::<::std::option::Option<_>>()?.map(server_event::Event::MessageEdited)
;
                        }
                        GeneratedField::MessageDeleted => {
                            if event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messageDeleted"));
                            }
                            event__ = map_.next_value::<::std::option::Option<_>>()?.map(server_event::Event::MessageDeleted)
;
                        }
                        GeneratedField::ReactionAdded => {
                            if event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reactionAdded"));
                            }
                            event__ = map_.next_value::<::std::option::Option<_>>()?.map(server_event::Event::ReactionAdded)
;
                        }
                        GeneratedField::ReactionRemoved => {
                            if event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reactionRemoved"));
                            }
                            event__ = map_.next_value::<::std::option::Option<_>>()?.map(server_event::Event::ReactionRemoved)
;
                        }
                        GeneratedField::UserTyping => {
                            if event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userTyping"));
                            }
                            event__ = map_.next_value::<::std::option::Option<_>>()?.map(server_event::Event::UserTyping)
;
                        }
                        GeneratedField::UserPresence => {
                            if event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userPresence"));
                            }
                            event__ = map_.next_value::<::std::option::Option<_>>()?.map(server_event::Event::UserPresence)
;
                        }
                        GeneratedField::UserJoined => {
                            if event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userJoined"));
                            }
                            event__ = map_.next_value::<::std::option::Option<_>>()?.map(server_event::Event::UserJoined)
;
                        }
                        GeneratedField::UserLeft => {
                            if event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userLeft"));
                            }
                            event__ = map_.next_value::<::std::option::Option<_>>()?.map(server_event::Event::UserLeft)
;
                        }
                        GeneratedField::ChannelUpdated => {
                            if event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelUpdated"));
                            }
                            event__ = map_.next_value::<::std::option::Option<_>>()?.map(server_event::Event::ChannelUpdated)
;
                        }
                        GeneratedField::Notification => {
                            if event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notification"));
                            }
                            event__ = map_.next_value::<::std::option::Option<_>>()?.map(server_event::Event::Notification)
;
                        }
                        GeneratedField::MessagePinned => {
                            if event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messagePinned"));
                            }
                            event__ = map_.next_value::<::std::option::Option<_>>()?.map(server_event::Event::MessagePinned)
;
                        }
                        GeneratedField::MessageUnpinned => {
                            if event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messageUnpinned"));
                            }
                            event__ = map_.next_value::<::std::option::Option<_>>()?.map(server_event::Event::MessageUnpinned)
;
                        }
                        GeneratedField::AgentStreaming => {
                            if event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("agentStreaming"));
                            }
                            event__ = map_.next_value::<::std::option::Option<_>>()?.map(server_event::Event::AgentStreaming)
;
                        }
                    }
                }
                Ok(ServerEvent {
                    channel_pid: channel_pid__.unwrap_or_default(),
                    event: event__,
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.ServerEvent", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StartConversationRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.participant_ids.is_empty() {
            len += 1;
        }
        if self.initial_message.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.StartConversationRequest", len)?;
        if !self.participant_ids.is_empty() {
            struct_ser.serialize_field("participantIds", &self.participant_ids)?;
        }
        if let Some(v) = self.initial_message.as_ref() {
            struct_ser.serialize_field("initialMessage", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StartConversationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "participant_ids",
            "participantIds",
            "initial_message",
            "initialMessage",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ParticipantIds,
            InitialMessage,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "participantIds" | "participant_ids" => Ok(GeneratedField::ParticipantIds),
                            "initialMessage" | "initial_message" => Ok(GeneratedField::InitialMessage),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StartConversationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.StartConversationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StartConversationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut participant_ids__ = None;
                let mut initial_message__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ParticipantIds => {
                            if participant_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("participantIds"));
                            }
                            participant_ids__ = Some(map_.next_value()?);
                        }
                        GeneratedField::InitialMessage => {
                            if initial_message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("initialMessage"));
                            }
                            initial_message__ = map_.next_value()?;
                        }
                    }
                }
                Ok(StartConversationRequest {
                    participant_ids: participant_ids__.unwrap_or_default(),
                    initial_message: initial_message__,
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.StartConversationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StartConversationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.channel.is_some() {
            len += 1;
        }
        if self.created {
            len += 1;
        }
        if self.initial_message_pid.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.StartConversationResponse", len)?;
        if let Some(v) = self.channel.as_ref() {
            struct_ser.serialize_field("channel", v)?;
        }
        if self.created {
            struct_ser.serialize_field("created", &self.created)?;
        }
        if let Some(v) = self.initial_message_pid.as_ref() {
            struct_ser.serialize_field("initialMessagePid", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StartConversationResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "channel",
            "created",
            "initial_message_pid",
            "initialMessagePid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Channel,
            Created,
            InitialMessagePid,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "channel" => Ok(GeneratedField::Channel),
                            "created" => Ok(GeneratedField::Created),
                            "initialMessagePid" | "initial_message_pid" => Ok(GeneratedField::InitialMessagePid),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StartConversationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.StartConversationResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StartConversationResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut channel__ = None;
                let mut created__ = None;
                let mut initial_message_pid__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Channel => {
                            if channel__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channel"));
                            }
                            channel__ = map_.next_value()?;
                        }
                        GeneratedField::Created => {
                            if created__.is_some() {
                                return Err(serde::de::Error::duplicate_field("created"));
                            }
                            created__ = Some(map_.next_value()?);
                        }
                        GeneratedField::InitialMessagePid => {
                            if initial_message_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("initialMessagePid"));
                            }
                            initial_message_pid__ = map_.next_value()?;
                        }
                    }
                }
                Ok(StartConversationResponse {
                    channel: channel__,
                    created: created__.unwrap_or_default(),
                    initial_message_pid: initial_message_pid__,
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.StartConversationResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SubscribeChannel {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.channel_pid.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.SubscribeChannel", len)?;
        if !self.channel_pid.is_empty() {
            struct_ser.serialize_field("channelPid", &self.channel_pid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SubscribeChannel {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "channel_pid",
            "channelPid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChannelPid,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "channelPid" | "channel_pid" => Ok(GeneratedField::ChannelPid),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SubscribeChannel;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.SubscribeChannel")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SubscribeChannel, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut channel_pid__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ChannelPid => {
                            if channel_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelPid"));
                            }
                            channel_pid__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SubscribeChannel {
                    channel_pid: channel_pid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.SubscribeChannel", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SystemNotification {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.kind != 0 {
            len += 1;
        }
        if !self.content.is_empty() {
            len += 1;
        }
        if !self.metadata.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.SystemNotification", len)?;
        if self.kind != 0 {
            let v = SystemNotificationType::try_from(self.kind)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.kind)))?;
            struct_ser.serialize_field("kind", &v)?;
        }
        if !self.content.is_empty() {
            struct_ser.serialize_field("content", &self.content)?;
        }
        if !self.metadata.is_empty() {
            struct_ser.serialize_field("metadata", &self.metadata)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SystemNotification {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "kind",
            "content",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Kind,
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
                            "kind" => Ok(GeneratedField::Kind),
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
            type Value = SystemNotification;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.SystemNotification")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SystemNotification, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut kind__ = None;
                let mut content__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Kind => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kind"));
                            }
                            kind__ = Some(map_.next_value::<SystemNotificationType>()? as i32);
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
                            metadata__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(SystemNotification {
                    kind: kind__.unwrap_or_default(),
                    content: content__.unwrap_or_default(),
                    metadata: metadata__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.SystemNotification", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SystemNotificationType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "SYSTEM_NOTIFICATION_TYPE_UNSPECIFIED",
            Self::Info => "SYSTEM_NOTIFICATION_TYPE_INFO",
            Self::Warning => "SYSTEM_NOTIFICATION_TYPE_WARNING",
            Self::Error => "SYSTEM_NOTIFICATION_TYPE_ERROR",
            Self::Maintenance => "SYSTEM_NOTIFICATION_TYPE_MAINTENANCE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for SystemNotificationType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SYSTEM_NOTIFICATION_TYPE_UNSPECIFIED",
            "SYSTEM_NOTIFICATION_TYPE_INFO",
            "SYSTEM_NOTIFICATION_TYPE_WARNING",
            "SYSTEM_NOTIFICATION_TYPE_ERROR",
            "SYSTEM_NOTIFICATION_TYPE_MAINTENANCE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SystemNotificationType;

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
                    "SYSTEM_NOTIFICATION_TYPE_UNSPECIFIED" => Ok(SystemNotificationType::Unspecified),
                    "SYSTEM_NOTIFICATION_TYPE_INFO" => Ok(SystemNotificationType::Info),
                    "SYSTEM_NOTIFICATION_TYPE_WARNING" => Ok(SystemNotificationType::Warning),
                    "SYSTEM_NOTIFICATION_TYPE_ERROR" => Ok(SystemNotificationType::Error),
                    "SYSTEM_NOTIFICATION_TYPE_MAINTENANCE" => Ok(SystemNotificationType::Maintenance),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for TypingIndicator {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.channel_pid.is_empty() {
            len += 1;
        }
        if self.is_typing {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.TypingIndicator", len)?;
        if !self.channel_pid.is_empty() {
            struct_ser.serialize_field("channelPid", &self.channel_pid)?;
        }
        if self.is_typing {
            struct_ser.serialize_field("isTyping", &self.is_typing)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TypingIndicator {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "channel_pid",
            "channelPid",
            "is_typing",
            "isTyping",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChannelPid,
            IsTyping,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "channelPid" | "channel_pid" => Ok(GeneratedField::ChannelPid),
                            "isTyping" | "is_typing" => Ok(GeneratedField::IsTyping),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TypingIndicator;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.TypingIndicator")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TypingIndicator, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut channel_pid__ = None;
                let mut is_typing__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ChannelPid => {
                            if channel_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelPid"));
                            }
                            channel_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsTyping => {
                            if is_typing__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isTyping"));
                            }
                            is_typing__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(TypingIndicator {
                    channel_pid: channel_pid__.unwrap_or_default(),
                    is_typing: is_typing__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.TypingIndicator", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UnbanUserRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.channel_pid.is_empty() {
            len += 1;
        }
        if !self.user_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.UnbanUserRequest", len)?;
        if !self.channel_pid.is_empty() {
            struct_ser.serialize_field("channelPid", &self.channel_pid)?;
        }
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UnbanUserRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "channel_pid",
            "channelPid",
            "user_id",
            "userId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChannelPid,
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
                            "channelPid" | "channel_pid" => Ok(GeneratedField::ChannelPid),
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
            type Value = UnbanUserRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.UnbanUserRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UnbanUserRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut channel_pid__ = None;
                let mut user_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ChannelPid => {
                            if channel_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelPid"));
                            }
                            channel_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UnbanUserRequest {
                    channel_pid: channel_pid__.unwrap_or_default(),
                    user_id: user_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.UnbanUserRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UnbanUserResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("chat.v1.UnbanUserResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UnbanUserResponse {
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
            type Value = UnbanUserResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.UnbanUserResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UnbanUserResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(UnbanUserResponse {
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.UnbanUserResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UnmuteUserRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.channel_pid.is_empty() {
            len += 1;
        }
        if !self.user_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.UnmuteUserRequest", len)?;
        if !self.channel_pid.is_empty() {
            struct_ser.serialize_field("channelPid", &self.channel_pid)?;
        }
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UnmuteUserRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "channel_pid",
            "channelPid",
            "user_id",
            "userId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChannelPid,
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
                            "channelPid" | "channel_pid" => Ok(GeneratedField::ChannelPid),
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
            type Value = UnmuteUserRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.UnmuteUserRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UnmuteUserRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut channel_pid__ = None;
                let mut user_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ChannelPid => {
                            if channel_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelPid"));
                            }
                            channel_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UnmuteUserRequest {
                    channel_pid: channel_pid__.unwrap_or_default(),
                    user_id: user_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.UnmuteUserRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UnmuteUserResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("chat.v1.UnmuteUserResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UnmuteUserResponse {
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
            type Value = UnmuteUserResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.UnmuteUserResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UnmuteUserResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(UnmuteUserResponse {
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.UnmuteUserResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UnpinMessageRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.message_pid.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.UnpinMessageRequest", len)?;
        if !self.message_pid.is_empty() {
            struct_ser.serialize_field("messagePid", &self.message_pid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UnpinMessageRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "message_pid",
            "messagePid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = UnpinMessageRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.UnpinMessageRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UnpinMessageRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut message_pid__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MessagePid => {
                            if message_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messagePid"));
                            }
                            message_pid__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UnpinMessageRequest {
                    message_pid: message_pid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.UnpinMessageRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UnpinMessageResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("chat.v1.UnpinMessageResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UnpinMessageResponse {
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
            type Value = UnpinMessageResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.UnpinMessageResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UnpinMessageResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(UnpinMessageResponse {
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.UnpinMessageResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UnreadInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.unread_count != 0 {
            len += 1;
        }
        if self.mention_count != 0 {
            len += 1;
        }
        if self.last_message_at.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.UnreadInfo", len)?;
        if self.unread_count != 0 {
            struct_ser.serialize_field("unreadCount", &self.unread_count)?;
        }
        if self.mention_count != 0 {
            struct_ser.serialize_field("mentionCount", &self.mention_count)?;
        }
        if let Some(v) = self.last_message_at.as_ref() {
            struct_ser.serialize_field("lastMessageAt", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UnreadInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "unread_count",
            "unreadCount",
            "mention_count",
            "mentionCount",
            "last_message_at",
            "lastMessageAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UnreadCount,
            MentionCount,
            LastMessageAt,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "unreadCount" | "unread_count" => Ok(GeneratedField::UnreadCount),
                            "mentionCount" | "mention_count" => Ok(GeneratedField::MentionCount),
                            "lastMessageAt" | "last_message_at" => Ok(GeneratedField::LastMessageAt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UnreadInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.UnreadInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UnreadInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut unread_count__ = None;
                let mut mention_count__ = None;
                let mut last_message_at__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UnreadCount => {
                            if unread_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unreadCount"));
                            }
                            unread_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MentionCount => {
                            if mention_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mentionCount"));
                            }
                            mention_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::LastMessageAt => {
                            if last_message_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastMessageAt"));
                            }
                            last_message_at__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UnreadInfo {
                    unread_count: unread_count__.unwrap_or_default(),
                    mention_count: mention_count__.unwrap_or_default(),
                    last_message_at: last_message_at__,
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.UnreadInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UnsubscribeChannel {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.channel_pid.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.UnsubscribeChannel", len)?;
        if !self.channel_pid.is_empty() {
            struct_ser.serialize_field("channelPid", &self.channel_pid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UnsubscribeChannel {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "channel_pid",
            "channelPid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChannelPid,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "channelPid" | "channel_pid" => Ok(GeneratedField::ChannelPid),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UnsubscribeChannel;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.UnsubscribeChannel")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UnsubscribeChannel, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut channel_pid__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ChannelPid => {
                            if channel_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelPid"));
                            }
                            channel_pid__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UnsubscribeChannel {
                    channel_pid: channel_pid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.UnsubscribeChannel", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateAutoModSettingsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.channel_pid.is_empty() {
            len += 1;
        }
        if self.settings.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.UpdateAutoModSettingsRequest", len)?;
        if !self.channel_pid.is_empty() {
            struct_ser.serialize_field("channelPid", &self.channel_pid)?;
        }
        if let Some(v) = self.settings.as_ref() {
            struct_ser.serialize_field("settings", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateAutoModSettingsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "channel_pid",
            "channelPid",
            "settings",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChannelPid,
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
                            "channelPid" | "channel_pid" => Ok(GeneratedField::ChannelPid),
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
            type Value = UpdateAutoModSettingsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.UpdateAutoModSettingsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateAutoModSettingsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut channel_pid__ = None;
                let mut settings__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ChannelPid => {
                            if channel_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelPid"));
                            }
                            channel_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Settings => {
                            if settings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("settings"));
                            }
                            settings__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateAutoModSettingsRequest {
                    channel_pid: channel_pid__.unwrap_or_default(),
                    settings: settings__,
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.UpdateAutoModSettingsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateAutoModSettingsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.settings.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.UpdateAutoModSettingsResponse", len)?;
        if let Some(v) = self.settings.as_ref() {
            struct_ser.serialize_field("settings", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateAutoModSettingsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "settings",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = UpdateAutoModSettingsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.UpdateAutoModSettingsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateAutoModSettingsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut settings__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Settings => {
                            if settings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("settings"));
                            }
                            settings__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateAutoModSettingsResponse {
                    settings: settings__,
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.UpdateAutoModSettingsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateChannelRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.channel_pid.is_empty() {
            len += 1;
        }
        if self.name.is_some() {
            len += 1;
        }
        if self.description.is_some() {
            len += 1;
        }
        if self.icon_url.is_some() {
            len += 1;
        }
        if self.archived.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.UpdateChannelRequest", len)?;
        if !self.channel_pid.is_empty() {
            struct_ser.serialize_field("channelPid", &self.channel_pid)?;
        }
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        if let Some(v) = self.icon_url.as_ref() {
            struct_ser.serialize_field("iconUrl", v)?;
        }
        if let Some(v) = self.archived.as_ref() {
            struct_ser.serialize_field("archived", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateChannelRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "channel_pid",
            "channelPid",
            "name",
            "description",
            "icon_url",
            "iconUrl",
            "archived",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChannelPid,
            Name,
            Description,
            IconUrl,
            Archived,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "channelPid" | "channel_pid" => Ok(GeneratedField::ChannelPid),
                            "name" => Ok(GeneratedField::Name),
                            "description" => Ok(GeneratedField::Description),
                            "iconUrl" | "icon_url" => Ok(GeneratedField::IconUrl),
                            "archived" => Ok(GeneratedField::Archived),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateChannelRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.UpdateChannelRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateChannelRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut channel_pid__ = None;
                let mut name__ = None;
                let mut description__ = None;
                let mut icon_url__ = None;
                let mut archived__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ChannelPid => {
                            if channel_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelPid"));
                            }
                            channel_pid__ = Some(map_.next_value()?);
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
                        GeneratedField::IconUrl => {
                            if icon_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("iconUrl"));
                            }
                            icon_url__ = map_.next_value()?;
                        }
                        GeneratedField::Archived => {
                            if archived__.is_some() {
                                return Err(serde::de::Error::duplicate_field("archived"));
                            }
                            archived__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateChannelRequest {
                    channel_pid: channel_pid__.unwrap_or_default(),
                    name: name__,
                    description: description__,
                    icon_url: icon_url__,
                    archived: archived__,
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.UpdateChannelRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateChannelResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.channel.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.UpdateChannelResponse", len)?;
        if let Some(v) = self.channel.as_ref() {
            struct_ser.serialize_field("channel", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateChannelResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "channel",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Channel,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "channel" => Ok(GeneratedField::Channel),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateChannelResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.UpdateChannelResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateChannelResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut channel__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Channel => {
                            if channel__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channel"));
                            }
                            channel__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateChannelResponse {
                    channel: channel__,
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.UpdateChannelResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateMemberRoleRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.channel_pid.is_empty() {
            len += 1;
        }
        if !self.user_id.is_empty() {
            len += 1;
        }
        if self.new_role != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.UpdateMemberRoleRequest", len)?;
        if !self.channel_pid.is_empty() {
            struct_ser.serialize_field("channelPid", &self.channel_pid)?;
        }
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if self.new_role != 0 {
            let v = ChannelRole::try_from(self.new_role)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.new_role)))?;
            struct_ser.serialize_field("newRole", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateMemberRoleRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "channel_pid",
            "channelPid",
            "user_id",
            "userId",
            "new_role",
            "newRole",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChannelPid,
            UserId,
            NewRole,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "channelPid" | "channel_pid" => Ok(GeneratedField::ChannelPid),
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
                            "newRole" | "new_role" => Ok(GeneratedField::NewRole),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateMemberRoleRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.UpdateMemberRoleRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateMemberRoleRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut channel_pid__ = None;
                let mut user_id__ = None;
                let mut new_role__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ChannelPid => {
                            if channel_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelPid"));
                            }
                            channel_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NewRole => {
                            if new_role__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newRole"));
                            }
                            new_role__ = Some(map_.next_value::<ChannelRole>()? as i32);
                        }
                    }
                }
                Ok(UpdateMemberRoleRequest {
                    channel_pid: channel_pid__.unwrap_or_default(),
                    user_id: user_id__.unwrap_or_default(),
                    new_role: new_role__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.UpdateMemberRoleRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateMemberRoleResponse {
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
        let mut struct_ser = serializer.serialize_struct("chat.v1.UpdateMemberRoleResponse", len)?;
        if let Some(v) = self.member.as_ref() {
            struct_ser.serialize_field("member", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateMemberRoleResponse {
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
            type Value = UpdateMemberRoleResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.UpdateMemberRoleResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateMemberRoleResponse, V::Error>
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
                Ok(UpdateMemberRoleResponse {
                    member: member__,
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.UpdateMemberRoleResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UserJoinedChannel {
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
        if !self.user_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.UserJoinedChannel", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if !self.user_name.is_empty() {
            struct_ser.serialize_field("userName", &self.user_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UserJoinedChannel {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
            "user_name",
            "userName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
            UserName,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "userName" | "user_name" => Ok(GeneratedField::UserName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UserJoinedChannel;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.UserJoinedChannel")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UserJoinedChannel, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut user_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserName => {
                            if user_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userName"));
                            }
                            user_name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UserJoinedChannel {
                    user_id: user_id__.unwrap_or_default(),
                    user_name: user_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.UserJoinedChannel", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UserLeftChannel {
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
        let mut struct_ser = serializer.serialize_struct("chat.v1.UserLeftChannel", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UserLeftChannel {
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
            type Value = UserLeftChannel;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.UserLeftChannel")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UserLeftChannel, V::Error>
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
                Ok(UserLeftChannel {
                    user_id: user_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.UserLeftChannel", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UserPresence {
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
        if self.status != 0 {
            len += 1;
        }
        if self.status_text.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.UserPresence", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if self.status != 0 {
            let v = PresenceStatus::try_from(self.status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        if let Some(v) = self.status_text.as_ref() {
            struct_ser.serialize_field("statusText", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UserPresence {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
            "status",
            "status_text",
            "statusText",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
            Status,
            StatusText,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "status" => Ok(GeneratedField::Status),
                            "statusText" | "status_text" => Ok(GeneratedField::StatusText),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UserPresence;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.UserPresence")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UserPresence, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut status__ = None;
                let mut status_text__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value::<PresenceStatus>()? as i32);
                        }
                        GeneratedField::StatusText => {
                            if status_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statusText"));
                            }
                            status_text__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UserPresence {
                    user_id: user_id__.unwrap_or_default(),
                    status: status__.unwrap_or_default(),
                    status_text: status_text__,
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.UserPresence", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UserSender {
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
        if self.avatar_url.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.UserSender", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if let Some(v) = self.avatar_url.as_ref() {
            struct_ser.serialize_field("avatarUrl", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UserSender {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
            "avatar_url",
            "avatarUrl",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
            AvatarUrl,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "avatarUrl" | "avatar_url" => Ok(GeneratedField::AvatarUrl),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UserSender;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.UserSender")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UserSender, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut avatar_url__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AvatarUrl => {
                            if avatar_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("avatarUrl"));
                            }
                            avatar_url__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UserSender {
                    user_id: user_id__.unwrap_or_default(),
                    avatar_url: avatar_url__,
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.UserSender", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UserTyping {
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
        if !self.user_name.is_empty() {
            len += 1;
        }
        if self.is_typing {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.UserTyping", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if !self.user_name.is_empty() {
            struct_ser.serialize_field("userName", &self.user_name)?;
        }
        if self.is_typing {
            struct_ser.serialize_field("isTyping", &self.is_typing)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UserTyping {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
            "user_name",
            "userName",
            "is_typing",
            "isTyping",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
            UserName,
            IsTyping,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "userName" | "user_name" => Ok(GeneratedField::UserName),
                            "isTyping" | "is_typing" => Ok(GeneratedField::IsTyping),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UserTyping;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.UserTyping")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UserTyping, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut user_name__ = None;
                let mut is_typing__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserName => {
                            if user_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userName"));
                            }
                            user_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsTyping => {
                            if is_typing__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isTyping"));
                            }
                            is_typing__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UserTyping {
                    user_id: user_id__.unwrap_or_default(),
                    user_name: user_name__.unwrap_or_default(),
                    is_typing: is_typing__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.UserTyping", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WarnUserAction {
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
        if !self.warning_message.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("chat.v1.WarnUserAction", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if !self.warning_message.is_empty() {
            struct_ser.serialize_field("warningMessage", &self.warning_message)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WarnUserAction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
            "warning_message",
            "warningMessage",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
            WarningMessage,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
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
                            "warningMessage" | "warning_message" => Ok(GeneratedField::WarningMessage),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WarnUserAction;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct chat.v1.WarnUserAction")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WarnUserAction, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut warning_message__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::WarningMessage => {
                            if warning_message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("warningMessage"));
                            }
                            warning_message__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(WarnUserAction {
                    user_id: user_id__.unwrap_or_default(),
                    warning_message: warning_message__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("chat.v1.WarnUserAction", FIELDS, GeneratedVisitor)
    }
}
