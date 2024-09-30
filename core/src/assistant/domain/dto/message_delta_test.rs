#[cfg(test)]
mod test_serialization {
    use crate::assistant::domain::dto::message_delta::{MessageDeltaContent, MessageDeltaDto};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_delta_role_and_empty_content_serialization() {
        let delta = MessageDeltaDto {
            role: "assistant".to_string(),
            content: vec![],
        };

        let serialized = serde_json::to_string(&delta).unwrap();
        assert_eq!(serialized, r#"{"role":"assistant","content":[]}"#);
    }

    #[test]
    fn test_delta_content_serialization() {
        let delta = MessageDeltaDto {
            role: "assistant".to_string(),
            content: vec![MessageDeltaContent::text("Hello")],
        };

        let serialized = serde_json::to_string(&delta).unwrap();
        assert_eq!(
            serialized,
            r#"{"role":"assistant","content":[{"index":0,"type":"text","text":{"value":"Hello","annotations":[]}}]}"#
        );
    }
}
