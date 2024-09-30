#[cfg(test)]
mod test_serialization {
    use serde_json::Map;

    use crate::assistant::domain::dto::{RunDto, ThreadEventDto};

    #[test]
    fn test_run_created_serialization() {
        let run = RunDto {
            id: "123".to_string(),
            ..RunDto::default()
        };
        let event = ThreadEventDto::thread_run_created(&run);
        let serialized = serde_json::to_string(&event.data).unwrap();
        let deserialized: Map<String, serde_json::Value> =
            serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.get("object").unwrap(), "thread.run");
        assert_eq!(deserialized.get("id").unwrap(), "123");
    }
}
