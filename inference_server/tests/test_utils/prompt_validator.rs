use super::OpenaiClient;

#[allow(dead_code)]
pub async fn yes_or_no(yes_or_no_question: String) -> bool {
    let client = OpenaiClient::new();

    let prompt: String = format!("ALWAYS answer by yes or no. {}", yes_or_no_question);
    let response = client.user_invoke("gpt-4o-mini", prompt.as_str()).await;

    let str_response = response.choices[0]
        .message
        .content
        .clone()
        .unwrap_or("".to_string())
        .to_lowercase();

    if str_response.contains("yes") {
        return true;
    } else if str_response.contains("no") {
        return false;
    } else {
        panic!("Unexpected response: {}", str_response)
    }
}
