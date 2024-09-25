pub fn to_concrete_metadata(metadata: Option<String>) -> String {
    metadata
        .as_ref()
        .map(|s| s.clone())
        .unwrap_or("{}".to_string())
}
