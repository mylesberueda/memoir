use rig::{completion::ToolDefinition, tool::Tool};
use schemars::{JsonSchema, schema_for};

#[derive(Debug)]
pub(crate) struct CurrentTimeTool {}

impl CurrentTimeTool {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, JsonSchema)]
pub(crate) struct CurrentTimeToolArgs {}

impl Tool for CurrentTimeTool {
    const NAME: &'static str = "current_time";
    type Error = CurrentTimeToolError;
    type Args = CurrentTimeToolArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Get the current date and time in ISO8601 format.".to_string(),
            parameters: serde_json::to_value(schema_for!(CurrentTimeToolArgs))
                .expect("schema serialization should not fail"),
        }
    }

    async fn call(&self, _args: Self::Args) -> Result<Self::Output, Self::Error> {
        let time = chrono::Utc::now();
        Ok(time.to_rfc3339())
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum CurrentTimeToolError {
    #[allow(dead_code)]
    #[error("invalid timezone: {0}")]
    InvalidTimezone(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    mod call {
        use super::*;

        #[tokio::test]
        async fn should_return_rfc3339_formatted_timestamp() {
            let tool = CurrentTimeTool::new();
            let result = tool.call(CurrentTimeToolArgs {}).await;

            assert!(result.is_ok(), "should return Ok: {:?}", result.err());

            let time_str = result.unwrap();
            // RFC3339 format: 2024-01-15T10:30:00+00:00 or 2024-01-15T10:30:00Z
            assert!(
                time_str.contains('T'),
                "should contain 'T' separator in RFC3339 format: {time_str}"
            );
            // Should be parseable as RFC3339
            let parsed = chrono::DateTime::parse_from_rfc3339(&time_str);
            assert!(parsed.is_ok(), "should be valid RFC3339 timestamp: {time_str}");
        }

        #[tokio::test]
        async fn should_return_utc_timezone() {
            let tool = CurrentTimeTool::new();
            let result = tool.call(CurrentTimeToolArgs {}).await.unwrap();

            // UTC times end with +00:00 or Z
            assert!(
                result.ends_with("+00:00") || result.ends_with('Z'),
                "should be UTC timezone: {result}"
            );
        }
    }

    mod definition {
        use super::*;

        #[tokio::test]
        async fn should_have_name_current_time() {
            assert_eq!(CurrentTimeTool::NAME, "current_time");
        }

        #[tokio::test]
        async fn should_have_object_type_schema() {
            let tool = CurrentTimeTool::new();
            let definition = tool.definition("".to_string()).await;

            // schemars generates a valid JSON Schema with type: "object"
            assert_eq!(
                definition.parameters.get("type"),
                Some(&serde_json::json!("object")),
                "schema should have type: object"
            );
        }
    }
}
