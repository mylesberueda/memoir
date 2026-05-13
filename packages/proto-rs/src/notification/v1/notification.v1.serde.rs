// @generated
impl serde::Serialize for ActionType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "ACTION_TYPE_UNSPECIFIED",
            Self::Navigate => "ACTION_TYPE_NAVIGATE",
            Self::Post => "ACTION_TYPE_POST",
            Self::Get => "ACTION_TYPE_GET",
            Self::Dismiss => "ACTION_TYPE_DISMISS",
            Self::External => "ACTION_TYPE_EXTERNAL",
            Self::Modal => "ACTION_TYPE_MODAL",
            Self::Download => "ACTION_TYPE_DOWNLOAD",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ActionType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ACTION_TYPE_UNSPECIFIED",
            "ACTION_TYPE_NAVIGATE",
            "ACTION_TYPE_POST",
            "ACTION_TYPE_GET",
            "ACTION_TYPE_DISMISS",
            "ACTION_TYPE_EXTERNAL",
            "ACTION_TYPE_MODAL",
            "ACTION_TYPE_DOWNLOAD",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ActionType;

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
                    "ACTION_TYPE_UNSPECIFIED" => Ok(ActionType::Unspecified),
                    "ACTION_TYPE_NAVIGATE" => Ok(ActionType::Navigate),
                    "ACTION_TYPE_POST" => Ok(ActionType::Post),
                    "ACTION_TYPE_GET" => Ok(ActionType::Get),
                    "ACTION_TYPE_DISMISS" => Ok(ActionType::Dismiss),
                    "ACTION_TYPE_EXTERNAL" => Ok(ActionType::External),
                    "ACTION_TYPE_MODAL" => Ok(ActionType::Modal),
                    "ACTION_TYPE_DOWNLOAD" => Ok(ActionType::Download),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for AgentCompleteContext {
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
        if !self.agent_name.is_empty() {
            len += 1;
        }
        if !self.run_pid.is_empty() {
            len += 1;
        }
        if self.success {
            len += 1;
        }
        if self.error_message.is_some() {
            len += 1;
        }
        if self.result_summary.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("notification.v1.AgentCompleteContext", len)?;
        if !self.agent_pid.is_empty() {
            struct_ser.serialize_field("agentPid", &self.agent_pid)?;
        }
        if !self.agent_name.is_empty() {
            struct_ser.serialize_field("agentName", &self.agent_name)?;
        }
        if !self.run_pid.is_empty() {
            struct_ser.serialize_field("runPid", &self.run_pid)?;
        }
        if self.success {
            struct_ser.serialize_field("success", &self.success)?;
        }
        if let Some(v) = self.error_message.as_ref() {
            struct_ser.serialize_field("errorMessage", v)?;
        }
        if let Some(v) = self.result_summary.as_ref() {
            struct_ser.serialize_field("resultSummary", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AgentCompleteContext {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "agent_pid",
            "agentPid",
            "agent_name",
            "agentName",
            "run_pid",
            "runPid",
            "success",
            "error_message",
            "errorMessage",
            "result_summary",
            "resultSummary",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AgentPid,
            AgentName,
            RunPid,
            Success,
            ErrorMessage,
            ResultSummary,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "agentName" | "agent_name" => Ok(GeneratedField::AgentName),
                            "runPid" | "run_pid" => Ok(GeneratedField::RunPid),
                            "success" => Ok(GeneratedField::Success),
                            "errorMessage" | "error_message" => Ok(GeneratedField::ErrorMessage),
                            "resultSummary" | "result_summary" => Ok(GeneratedField::ResultSummary),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AgentCompleteContext;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct notification.v1.AgentCompleteContext")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AgentCompleteContext, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut agent_pid__ = None;
                let mut agent_name__ = None;
                let mut run_pid__ = None;
                let mut success__ = None;
                let mut error_message__ = None;
                let mut result_summary__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AgentPid => {
                            if agent_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("agentPid"));
                            }
                            agent_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AgentName => {
                            if agent_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("agentName"));
                            }
                            agent_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RunPid => {
                            if run_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runPid"));
                            }
                            run_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Success => {
                            if success__.is_some() {
                                return Err(serde::de::Error::duplicate_field("success"));
                            }
                            success__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ErrorMessage => {
                            if error_message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorMessage"));
                            }
                            error_message__ = map_.next_value()?;
                        }
                        GeneratedField::ResultSummary => {
                            if result_summary__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resultSummary"));
                            }
                            result_summary__ = map_.next_value()?;
                        }
                    }
                }
                Ok(AgentCompleteContext {
                    agent_pid: agent_pid__.unwrap_or_default(),
                    agent_name: agent_name__.unwrap_or_default(),
                    run_pid: run_pid__.unwrap_or_default(),
                    success: success__.unwrap_or_default(),
                    error_message: error_message__,
                    result_summary: result_summary__,
                })
            }
        }
        deserializer.deserialize_struct("notification.v1.AgentCompleteContext", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CategoryPreference {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.category != 0 {
            len += 1;
        }
        if self.enabled {
            len += 1;
        }
        if self.push {
            len += 1;
        }
        if self.email {
            len += 1;
        }
        if self.min_priority != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("notification.v1.CategoryPreference", len)?;
        if self.category != 0 {
            let v = NotificationCategory::try_from(self.category)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.category)))?;
            struct_ser.serialize_field("category", &v)?;
        }
        if self.enabled {
            struct_ser.serialize_field("enabled", &self.enabled)?;
        }
        if self.push {
            struct_ser.serialize_field("push", &self.push)?;
        }
        if self.email {
            struct_ser.serialize_field("email", &self.email)?;
        }
        if self.min_priority != 0 {
            let v = NotificationPriority::try_from(self.min_priority)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.min_priority)))?;
            struct_ser.serialize_field("minPriority", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CategoryPreference {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "category",
            "enabled",
            "push",
            "email",
            "min_priority",
            "minPriority",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Category,
            Enabled,
            Push,
            Email,
            MinPriority,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "category" => Ok(GeneratedField::Category),
                            "enabled" => Ok(GeneratedField::Enabled),
                            "push" => Ok(GeneratedField::Push),
                            "email" => Ok(GeneratedField::Email),
                            "minPriority" | "min_priority" => Ok(GeneratedField::MinPriority),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CategoryPreference;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct notification.v1.CategoryPreference")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CategoryPreference, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut category__ = None;
                let mut enabled__ = None;
                let mut push__ = None;
                let mut email__ = None;
                let mut min_priority__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Category => {
                            if category__.is_some() {
                                return Err(serde::de::Error::duplicate_field("category"));
                            }
                            category__ = Some(map_.next_value::<NotificationCategory>()? as i32);
                        }
                        GeneratedField::Enabled => {
                            if enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enabled"));
                            }
                            enabled__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Push => {
                            if push__.is_some() {
                                return Err(serde::de::Error::duplicate_field("push"));
                            }
                            push__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Email => {
                            if email__.is_some() {
                                return Err(serde::de::Error::duplicate_field("email"));
                            }
                            email__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MinPriority => {
                            if min_priority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minPriority"));
                            }
                            min_priority__ = Some(map_.next_value::<NotificationPriority>()? as i32);
                        }
                    }
                }
                Ok(CategoryPreference {
                    category: category__.unwrap_or_default(),
                    enabled: enabled__.unwrap_or_default(),
                    push: push__.unwrap_or_default(),
                    email: email__.unwrap_or_default(),
                    min_priority: min_priority__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("notification.v1.CategoryPreference", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ChatMentionContext {
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
        if !self.channel_name.is_empty() {
            len += 1;
        }
        if !self.message_pid.is_empty() {
            len += 1;
        }
        if !self.sender_name.is_empty() {
            len += 1;
        }
        if !self.message_preview.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("notification.v1.ChatMentionContext", len)?;
        if !self.channel_pid.is_empty() {
            struct_ser.serialize_field("channelPid", &self.channel_pid)?;
        }
        if !self.channel_name.is_empty() {
            struct_ser.serialize_field("channelName", &self.channel_name)?;
        }
        if !self.message_pid.is_empty() {
            struct_ser.serialize_field("messagePid", &self.message_pid)?;
        }
        if !self.sender_name.is_empty() {
            struct_ser.serialize_field("senderName", &self.sender_name)?;
        }
        if !self.message_preview.is_empty() {
            struct_ser.serialize_field("messagePreview", &self.message_preview)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ChatMentionContext {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "channel_pid",
            "channelPid",
            "channel_name",
            "channelName",
            "message_pid",
            "messagePid",
            "sender_name",
            "senderName",
            "message_preview",
            "messagePreview",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChannelPid,
            ChannelName,
            MessagePid,
            SenderName,
            MessagePreview,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "channelName" | "channel_name" => Ok(GeneratedField::ChannelName),
                            "messagePid" | "message_pid" => Ok(GeneratedField::MessagePid),
                            "senderName" | "sender_name" => Ok(GeneratedField::SenderName),
                            "messagePreview" | "message_preview" => Ok(GeneratedField::MessagePreview),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ChatMentionContext;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct notification.v1.ChatMentionContext")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ChatMentionContext, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut channel_pid__ = None;
                let mut channel_name__ = None;
                let mut message_pid__ = None;
                let mut sender_name__ = None;
                let mut message_preview__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ChannelPid => {
                            if channel_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelPid"));
                            }
                            channel_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ChannelName => {
                            if channel_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelName"));
                            }
                            channel_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MessagePid => {
                            if message_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messagePid"));
                            }
                            message_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SenderName => {
                            if sender_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("senderName"));
                            }
                            sender_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MessagePreview => {
                            if message_preview__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messagePreview"));
                            }
                            message_preview__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ChatMentionContext {
                    channel_pid: channel_pid__.unwrap_or_default(),
                    channel_name: channel_name__.unwrap_or_default(),
                    message_pid: message_pid__.unwrap_or_default(),
                    sender_name: sender_name__.unwrap_or_default(),
                    message_preview: message_preview__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("notification.v1.ChatMentionContext", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DismissAllRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.category.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("notification.v1.DismissAllRequest", len)?;
        if let Some(v) = self.category.as_ref() {
            let v = NotificationCategory::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("category", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DismissAllRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "category",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Category,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "category" => Ok(GeneratedField::Category),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DismissAllRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct notification.v1.DismissAllRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DismissAllRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut category__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Category => {
                            if category__.is_some() {
                                return Err(serde::de::Error::duplicate_field("category"));
                            }
                            category__ = map_.next_value::<::std::option::Option<NotificationCategory>>()?.map(|x| x as i32);
                        }
                    }
                }
                Ok(DismissAllRequest {
                    category: category__,
                })
            }
        }
        deserializer.deserialize_struct("notification.v1.DismissAllRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DismissAllResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.dismissed_count != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("notification.v1.DismissAllResponse", len)?;
        if self.dismissed_count != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("dismissedCount", ToString::to_string(&self.dismissed_count).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DismissAllResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "dismissed_count",
            "dismissedCount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DismissedCount,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "dismissedCount" | "dismissed_count" => Ok(GeneratedField::DismissedCount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DismissAllResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct notification.v1.DismissAllResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DismissAllResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut dismissed_count__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DismissedCount => {
                            if dismissed_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dismissedCount"));
                            }
                            dismissed_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(DismissAllResponse {
                    dismissed_count: dismissed_count__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("notification.v1.DismissAllResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DismissRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.notification_pid.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("notification.v1.DismissRequest", len)?;
        if !self.notification_pid.is_empty() {
            struct_ser.serialize_field("notificationPid", &self.notification_pid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DismissRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "notification_pid",
            "notificationPid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NotificationPid,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "notificationPid" | "notification_pid" => Ok(GeneratedField::NotificationPid),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DismissRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct notification.v1.DismissRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DismissRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut notification_pid__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::NotificationPid => {
                            if notification_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notificationPid"));
                            }
                            notification_pid__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DismissRequest {
                    notification_pid: notification_pid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("notification.v1.DismissRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetPreferencesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("notification.v1.GetPreferencesRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetPreferencesRequest {
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
            type Value = GetPreferencesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct notification.v1.GetPreferencesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetPreferencesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(GetPreferencesRequest {
                })
            }
        }
        deserializer.deserialize_struct("notification.v1.GetPreferencesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetPreferencesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.preferences.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("notification.v1.GetPreferencesResponse", len)?;
        if let Some(v) = self.preferences.as_ref() {
            struct_ser.serialize_field("preferences", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetPreferencesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "preferences",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Preferences,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "preferences" => Ok(GeneratedField::Preferences),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetPreferencesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct notification.v1.GetPreferencesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetPreferencesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut preferences__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Preferences => {
                            if preferences__.is_some() {
                                return Err(serde::de::Error::duplicate_field("preferences"));
                            }
                            preferences__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetPreferencesResponse {
                    preferences: preferences__,
                })
            }
        }
        deserializer.deserialize_struct("notification.v1.GetPreferencesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.notification_pid.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("notification.v1.GetRequest", len)?;
        if !self.notification_pid.is_empty() {
            struct_ser.serialize_field("notificationPid", &self.notification_pid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "notification_pid",
            "notificationPid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NotificationPid,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "notificationPid" | "notification_pid" => Ok(GeneratedField::NotificationPid),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct notification.v1.GetRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut notification_pid__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::NotificationPid => {
                            if notification_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notificationPid"));
                            }
                            notification_pid__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetRequest {
                    notification_pid: notification_pid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("notification.v1.GetRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.notification.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("notification.v1.GetResponse", len)?;
        if let Some(v) = self.notification.as_ref() {
            struct_ser.serialize_field("notification", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "notification",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Notification,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "notification" => Ok(GeneratedField::Notification),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct notification.v1.GetResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut notification__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Notification => {
                            if notification__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notification"));
                            }
                            notification__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetResponse {
                    notification: notification__,
                })
            }
        }
        deserializer.deserialize_struct("notification.v1.GetResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetUnreadCountRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("notification.v1.GetUnreadCountRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetUnreadCountRequest {
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
            type Value = GetUnreadCountRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct notification.v1.GetUnreadCountRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetUnreadCountRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(GetUnreadCountRequest {
                })
            }
        }
        deserializer.deserialize_struct("notification.v1.GetUnreadCountRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetUnreadCountResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.total != 0 {
            len += 1;
        }
        if !self.by_category.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("notification.v1.GetUnreadCountResponse", len)?;
        if self.total != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("total", ToString::to_string(&self.total).as_str())?;
        }
        if !self.by_category.is_empty() {
            let v: std::collections::HashMap<_, _> = self.by_category.iter()
                .map(|(k, v)| (k, v.to_string())).collect();
            struct_ser.serialize_field("byCategory", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetUnreadCountResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "total",
            "by_category",
            "byCategory",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Total,
            ByCategory,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "total" => Ok(GeneratedField::Total),
                            "byCategory" | "by_category" => Ok(GeneratedField::ByCategory),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetUnreadCountResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct notification.v1.GetUnreadCountResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetUnreadCountResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut total__ = None;
                let mut by_category__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Total => {
                            if total__.is_some() {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            total__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ByCategory => {
                            if by_category__.is_some() {
                                return Err(serde::de::Error::duplicate_field("byCategory"));
                            }
                            by_category__ = Some(
                                map_.next_value::<std::collections::HashMap<_, ::pbjson::private::NumberDeserialize<u64>>>()?
                                    .into_iter().map(|(k,v)| (k, v.0)).collect()
                            );
                        }
                    }
                }
                Ok(GetUnreadCountResponse {
                    total: total__.unwrap_or_default(),
                    by_category: by_category__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("notification.v1.GetUnreadCountResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.unread_only.is_some() {
            len += 1;
        }
        if !self.categories.is_empty() {
            len += 1;
        }
        if self.min_priority.is_some() {
            len += 1;
        }
        if self.limit != 0 {
            len += 1;
        }
        if self.cursor.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("notification.v1.ListRequest", len)?;
        if let Some(v) = self.unread_only.as_ref() {
            struct_ser.serialize_field("unreadOnly", v)?;
        }
        if !self.categories.is_empty() {
            let v = self.categories.iter().cloned().map(|v| {
                NotificationCategory::try_from(v)
                    .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", v)))
                }).collect::<std::result::Result<Vec<_>, _>>()?;
            struct_ser.serialize_field("categories", &v)?;
        }
        if let Some(v) = self.min_priority.as_ref() {
            let v = NotificationPriority::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("minPriority", &v)?;
        }
        if self.limit != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("limit", ToString::to_string(&self.limit).as_str())?;
        }
        if let Some(v) = self.cursor.as_ref() {
            struct_ser.serialize_field("cursor", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "unread_only",
            "unreadOnly",
            "categories",
            "min_priority",
            "minPriority",
            "limit",
            "cursor",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UnreadOnly,
            Categories,
            MinPriority,
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
                            "unreadOnly" | "unread_only" => Ok(GeneratedField::UnreadOnly),
                            "categories" => Ok(GeneratedField::Categories),
                            "minPriority" | "min_priority" => Ok(GeneratedField::MinPriority),
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
            type Value = ListRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct notification.v1.ListRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut unread_only__ = None;
                let mut categories__ = None;
                let mut min_priority__ = None;
                let mut limit__ = None;
                let mut cursor__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UnreadOnly => {
                            if unread_only__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unreadOnly"));
                            }
                            unread_only__ = map_.next_value()?;
                        }
                        GeneratedField::Categories => {
                            if categories__.is_some() {
                                return Err(serde::de::Error::duplicate_field("categories"));
                            }
                            categories__ = Some(map_.next_value::<Vec<NotificationCategory>>()?.into_iter().map(|x| x as i32).collect());
                        }
                        GeneratedField::MinPriority => {
                            if min_priority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minPriority"));
                            }
                            min_priority__ = map_.next_value::<::std::option::Option<NotificationPriority>>()?.map(|x| x as i32);
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
                Ok(ListRequest {
                    unread_only: unread_only__,
                    categories: categories__.unwrap_or_default(),
                    min_priority: min_priority__,
                    limit: limit__.unwrap_or_default(),
                    cursor: cursor__,
                })
            }
        }
        deserializer.deserialize_struct("notification.v1.ListRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.notifications.is_empty() {
            len += 1;
        }
        if self.total_count != 0 {
            len += 1;
        }
        if self.unread_count != 0 {
            len += 1;
        }
        if self.next_cursor.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("notification.v1.ListResponse", len)?;
        if !self.notifications.is_empty() {
            struct_ser.serialize_field("notifications", &self.notifications)?;
        }
        if self.total_count != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("totalCount", ToString::to_string(&self.total_count).as_str())?;
        }
        if self.unread_count != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("unreadCount", ToString::to_string(&self.unread_count).as_str())?;
        }
        if let Some(v) = self.next_cursor.as_ref() {
            struct_ser.serialize_field("nextCursor", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "notifications",
            "total_count",
            "totalCount",
            "unread_count",
            "unreadCount",
            "next_cursor",
            "nextCursor",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Notifications,
            TotalCount,
            UnreadCount,
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
                            "notifications" => Ok(GeneratedField::Notifications),
                            "totalCount" | "total_count" => Ok(GeneratedField::TotalCount),
                            "unreadCount" | "unread_count" => Ok(GeneratedField::UnreadCount),
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
            type Value = ListResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct notification.v1.ListResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut notifications__ = None;
                let mut total_count__ = None;
                let mut unread_count__ = None;
                let mut next_cursor__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Notifications => {
                            if notifications__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notifications"));
                            }
                            notifications__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TotalCount => {
                            if total_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalCount"));
                            }
                            total_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::UnreadCount => {
                            if unread_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unreadCount"));
                            }
                            unread_count__ = 
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
                Ok(ListResponse {
                    notifications: notifications__.unwrap_or_default(),
                    total_count: total_count__.unwrap_or_default(),
                    unread_count: unread_count__.unwrap_or_default(),
                    next_cursor: next_cursor__,
                })
            }
        }
        deserializer.deserialize_struct("notification.v1.ListResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MarkAllAsReadRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.category.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("notification.v1.MarkAllAsReadRequest", len)?;
        if let Some(v) = self.category.as_ref() {
            let v = NotificationCategory::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("category", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MarkAllAsReadRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "category",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Category,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "category" => Ok(GeneratedField::Category),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MarkAllAsReadRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct notification.v1.MarkAllAsReadRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MarkAllAsReadRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut category__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Category => {
                            if category__.is_some() {
                                return Err(serde::de::Error::duplicate_field("category"));
                            }
                            category__ = map_.next_value::<::std::option::Option<NotificationCategory>>()?.map(|x| x as i32);
                        }
                    }
                }
                Ok(MarkAllAsReadRequest {
                    category: category__,
                })
            }
        }
        deserializer.deserialize_struct("notification.v1.MarkAllAsReadRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MarkAllAsReadResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.marked_count != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("notification.v1.MarkAllAsReadResponse", len)?;
        if self.marked_count != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("markedCount", ToString::to_string(&self.marked_count).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MarkAllAsReadResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "marked_count",
            "markedCount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MarkedCount,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "markedCount" | "marked_count" => Ok(GeneratedField::MarkedCount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MarkAllAsReadResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct notification.v1.MarkAllAsReadResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MarkAllAsReadResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut marked_count__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MarkedCount => {
                            if marked_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("markedCount"));
                            }
                            marked_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(MarkAllAsReadResponse {
                    marked_count: marked_count__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("notification.v1.MarkAllAsReadResponse", FIELDS, GeneratedVisitor)
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
        if !self.notification_pids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("notification.v1.MarkAsReadRequest", len)?;
        if !self.notification_pids.is_empty() {
            struct_ser.serialize_field("notificationPids", &self.notification_pids)?;
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
            "notification_pids",
            "notificationPids",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NotificationPids,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "notificationPids" | "notification_pids" => Ok(GeneratedField::NotificationPids),
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
                formatter.write_str("struct notification.v1.MarkAsReadRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MarkAsReadRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut notification_pids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::NotificationPids => {
                            if notification_pids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notificationPids"));
                            }
                            notification_pids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MarkAsReadRequest {
                    notification_pids: notification_pids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("notification.v1.MarkAsReadRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Notification {
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
        if !self.user_id.is_empty() {
            len += 1;
        }
        if !self.org_pid.is_empty() {
            len += 1;
        }
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if self.icon_url.is_some() {
            len += 1;
        }
        if self.category != 0 {
            len += 1;
        }
        if self.priority != 0 {
            len += 1;
        }
        if !self.actions.is_empty() {
            len += 1;
        }
        if self.origin_service != 0 {
            len += 1;
        }
        if self.origin_entity_type.is_some() {
            len += 1;
        }
        if self.origin_entity_pid.is_some() {
            len += 1;
        }
        if self.is_read {
            len += 1;
        }
        if self.read_at.is_some() {
            len += 1;
        }
        if self.is_dismissed {
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
        let mut struct_ser = serializer.serialize_struct("notification.v1.Notification", len)?;
        if !self.pid.is_empty() {
            struct_ser.serialize_field("pid", &self.pid)?;
        }
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if !self.org_pid.is_empty() {
            struct_ser.serialize_field("orgPid", &self.org_pid)?;
        }
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if let Some(v) = self.icon_url.as_ref() {
            struct_ser.serialize_field("iconUrl", v)?;
        }
        if self.category != 0 {
            let v = NotificationCategory::try_from(self.category)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.category)))?;
            struct_ser.serialize_field("category", &v)?;
        }
        if self.priority != 0 {
            let v = NotificationPriority::try_from(self.priority)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.priority)))?;
            struct_ser.serialize_field("priority", &v)?;
        }
        if !self.actions.is_empty() {
            struct_ser.serialize_field("actions", &self.actions)?;
        }
        if self.origin_service != 0 {
            let v = OriginService::try_from(self.origin_service)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.origin_service)))?;
            struct_ser.serialize_field("originService", &v)?;
        }
        if let Some(v) = self.origin_entity_type.as_ref() {
            let v = OriginEntityType::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("originEntityType", &v)?;
        }
        if let Some(v) = self.origin_entity_pid.as_ref() {
            struct_ser.serialize_field("originEntityPid", v)?;
        }
        if self.is_read {
            struct_ser.serialize_field("isRead", &self.is_read)?;
        }
        if let Some(v) = self.read_at.as_ref() {
            struct_ser.serialize_field("readAt", v)?;
        }
        if self.is_dismissed {
            struct_ser.serialize_field("isDismissed", &self.is_dismissed)?;
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
impl<'de> serde::Deserialize<'de> for Notification {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pid",
            "user_id",
            "userId",
            "org_pid",
            "orgPid",
            "title",
            "description",
            "icon_url",
            "iconUrl",
            "category",
            "priority",
            "actions",
            "origin_service",
            "originService",
            "origin_entity_type",
            "originEntityType",
            "origin_entity_pid",
            "originEntityPid",
            "is_read",
            "isRead",
            "read_at",
            "readAt",
            "is_dismissed",
            "isDismissed",
            "expires_at",
            "expiresAt",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pid,
            UserId,
            OrgPid,
            Title,
            Description,
            IconUrl,
            Category,
            Priority,
            Actions,
            OriginService,
            OriginEntityType,
            OriginEntityPid,
            IsRead,
            ReadAt,
            IsDismissed,
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
                            "pid" => Ok(GeneratedField::Pid),
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
                            "orgPid" | "org_pid" => Ok(GeneratedField::OrgPid),
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "iconUrl" | "icon_url" => Ok(GeneratedField::IconUrl),
                            "category" => Ok(GeneratedField::Category),
                            "priority" => Ok(GeneratedField::Priority),
                            "actions" => Ok(GeneratedField::Actions),
                            "originService" | "origin_service" => Ok(GeneratedField::OriginService),
                            "originEntityType" | "origin_entity_type" => Ok(GeneratedField::OriginEntityType),
                            "originEntityPid" | "origin_entity_pid" => Ok(GeneratedField::OriginEntityPid),
                            "isRead" | "is_read" => Ok(GeneratedField::IsRead),
                            "readAt" | "read_at" => Ok(GeneratedField::ReadAt),
                            "isDismissed" | "is_dismissed" => Ok(GeneratedField::IsDismissed),
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
            type Value = Notification;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct notification.v1.Notification")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Notification, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pid__ = None;
                let mut user_id__ = None;
                let mut org_pid__ = None;
                let mut title__ = None;
                let mut description__ = None;
                let mut icon_url__ = None;
                let mut category__ = None;
                let mut priority__ = None;
                let mut actions__ = None;
                let mut origin_service__ = None;
                let mut origin_entity_type__ = None;
                let mut origin_entity_pid__ = None;
                let mut is_read__ = None;
                let mut read_at__ = None;
                let mut is_dismissed__ = None;
                let mut expires_at__ = None;
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
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrgPid => {
                            if org_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orgPid"));
                            }
                            org_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IconUrl => {
                            if icon_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("iconUrl"));
                            }
                            icon_url__ = map_.next_value()?;
                        }
                        GeneratedField::Category => {
                            if category__.is_some() {
                                return Err(serde::de::Error::duplicate_field("category"));
                            }
                            category__ = Some(map_.next_value::<NotificationCategory>()? as i32);
                        }
                        GeneratedField::Priority => {
                            if priority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priority"));
                            }
                            priority__ = Some(map_.next_value::<NotificationPriority>()? as i32);
                        }
                        GeneratedField::Actions => {
                            if actions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actions"));
                            }
                            actions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OriginService => {
                            if origin_service__.is_some() {
                                return Err(serde::de::Error::duplicate_field("originService"));
                            }
                            origin_service__ = Some(map_.next_value::<OriginService>()? as i32);
                        }
                        GeneratedField::OriginEntityType => {
                            if origin_entity_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("originEntityType"));
                            }
                            origin_entity_type__ = map_.next_value::<::std::option::Option<OriginEntityType>>()?.map(|x| x as i32);
                        }
                        GeneratedField::OriginEntityPid => {
                            if origin_entity_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("originEntityPid"));
                            }
                            origin_entity_pid__ = map_.next_value()?;
                        }
                        GeneratedField::IsRead => {
                            if is_read__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isRead"));
                            }
                            is_read__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ReadAt => {
                            if read_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("readAt"));
                            }
                            read_at__ = map_.next_value()?;
                        }
                        GeneratedField::IsDismissed => {
                            if is_dismissed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isDismissed"));
                            }
                            is_dismissed__ = Some(map_.next_value()?);
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
                Ok(Notification {
                    pid: pid__.unwrap_or_default(),
                    user_id: user_id__.unwrap_or_default(),
                    org_pid: org_pid__.unwrap_or_default(),
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    icon_url: icon_url__,
                    category: category__.unwrap_or_default(),
                    priority: priority__.unwrap_or_default(),
                    actions: actions__.unwrap_or_default(),
                    origin_service: origin_service__.unwrap_or_default(),
                    origin_entity_type: origin_entity_type__,
                    origin_entity_pid: origin_entity_pid__,
                    is_read: is_read__.unwrap_or_default(),
                    read_at: read_at__,
                    is_dismissed: is_dismissed__.unwrap_or_default(),
                    expires_at: expires_at__,
                    created_at: created_at__.unwrap_or_default(),
                    updated_at: updated_at__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("notification.v1.Notification", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NotificationAction {
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
        if !self.target.is_empty() {
            len += 1;
        }
        if !self.params.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("notification.v1.NotificationAction", len)?;
        if self.kind != 0 {
            let v = ActionType::try_from(self.kind)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.kind)))?;
            struct_ser.serialize_field("kind", &v)?;
        }
        if !self.target.is_empty() {
            struct_ser.serialize_field("target", &self.target)?;
        }
        if !self.params.is_empty() {
            struct_ser.serialize_field("params", &self.params)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NotificationAction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "kind",
            "target",
            "params",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Kind,
            Target,
            Params,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "target" => Ok(GeneratedField::Target),
                            "params" => Ok(GeneratedField::Params),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NotificationAction;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct notification.v1.NotificationAction")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<NotificationAction, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut kind__ = None;
                let mut target__ = None;
                let mut params__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Kind => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kind"));
                            }
                            kind__ = Some(map_.next_value::<ActionType>()? as i32);
                        }
                        GeneratedField::Target => {
                            if target__.is_some() {
                                return Err(serde::de::Error::duplicate_field("target"));
                            }
                            target__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Params => {
                            if params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            params__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(NotificationAction {
                    kind: kind__.unwrap_or_default(),
                    target: target__.unwrap_or_default(),
                    params: params__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("notification.v1.NotificationAction", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NotificationCategory {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "NOTIFICATION_CATEGORY_UNSPECIFIED",
            Self::Chat => "NOTIFICATION_CATEGORY_CHAT",
            Self::Agent => "NOTIFICATION_CATEGORY_AGENT",
            Self::System => "NOTIFICATION_CATEGORY_SYSTEM",
            Self::Moderation => "NOTIFICATION_CATEGORY_MODERATION",
            Self::Billing => "NOTIFICATION_CATEGORY_BILLING",
            Self::Social => "NOTIFICATION_CATEGORY_SOCIAL",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for NotificationCategory {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "NOTIFICATION_CATEGORY_UNSPECIFIED",
            "NOTIFICATION_CATEGORY_CHAT",
            "NOTIFICATION_CATEGORY_AGENT",
            "NOTIFICATION_CATEGORY_SYSTEM",
            "NOTIFICATION_CATEGORY_MODERATION",
            "NOTIFICATION_CATEGORY_BILLING",
            "NOTIFICATION_CATEGORY_SOCIAL",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NotificationCategory;

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
                    "NOTIFICATION_CATEGORY_UNSPECIFIED" => Ok(NotificationCategory::Unspecified),
                    "NOTIFICATION_CATEGORY_CHAT" => Ok(NotificationCategory::Chat),
                    "NOTIFICATION_CATEGORY_AGENT" => Ok(NotificationCategory::Agent),
                    "NOTIFICATION_CATEGORY_SYSTEM" => Ok(NotificationCategory::System),
                    "NOTIFICATION_CATEGORY_MODERATION" => Ok(NotificationCategory::Moderation),
                    "NOTIFICATION_CATEGORY_BILLING" => Ok(NotificationCategory::Billing),
                    "NOTIFICATION_CATEGORY_SOCIAL" => Ok(NotificationCategory::Social),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for NotificationDismissed {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.notification_pid.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("notification.v1.NotificationDismissed", len)?;
        if !self.notification_pid.is_empty() {
            struct_ser.serialize_field("notificationPid", &self.notification_pid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NotificationDismissed {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "notification_pid",
            "notificationPid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NotificationPid,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "notificationPid" | "notification_pid" => Ok(GeneratedField::NotificationPid),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NotificationDismissed;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct notification.v1.NotificationDismissed")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<NotificationDismissed, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut notification_pid__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::NotificationPid => {
                            if notification_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notificationPid"));
                            }
                            notification_pid__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(NotificationDismissed {
                    notification_pid: notification_pid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("notification.v1.NotificationDismissed", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NotificationEvent {
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
        let mut struct_ser = serializer.serialize_struct("notification.v1.NotificationEvent", len)?;
        if let Some(v) = self.event.as_ref() {
            match v {
                notification_event::Event::Notification(v) => {
                    struct_ser.serialize_field("notification", v)?;
                }
                notification_event::Event::Updated(v) => {
                    struct_ser.serialize_field("updated", v)?;
                }
                notification_event::Event::Read(v) => {
                    struct_ser.serialize_field("read", v)?;
                }
                notification_event::Event::Dismissed(v) => {
                    struct_ser.serialize_field("dismissed", v)?;
                }
                notification_event::Event::CountChanged(v) => {
                    struct_ser.serialize_field("countChanged", v)?;
                }
                notification_event::Event::Progress(v) => {
                    struct_ser.serialize_field("progress", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NotificationEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "notification",
            "updated",
            "read",
            "dismissed",
            "count_changed",
            "countChanged",
            "progress",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Notification,
            Updated,
            Read,
            Dismissed,
            CountChanged,
            Progress,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "notification" => Ok(GeneratedField::Notification),
                            "updated" => Ok(GeneratedField::Updated),
                            "read" => Ok(GeneratedField::Read),
                            "dismissed" => Ok(GeneratedField::Dismissed),
                            "countChanged" | "count_changed" => Ok(GeneratedField::CountChanged),
                            "progress" => Ok(GeneratedField::Progress),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NotificationEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct notification.v1.NotificationEvent")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<NotificationEvent, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut event__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Notification => {
                            if event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notification"));
                            }
                            event__ = map_.next_value::<::std::option::Option<_>>()?.map(notification_event::Event::Notification)
;
                        }
                        GeneratedField::Updated => {
                            if event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updated"));
                            }
                            event__ = map_.next_value::<::std::option::Option<_>>()?.map(notification_event::Event::Updated)
;
                        }
                        GeneratedField::Read => {
                            if event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("read"));
                            }
                            event__ = map_.next_value::<::std::option::Option<_>>()?.map(notification_event::Event::Read)
;
                        }
                        GeneratedField::Dismissed => {
                            if event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dismissed"));
                            }
                            event__ = map_.next_value::<::std::option::Option<_>>()?.map(notification_event::Event::Dismissed)
;
                        }
                        GeneratedField::CountChanged => {
                            if event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("countChanged"));
                            }
                            event__ = map_.next_value::<::std::option::Option<_>>()?.map(notification_event::Event::CountChanged)
;
                        }
                        GeneratedField::Progress => {
                            if event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("progress"));
                            }
                            event__ = map_.next_value::<::std::option::Option<_>>()?.map(notification_event::Event::Progress)
;
                        }
                    }
                }
                Ok(NotificationEvent {
                    event: event__,
                })
            }
        }
        deserializer.deserialize_struct("notification.v1.NotificationEvent", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NotificationPreferences {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.push_enabled {
            len += 1;
        }
        if self.email_enabled {
            len += 1;
        }
        if self.sound_enabled {
            len += 1;
        }
        if !self.category_preferences.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("notification.v1.NotificationPreferences", len)?;
        if self.push_enabled {
            struct_ser.serialize_field("pushEnabled", &self.push_enabled)?;
        }
        if self.email_enabled {
            struct_ser.serialize_field("emailEnabled", &self.email_enabled)?;
        }
        if self.sound_enabled {
            struct_ser.serialize_field("soundEnabled", &self.sound_enabled)?;
        }
        if !self.category_preferences.is_empty() {
            struct_ser.serialize_field("categoryPreferences", &self.category_preferences)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NotificationPreferences {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "push_enabled",
            "pushEnabled",
            "email_enabled",
            "emailEnabled",
            "sound_enabled",
            "soundEnabled",
            "category_preferences",
            "categoryPreferences",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PushEnabled,
            EmailEnabled,
            SoundEnabled,
            CategoryPreferences,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "pushEnabled" | "push_enabled" => Ok(GeneratedField::PushEnabled),
                            "emailEnabled" | "email_enabled" => Ok(GeneratedField::EmailEnabled),
                            "soundEnabled" | "sound_enabled" => Ok(GeneratedField::SoundEnabled),
                            "categoryPreferences" | "category_preferences" => Ok(GeneratedField::CategoryPreferences),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NotificationPreferences;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct notification.v1.NotificationPreferences")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<NotificationPreferences, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut push_enabled__ = None;
                let mut email_enabled__ = None;
                let mut sound_enabled__ = None;
                let mut category_preferences__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PushEnabled => {
                            if push_enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pushEnabled"));
                            }
                            push_enabled__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EmailEnabled => {
                            if email_enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("emailEnabled"));
                            }
                            email_enabled__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SoundEnabled => {
                            if sound_enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("soundEnabled"));
                            }
                            sound_enabled__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CategoryPreferences => {
                            if category_preferences__.is_some() {
                                return Err(serde::de::Error::duplicate_field("categoryPreferences"));
                            }
                            category_preferences__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(NotificationPreferences {
                    push_enabled: push_enabled__.unwrap_or_default(),
                    email_enabled: email_enabled__.unwrap_or_default(),
                    sound_enabled: sound_enabled__.unwrap_or_default(),
                    category_preferences: category_preferences__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("notification.v1.NotificationPreferences", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NotificationPriority {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "NOTIFICATION_PRIORITY_UNSPECIFIED",
            Self::Low => "NOTIFICATION_PRIORITY_LOW",
            Self::Normal => "NOTIFICATION_PRIORITY_NORMAL",
            Self::High => "NOTIFICATION_PRIORITY_HIGH",
            Self::Urgent => "NOTIFICATION_PRIORITY_URGENT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for NotificationPriority {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "NOTIFICATION_PRIORITY_UNSPECIFIED",
            "NOTIFICATION_PRIORITY_LOW",
            "NOTIFICATION_PRIORITY_NORMAL",
            "NOTIFICATION_PRIORITY_HIGH",
            "NOTIFICATION_PRIORITY_URGENT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NotificationPriority;

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
                    "NOTIFICATION_PRIORITY_UNSPECIFIED" => Ok(NotificationPriority::Unspecified),
                    "NOTIFICATION_PRIORITY_LOW" => Ok(NotificationPriority::Low),
                    "NOTIFICATION_PRIORITY_NORMAL" => Ok(NotificationPriority::Normal),
                    "NOTIFICATION_PRIORITY_HIGH" => Ok(NotificationPriority::High),
                    "NOTIFICATION_PRIORITY_URGENT" => Ok(NotificationPriority::Urgent),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for NotificationProgress {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.notification_pid.is_empty() {
            len += 1;
        }
        if self.title.is_some() {
            len += 1;
        }
        if self.description.is_some() {
            len += 1;
        }
        if self.icon_url.is_some() {
            len += 1;
        }
        if self.priority.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("notification.v1.NotificationProgress", len)?;
        if !self.notification_pid.is_empty() {
            struct_ser.serialize_field("notificationPid", &self.notification_pid)?;
        }
        if let Some(v) = self.title.as_ref() {
            struct_ser.serialize_field("title", v)?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        if let Some(v) = self.icon_url.as_ref() {
            struct_ser.serialize_field("iconUrl", v)?;
        }
        if let Some(v) = self.priority.as_ref() {
            let v = NotificationPriority::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("priority", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NotificationProgress {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "notification_pid",
            "notificationPid",
            "title",
            "description",
            "icon_url",
            "iconUrl",
            "priority",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NotificationPid,
            Title,
            Description,
            IconUrl,
            Priority,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "notificationPid" | "notification_pid" => Ok(GeneratedField::NotificationPid),
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "iconUrl" | "icon_url" => Ok(GeneratedField::IconUrl),
                            "priority" => Ok(GeneratedField::Priority),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NotificationProgress;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct notification.v1.NotificationProgress")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<NotificationProgress, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut notification_pid__ = None;
                let mut title__ = None;
                let mut description__ = None;
                let mut icon_url__ = None;
                let mut priority__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::NotificationPid => {
                            if notification_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notificationPid"));
                            }
                            notification_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = map_.next_value()?;
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
                        GeneratedField::Priority => {
                            if priority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priority"));
                            }
                            priority__ = map_.next_value::<::std::option::Option<NotificationPriority>>()?.map(|x| x as i32);
                        }
                    }
                }
                Ok(NotificationProgress {
                    notification_pid: notification_pid__.unwrap_or_default(),
                    title: title__,
                    description: description__,
                    icon_url: icon_url__,
                    priority: priority__,
                })
            }
        }
        deserializer.deserialize_struct("notification.v1.NotificationProgress", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NotificationRead {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.notification_pid.is_empty() {
            len += 1;
        }
        if !self.read_at.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("notification.v1.NotificationRead", len)?;
        if !self.notification_pid.is_empty() {
            struct_ser.serialize_field("notificationPid", &self.notification_pid)?;
        }
        if !self.read_at.is_empty() {
            struct_ser.serialize_field("readAt", &self.read_at)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NotificationRead {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "notification_pid",
            "notificationPid",
            "read_at",
            "readAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NotificationPid,
            ReadAt,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "notificationPid" | "notification_pid" => Ok(GeneratedField::NotificationPid),
                            "readAt" | "read_at" => Ok(GeneratedField::ReadAt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NotificationRead;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct notification.v1.NotificationRead")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<NotificationRead, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut notification_pid__ = None;
                let mut read_at__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::NotificationPid => {
                            if notification_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notificationPid"));
                            }
                            notification_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ReadAt => {
                            if read_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("readAt"));
                            }
                            read_at__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(NotificationRead {
                    notification_pid: notification_pid__.unwrap_or_default(),
                    read_at: read_at__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("notification.v1.NotificationRead", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NotificationUpdated {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.notification.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("notification.v1.NotificationUpdated", len)?;
        if let Some(v) = self.notification.as_ref() {
            struct_ser.serialize_field("notification", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NotificationUpdated {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "notification",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Notification,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "notification" => Ok(GeneratedField::Notification),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NotificationUpdated;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct notification.v1.NotificationUpdated")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<NotificationUpdated, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut notification__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Notification => {
                            if notification__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notification"));
                            }
                            notification__ = map_.next_value()?;
                        }
                    }
                }
                Ok(NotificationUpdated {
                    notification: notification__,
                })
            }
        }
        deserializer.deserialize_struct("notification.v1.NotificationUpdated", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OriginEntityType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "ORIGIN_ENTITY_TYPE_UNSPECIFIED",
            Self::Channel => "ORIGIN_ENTITY_TYPE_CHANNEL",
            Self::Message => "ORIGIN_ENTITY_TYPE_MESSAGE",
            Self::Agent => "ORIGIN_ENTITY_TYPE_AGENT",
            Self::Run => "ORIGIN_ENTITY_TYPE_RUN",
            Self::Workflow => "ORIGIN_ENTITY_TYPE_WORKFLOW",
            Self::User => "ORIGIN_ENTITY_TYPE_USER",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for OriginEntityType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ORIGIN_ENTITY_TYPE_UNSPECIFIED",
            "ORIGIN_ENTITY_TYPE_CHANNEL",
            "ORIGIN_ENTITY_TYPE_MESSAGE",
            "ORIGIN_ENTITY_TYPE_AGENT",
            "ORIGIN_ENTITY_TYPE_RUN",
            "ORIGIN_ENTITY_TYPE_WORKFLOW",
            "ORIGIN_ENTITY_TYPE_USER",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OriginEntityType;

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
                    "ORIGIN_ENTITY_TYPE_UNSPECIFIED" => Ok(OriginEntityType::Unspecified),
                    "ORIGIN_ENTITY_TYPE_CHANNEL" => Ok(OriginEntityType::Channel),
                    "ORIGIN_ENTITY_TYPE_MESSAGE" => Ok(OriginEntityType::Message),
                    "ORIGIN_ENTITY_TYPE_AGENT" => Ok(OriginEntityType::Agent),
                    "ORIGIN_ENTITY_TYPE_RUN" => Ok(OriginEntityType::Run),
                    "ORIGIN_ENTITY_TYPE_WORKFLOW" => Ok(OriginEntityType::Workflow),
                    "ORIGIN_ENTITY_TYPE_USER" => Ok(OriginEntityType::User),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for OriginService {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "ORIGIN_SERVICE_UNSPECIFIED",
            Self::Api => "ORIGIN_SERVICE_API",
            Self::Chat => "ORIGIN_SERVICE_CHAT",
            Self::Rig => "ORIGIN_SERVICE_RIG",
            Self::Agent => "ORIGIN_SERVICE_AGENT",
            Self::Notification => "ORIGIN_SERVICE_NOTIFICATION",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for OriginService {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ORIGIN_SERVICE_UNSPECIFIED",
            "ORIGIN_SERVICE_API",
            "ORIGIN_SERVICE_CHAT",
            "ORIGIN_SERVICE_RIG",
            "ORIGIN_SERVICE_AGENT",
            "ORIGIN_SERVICE_NOTIFICATION",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OriginService;

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
                    "ORIGIN_SERVICE_UNSPECIFIED" => Ok(OriginService::Unspecified),
                    "ORIGIN_SERVICE_API" => Ok(OriginService::Api),
                    "ORIGIN_SERVICE_CHAT" => Ok(OriginService::Chat),
                    "ORIGIN_SERVICE_RIG" => Ok(OriginService::Rig),
                    "ORIGIN_SERVICE_AGENT" => Ok(OriginService::Agent),
                    "ORIGIN_SERVICE_NOTIFICATION" => Ok(OriginService::Notification),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for PushBatchRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.notifications.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("notification.v1.PushBatchRequest", len)?;
        if !self.notifications.is_empty() {
            struct_ser.serialize_field("notifications", &self.notifications)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PushBatchRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "notifications",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = PushBatchRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct notification.v1.PushBatchRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PushBatchRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut notifications__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Notifications => {
                            if notifications__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notifications"));
                            }
                            notifications__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PushBatchRequest {
                    notifications: notifications__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("notification.v1.PushBatchRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PushBatchResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.created_count != 0 {
            len += 1;
        }
        if self.deduplicated_count != 0 {
            len += 1;
        }
        if !self.notification_pids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("notification.v1.PushBatchResponse", len)?;
        if self.created_count != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("createdCount", ToString::to_string(&self.created_count).as_str())?;
        }
        if self.deduplicated_count != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("deduplicatedCount", ToString::to_string(&self.deduplicated_count).as_str())?;
        }
        if !self.notification_pids.is_empty() {
            struct_ser.serialize_field("notificationPids", &self.notification_pids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PushBatchResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "created_count",
            "createdCount",
            "deduplicated_count",
            "deduplicatedCount",
            "notification_pids",
            "notificationPids",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CreatedCount,
            DeduplicatedCount,
            NotificationPids,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "createdCount" | "created_count" => Ok(GeneratedField::CreatedCount),
                            "deduplicatedCount" | "deduplicated_count" => Ok(GeneratedField::DeduplicatedCount),
                            "notificationPids" | "notification_pids" => Ok(GeneratedField::NotificationPids),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PushBatchResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct notification.v1.PushBatchResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PushBatchResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut created_count__ = None;
                let mut deduplicated_count__ = None;
                let mut notification_pids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CreatedCount => {
                            if created_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdCount"));
                            }
                            created_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::DeduplicatedCount => {
                            if deduplicated_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deduplicatedCount"));
                            }
                            deduplicated_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::NotificationPids => {
                            if notification_pids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notificationPids"));
                            }
                            notification_pids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PushBatchResponse {
                    created_count: created_count__.unwrap_or_default(),
                    deduplicated_count: deduplicated_count__.unwrap_or_default(),
                    notification_pids: notification_pids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("notification.v1.PushBatchResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PushRequest {
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
        if !self.org_pid.is_empty() {
            len += 1;
        }
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if self.icon_url.is_some() {
            len += 1;
        }
        if self.category != 0 {
            len += 1;
        }
        if self.priority != 0 {
            len += 1;
        }
        if !self.actions.is_empty() {
            len += 1;
        }
        if self.origin_service != 0 {
            len += 1;
        }
        if self.origin_entity_type.is_some() {
            len += 1;
        }
        if self.origin_entity_pid.is_some() {
            len += 1;
        }
        if self.expires_at.is_some() {
            len += 1;
        }
        if self.idempotency_key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("notification.v1.PushRequest", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if !self.org_pid.is_empty() {
            struct_ser.serialize_field("orgPid", &self.org_pid)?;
        }
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if let Some(v) = self.icon_url.as_ref() {
            struct_ser.serialize_field("iconUrl", v)?;
        }
        if self.category != 0 {
            let v = NotificationCategory::try_from(self.category)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.category)))?;
            struct_ser.serialize_field("category", &v)?;
        }
        if self.priority != 0 {
            let v = NotificationPriority::try_from(self.priority)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.priority)))?;
            struct_ser.serialize_field("priority", &v)?;
        }
        if !self.actions.is_empty() {
            struct_ser.serialize_field("actions", &self.actions)?;
        }
        if self.origin_service != 0 {
            let v = OriginService::try_from(self.origin_service)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.origin_service)))?;
            struct_ser.serialize_field("originService", &v)?;
        }
        if let Some(v) = self.origin_entity_type.as_ref() {
            let v = OriginEntityType::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("originEntityType", &v)?;
        }
        if let Some(v) = self.origin_entity_pid.as_ref() {
            struct_ser.serialize_field("originEntityPid", v)?;
        }
        if let Some(v) = self.expires_at.as_ref() {
            struct_ser.serialize_field("expiresAt", v)?;
        }
        if let Some(v) = self.idempotency_key.as_ref() {
            struct_ser.serialize_field("idempotencyKey", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PushRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
            "org_pid",
            "orgPid",
            "title",
            "description",
            "icon_url",
            "iconUrl",
            "category",
            "priority",
            "actions",
            "origin_service",
            "originService",
            "origin_entity_type",
            "originEntityType",
            "origin_entity_pid",
            "originEntityPid",
            "expires_at",
            "expiresAt",
            "idempotency_key",
            "idempotencyKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
            OrgPid,
            Title,
            Description,
            IconUrl,
            Category,
            Priority,
            Actions,
            OriginService,
            OriginEntityType,
            OriginEntityPid,
            ExpiresAt,
            IdempotencyKey,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "orgPid" | "org_pid" => Ok(GeneratedField::OrgPid),
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "iconUrl" | "icon_url" => Ok(GeneratedField::IconUrl),
                            "category" => Ok(GeneratedField::Category),
                            "priority" => Ok(GeneratedField::Priority),
                            "actions" => Ok(GeneratedField::Actions),
                            "originService" | "origin_service" => Ok(GeneratedField::OriginService),
                            "originEntityType" | "origin_entity_type" => Ok(GeneratedField::OriginEntityType),
                            "originEntityPid" | "origin_entity_pid" => Ok(GeneratedField::OriginEntityPid),
                            "expiresAt" | "expires_at" => Ok(GeneratedField::ExpiresAt),
                            "idempotencyKey" | "idempotency_key" => Ok(GeneratedField::IdempotencyKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PushRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct notification.v1.PushRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PushRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut org_pid__ = None;
                let mut title__ = None;
                let mut description__ = None;
                let mut icon_url__ = None;
                let mut category__ = None;
                let mut priority__ = None;
                let mut actions__ = None;
                let mut origin_service__ = None;
                let mut origin_entity_type__ = None;
                let mut origin_entity_pid__ = None;
                let mut expires_at__ = None;
                let mut idempotency_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrgPid => {
                            if org_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orgPid"));
                            }
                            org_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IconUrl => {
                            if icon_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("iconUrl"));
                            }
                            icon_url__ = map_.next_value()?;
                        }
                        GeneratedField::Category => {
                            if category__.is_some() {
                                return Err(serde::de::Error::duplicate_field("category"));
                            }
                            category__ = Some(map_.next_value::<NotificationCategory>()? as i32);
                        }
                        GeneratedField::Priority => {
                            if priority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priority"));
                            }
                            priority__ = Some(map_.next_value::<NotificationPriority>()? as i32);
                        }
                        GeneratedField::Actions => {
                            if actions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actions"));
                            }
                            actions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OriginService => {
                            if origin_service__.is_some() {
                                return Err(serde::de::Error::duplicate_field("originService"));
                            }
                            origin_service__ = Some(map_.next_value::<OriginService>()? as i32);
                        }
                        GeneratedField::OriginEntityType => {
                            if origin_entity_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("originEntityType"));
                            }
                            origin_entity_type__ = map_.next_value::<::std::option::Option<OriginEntityType>>()?.map(|x| x as i32);
                        }
                        GeneratedField::OriginEntityPid => {
                            if origin_entity_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("originEntityPid"));
                            }
                            origin_entity_pid__ = map_.next_value()?;
                        }
                        GeneratedField::ExpiresAt => {
                            if expires_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expiresAt"));
                            }
                            expires_at__ = map_.next_value()?;
                        }
                        GeneratedField::IdempotencyKey => {
                            if idempotency_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("idempotencyKey"));
                            }
                            idempotency_key__ = map_.next_value()?;
                        }
                    }
                }
                Ok(PushRequest {
                    user_id: user_id__.unwrap_or_default(),
                    org_pid: org_pid__.unwrap_or_default(),
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    icon_url: icon_url__,
                    category: category__.unwrap_or_default(),
                    priority: priority__.unwrap_or_default(),
                    actions: actions__.unwrap_or_default(),
                    origin_service: origin_service__.unwrap_or_default(),
                    origin_entity_type: origin_entity_type__,
                    origin_entity_pid: origin_entity_pid__,
                    expires_at: expires_at__,
                    idempotency_key: idempotency_key__,
                })
            }
        }
        deserializer.deserialize_struct("notification.v1.PushRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PushResponse {
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
        if !self.notification_pid.is_empty() {
            len += 1;
        }
        if self.deduplicated {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("notification.v1.PushResponse", len)?;
        if self.id != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("id", ToString::to_string(&self.id).as_str())?;
        }
        if !self.notification_pid.is_empty() {
            struct_ser.serialize_field("notificationPid", &self.notification_pid)?;
        }
        if self.deduplicated {
            struct_ser.serialize_field("deduplicated", &self.deduplicated)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PushResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "notification_pid",
            "notificationPid",
            "deduplicated",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            NotificationPid,
            Deduplicated,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "notificationPid" | "notification_pid" => Ok(GeneratedField::NotificationPid),
                            "deduplicated" => Ok(GeneratedField::Deduplicated),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PushResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct notification.v1.PushResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PushResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut notification_pid__ = None;
                let mut deduplicated__ = None;
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
                        GeneratedField::NotificationPid => {
                            if notification_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notificationPid"));
                            }
                            notification_pid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Deduplicated => {
                            if deduplicated__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deduplicated"));
                            }
                            deduplicated__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PushResponse {
                    id: id__.unwrap_or_default(),
                    notification_pid: notification_pid__.unwrap_or_default(),
                    deduplicated: deduplicated__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("notification.v1.PushResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PushStreamMessage {
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
        let mut struct_ser = serializer.serialize_struct("notification.v1.PushStreamMessage", len)?;
        if let Some(v) = self.message.as_ref() {
            match v {
                push_stream_message::Message::Init(v) => {
                    struct_ser.serialize_field("init", v)?;
                }
                push_stream_message::Message::Update(v) => {
                    struct_ser.serialize_field("update", v)?;
                }
                push_stream_message::Message::Complete(v) => {
                    struct_ser.serialize_field("complete", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PushStreamMessage {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "init",
            "update",
            "complete",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Init,
            Update,
            Complete,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "init" => Ok(GeneratedField::Init),
                            "update" => Ok(GeneratedField::Update),
                            "complete" => Ok(GeneratedField::Complete),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PushStreamMessage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct notification.v1.PushStreamMessage")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PushStreamMessage, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut message__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Init => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("init"));
                            }
                            message__ = map_.next_value::<::std::option::Option<_>>()?.map(push_stream_message::Message::Init)
;
                        }
                        GeneratedField::Update => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("update"));
                            }
                            message__ = map_.next_value::<::std::option::Option<_>>()?.map(push_stream_message::Message::Update)
;
                        }
                        GeneratedField::Complete => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("complete"));
                            }
                            message__ = map_.next_value::<::std::option::Option<_>>()?.map(push_stream_message::Message::Complete)
;
                        }
                    }
                }
                Ok(PushStreamMessage {
                    message: message__,
                })
            }
        }
        deserializer.deserialize_struct("notification.v1.PushStreamMessage", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PushStreamResponse {
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
        if !self.notification_pid.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("notification.v1.PushStreamResponse", len)?;
        if self.id != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("id", ToString::to_string(&self.id).as_str())?;
        }
        if !self.notification_pid.is_empty() {
            struct_ser.serialize_field("notificationPid", &self.notification_pid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PushStreamResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "notification_pid",
            "notificationPid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            NotificationPid,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "notificationPid" | "notification_pid" => Ok(GeneratedField::NotificationPid),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PushStreamResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct notification.v1.PushStreamResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PushStreamResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut notification_pid__ = None;
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
                        GeneratedField::NotificationPid => {
                            if notification_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notificationPid"));
                            }
                            notification_pid__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PushStreamResponse {
                    id: id__.unwrap_or_default(),
                    notification_pid: notification_pid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("notification.v1.PushStreamResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamComplete {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.actions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("notification.v1.StreamComplete", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.actions.is_empty() {
            struct_ser.serialize_field("actions", &self.actions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamComplete {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "actions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            Actions,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "actions" => Ok(GeneratedField::Actions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamComplete;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct notification.v1.StreamComplete")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StreamComplete, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut actions__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Actions => {
                            if actions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actions"));
                            }
                            actions__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(StreamComplete {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    actions: actions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("notification.v1.StreamComplete", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamUpdate {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.title.is_some() {
            len += 1;
        }
        if self.description.is_some() {
            len += 1;
        }
        if self.icon_url.is_some() {
            len += 1;
        }
        if self.priority.is_some() {
            len += 1;
        }
        if !self.actions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("notification.v1.StreamUpdate", len)?;
        if let Some(v) = self.title.as_ref() {
            struct_ser.serialize_field("title", v)?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        if let Some(v) = self.icon_url.as_ref() {
            struct_ser.serialize_field("iconUrl", v)?;
        }
        if let Some(v) = self.priority.as_ref() {
            let v = NotificationPriority::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("priority", &v)?;
        }
        if !self.actions.is_empty() {
            struct_ser.serialize_field("actions", &self.actions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamUpdate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "icon_url",
            "iconUrl",
            "priority",
            "actions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            IconUrl,
            Priority,
            Actions,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "iconUrl" | "icon_url" => Ok(GeneratedField::IconUrl),
                            "priority" => Ok(GeneratedField::Priority),
                            "actions" => Ok(GeneratedField::Actions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamUpdate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct notification.v1.StreamUpdate")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StreamUpdate, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut icon_url__ = None;
                let mut priority__ = None;
                let mut actions__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = map_.next_value()?;
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
                        GeneratedField::Priority => {
                            if priority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priority"));
                            }
                            priority__ = map_.next_value::<::std::option::Option<NotificationPriority>>()?.map(|x| x as i32);
                        }
                        GeneratedField::Actions => {
                            if actions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actions"));
                            }
                            actions__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(StreamUpdate {
                    title: title__,
                    description: description__,
                    icon_url: icon_url__,
                    priority: priority__,
                    actions: actions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("notification.v1.StreamUpdate", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SubscribeRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.categories.is_empty() {
            len += 1;
        }
        if self.min_priority.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("notification.v1.SubscribeRequest", len)?;
        if !self.categories.is_empty() {
            let v = self.categories.iter().cloned().map(|v| {
                NotificationCategory::try_from(v)
                    .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", v)))
                }).collect::<std::result::Result<Vec<_>, _>>()?;
            struct_ser.serialize_field("categories", &v)?;
        }
        if let Some(v) = self.min_priority.as_ref() {
            let v = NotificationPriority::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("minPriority", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SubscribeRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "categories",
            "min_priority",
            "minPriority",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Categories,
            MinPriority,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "categories" => Ok(GeneratedField::Categories),
                            "minPriority" | "min_priority" => Ok(GeneratedField::MinPriority),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SubscribeRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct notification.v1.SubscribeRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SubscribeRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut categories__ = None;
                let mut min_priority__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Categories => {
                            if categories__.is_some() {
                                return Err(serde::de::Error::duplicate_field("categories"));
                            }
                            categories__ = Some(map_.next_value::<Vec<NotificationCategory>>()?.into_iter().map(|x| x as i32).collect());
                        }
                        GeneratedField::MinPriority => {
                            if min_priority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minPriority"));
                            }
                            min_priority__ = map_.next_value::<::std::option::Option<NotificationPriority>>()?.map(|x| x as i32);
                        }
                    }
                }
                Ok(SubscribeRequest {
                    categories: categories__.unwrap_or_default(),
                    min_priority: min_priority__,
                })
            }
        }
        deserializer.deserialize_struct("notification.v1.SubscribeRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SystemAlertContext {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.alert_type.is_empty() {
            len += 1;
        }
        if !self.details.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("notification.v1.SystemAlertContext", len)?;
        if !self.alert_type.is_empty() {
            struct_ser.serialize_field("alertType", &self.alert_type)?;
        }
        if !self.details.is_empty() {
            struct_ser.serialize_field("details", &self.details)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SystemAlertContext {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "alert_type",
            "alertType",
            "details",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AlertType,
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
                            "alertType" | "alert_type" => Ok(GeneratedField::AlertType),
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
            type Value = SystemAlertContext;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct notification.v1.SystemAlertContext")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SystemAlertContext, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut alert_type__ = None;
                let mut details__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AlertType => {
                            if alert_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("alertType"));
                            }
                            alert_type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(SystemAlertContext {
                    alert_type: alert_type__.unwrap_or_default(),
                    details: details__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("notification.v1.SystemAlertContext", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UnreadCountChanged {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.total_unread != 0 {
            len += 1;
        }
        if !self.by_category.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("notification.v1.UnreadCountChanged", len)?;
        if self.total_unread != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("totalUnread", ToString::to_string(&self.total_unread).as_str())?;
        }
        if !self.by_category.is_empty() {
            let v: std::collections::HashMap<_, _> = self.by_category.iter()
                .map(|(k, v)| (k, v.to_string())).collect();
            struct_ser.serialize_field("byCategory", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UnreadCountChanged {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "total_unread",
            "totalUnread",
            "by_category",
            "byCategory",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TotalUnread,
            ByCategory,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "totalUnread" | "total_unread" => Ok(GeneratedField::TotalUnread),
                            "byCategory" | "by_category" => Ok(GeneratedField::ByCategory),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UnreadCountChanged;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct notification.v1.UnreadCountChanged")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UnreadCountChanged, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut total_unread__ = None;
                let mut by_category__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TotalUnread => {
                            if total_unread__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalUnread"));
                            }
                            total_unread__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ByCategory => {
                            if by_category__.is_some() {
                                return Err(serde::de::Error::duplicate_field("byCategory"));
                            }
                            by_category__ = Some(
                                map_.next_value::<std::collections::HashMap<_, ::pbjson::private::NumberDeserialize<u64>>>()?
                                    .into_iter().map(|(k,v)| (k, v.0)).collect()
                            );
                        }
                    }
                }
                Ok(UnreadCountChanged {
                    total_unread: total_unread__.unwrap_or_default(),
                    by_category: by_category__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("notification.v1.UnreadCountChanged", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdatePreferencesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.preferences.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("notification.v1.UpdatePreferencesRequest", len)?;
        if let Some(v) = self.preferences.as_ref() {
            struct_ser.serialize_field("preferences", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdatePreferencesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "preferences",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Preferences,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "preferences" => Ok(GeneratedField::Preferences),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdatePreferencesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct notification.v1.UpdatePreferencesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdatePreferencesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut preferences__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Preferences => {
                            if preferences__.is_some() {
                                return Err(serde::de::Error::duplicate_field("preferences"));
                            }
                            preferences__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdatePreferencesRequest {
                    preferences: preferences__,
                })
            }
        }
        deserializer.deserialize_struct("notification.v1.UpdatePreferencesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdatePreferencesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.preferences.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("notification.v1.UpdatePreferencesResponse", len)?;
        if let Some(v) = self.preferences.as_ref() {
            struct_ser.serialize_field("preferences", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdatePreferencesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "preferences",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Preferences,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "preferences" => Ok(GeneratedField::Preferences),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdatePreferencesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct notification.v1.UpdatePreferencesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdatePreferencesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut preferences__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Preferences => {
                            if preferences__.is_some() {
                                return Err(serde::de::Error::duplicate_field("preferences"));
                            }
                            preferences__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdatePreferencesResponse {
                    preferences: preferences__,
                })
            }
        }
        deserializer.deserialize_struct("notification.v1.UpdatePreferencesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateRequest {
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
        if self.title.is_some() {
            len += 1;
        }
        if self.description.is_some() {
            len += 1;
        }
        if self.icon_url.is_some() {
            len += 1;
        }
        if self.priority.is_some() {
            len += 1;
        }
        if !self.actions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("notification.v1.UpdateRequest", len)?;
        if self.id != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("id", ToString::to_string(&self.id).as_str())?;
        }
        if let Some(v) = self.title.as_ref() {
            struct_ser.serialize_field("title", v)?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        if let Some(v) = self.icon_url.as_ref() {
            struct_ser.serialize_field("iconUrl", v)?;
        }
        if let Some(v) = self.priority.as_ref() {
            let v = NotificationPriority::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("priority", &v)?;
        }
        if !self.actions.is_empty() {
            struct_ser.serialize_field("actions", &self.actions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "title",
            "description",
            "icon_url",
            "iconUrl",
            "priority",
            "actions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Title,
            Description,
            IconUrl,
            Priority,
            Actions,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "iconUrl" | "icon_url" => Ok(GeneratedField::IconUrl),
                            "priority" => Ok(GeneratedField::Priority),
                            "actions" => Ok(GeneratedField::Actions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct notification.v1.UpdateRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut title__ = None;
                let mut description__ = None;
                let mut icon_url__ = None;
                let mut priority__ = None;
                let mut actions__ = None;
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
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = map_.next_value()?;
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
                        GeneratedField::Priority => {
                            if priority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priority"));
                            }
                            priority__ = map_.next_value::<::std::option::Option<NotificationPriority>>()?.map(|x| x as i32);
                        }
                        GeneratedField::Actions => {
                            if actions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actions"));
                            }
                            actions__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UpdateRequest {
                    id: id__.unwrap_or_default(),
                    title: title__,
                    description: description__,
                    icon_url: icon_url__,
                    priority: priority__,
                    actions: actions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("notification.v1.UpdateRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.notification.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("notification.v1.UpdateResponse", len)?;
        if let Some(v) = self.notification.as_ref() {
            struct_ser.serialize_field("notification", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "notification",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Notification,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "notification" => Ok(GeneratedField::Notification),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct notification.v1.UpdateResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut notification__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Notification => {
                            if notification__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notification"));
                            }
                            notification__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateResponse {
                    notification: notification__,
                })
            }
        }
        deserializer.deserialize_struct("notification.v1.UpdateResponse", FIELDS, GeneratedVisitor)
    }
}
