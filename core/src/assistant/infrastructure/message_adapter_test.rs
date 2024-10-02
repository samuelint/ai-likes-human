#[cfg(test)]
mod test_from_entity_model_to_dto {
    use crate::{
        assistant::domain::dto::ThreadMessageDto, chat_completion::ApiMessageContent,
        entities::message, utils::time::TimeBuilder,
    };
    use pretty_assertions::assert_eq;

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
        let content: Vec<ApiMessageContent> = vec![ApiMessageContent::text("hello")];
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

#[cfg(test)]
mod test_from_db_dto_to_entity_model {
    use crate::{
        assistant::domain::dto::{DbCreateThreadMessageDto, DbMessageContent},
        chat_completion::ApiMessageContent,
        entities::message,
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn test_status_is_the_same() {
        let dto = DbCreateThreadMessageDto {
            status: "in_progress".to_string(),
            ..DbCreateThreadMessageDto::default()
        };

        let model: message::ActiveModel = (&dto).into();

        let value = match model.status.into_value().unwrap() {
            sea_orm::Value::String(s) => s,
            _ => panic!("Should be a string"),
        };
        let value = value.unwrap();

        assert_eq!(value.as_ref(), "in_progress");
    }

    #[test]
    fn test_empty_content_is_empty_json_vec() {
        let dto = DbCreateThreadMessageDto {
            content: vec![],
            ..DbCreateThreadMessageDto::default()
        };

        let model: message::ActiveModel = (&dto).into();

        let value = match model.content.into_value().unwrap() {
            sea_orm::Value::String(s) => s,
            _ => panic!("Should be a string"),
        };
        let value = value.unwrap();

        assert_eq!(value.as_ref(), "[]");
    }

    #[test]
    fn test_content_is_serialized_to_json() {
        let dto = DbCreateThreadMessageDto {
            content: vec![DbMessageContent::text_annotated("hello")],
            ..DbCreateThreadMessageDto::default()
        };

        let model: message::ActiveModel = (&dto).into();

        let value = match model.content.into_value().unwrap() {
            sea_orm::Value::String(s) => s,
            _ => panic!("Should be a string"),
        };
        let value = value.unwrap();
        let expected_value =
            serde_json::to_string(&vec![ApiMessageContent::text("hello")]).unwrap();
        assert_eq!(value.as_ref(), &expected_value);
    }
}
