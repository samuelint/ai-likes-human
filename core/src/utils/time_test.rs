#[cfg(test)]
mod time_builder_test {
    use chrono::DateTime;

    use crate::utils::time::TimeBuilder;

    #[test]
    fn test_string_time_is_rfc3339() {
        let date_str: String = TimeBuilder::now().into();

        assert!(DateTime::parse_from_rfc3339(&date_str).is_ok());
    }

    #[test]
    fn test_time_conversion() {
        let timestamp: i64 = TimeBuilder::from_string("2023-10-04T14:48:00+00:00").into();
        let date_str: String = TimeBuilder::from_i64(timestamp).into();

        assert_eq!(date_str, "2023-10-04T14:48:00+00:00");
    }

    #[test]
    fn test_is_not_near_now() {
        let time = TimeBuilder::from_string("2023-10-04T14:48:00+00:00");

        assert_eq!(time.is_near_now(), false);
    }

    #[test]
    fn test_is_near_now() {
        let time = TimeBuilder::now();

        assert_eq!(time.is_near_now(), true);
    }
}
