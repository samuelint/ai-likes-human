#[cfg(test)]
mod test_from_messate_entity_model_to_thread_message_dto {
    use crate::{
        assistant::domain::dto::{MessageContent, ThreadMessageDto},
        entities::message,
        utils::time::TimeBuilder,
    };

    #[test]
    fn test_status_is_the_same() {
        let model = message::Model {
            id: 0,
            created_at: TimeBuilder::now().into(),
            content: "".to_string(),
            role: "".to_string(),
            attachments: None,
            metadata: "".to_string(),
            thread_id: None,
            run_id: None,
            status: "in_progress".to_string(),
            assistant_id: None,
        };

        let dto: ThreadMessageDto = model.into();

        assert_eq!(dto.status, "in_progress");
    }

    #[test]
    fn test_thread_id_is_the_same() {
        let model = message::Model {
            id: 0,
            created_at: TimeBuilder::now().into(),
            content: "".to_string(),
            role: "".to_string(),
            attachments: None,
            metadata: "".to_string(),
            thread_id: Some(123),
            run_id: None,
            status: "".to_string(),
            assistant_id: None,
        };

        let dto: ThreadMessageDto = model.into();

        assert_eq!(dto.thread_id.unwrap(), "123");
    }

    #[test]
    fn test_empty_json_content_is_empty_vec() {
        let model = message::Model {
            id: 0,
            created_at: TimeBuilder::now().into(),
            content: "".to_string(),
            role: "".to_string(),
            attachments: None,
            metadata: "".to_string(),
            thread_id: Some(123),
            run_id: None,
            status: "".to_string(),
            assistant_id: None,
        };

        let dto: ThreadMessageDto = model.into();

        assert_eq!(dto.content.len(), 0);
    }

    #[test]
    fn test_json_content_with_empty_array_is_empty_vec() {
        let model = message::Model {
            id: 0,
            created_at: TimeBuilder::now().into(),
            content: "[]".to_string(),
            role: "".to_string(),
            attachments: None,
            metadata: "".to_string(),
            thread_id: Some(123),
            run_id: None,
            status: "".to_string(),
            assistant_id: None,
        };

        let dto: ThreadMessageDto = model.into();

        assert_eq!(dto.content.len(), 0);
    }

    #[test]
    fn test_content_is_deserialized_from_json() {
        let content: Vec<MessageContent> = vec![MessageContent::text("hello")];
        let json_content = serde_json::to_string(&content).unwrap();
        let model = message::Model {
            id: 0,
            created_at: TimeBuilder::now().into(),
            content: json_content,
            role: "".to_string(),
            attachments: None,
            metadata: "".to_string(),
            thread_id: Some(123),
            run_id: None,
            status: "".to_string(),
            assistant_id: None,
        };

        let dto: ThreadMessageDto = model.into();

        assert_eq!(dto.content, content);
    }
}
