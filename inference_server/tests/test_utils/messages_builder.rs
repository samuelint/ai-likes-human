use app_core::assistant::domain::dto::{ApiCreateThreadMessageDto, ApiMessageContent};

pub struct MessagesBuilder {
    messages: Vec<ApiCreateThreadMessageDto>,
}

impl MessagesBuilder {
    #[allow(dead_code)]

    pub fn new() -> Self {
        Self { messages: vec![] }
    }

    pub fn from_text(text: &str) -> Self {
        Self {
            messages: vec![ApiCreateThreadMessageDto {
                content: vec![ApiMessageContent::text(text)],
                ..ApiCreateThreadMessageDto::user()
            }],
        }
    }

    pub fn add_image_url(&mut self, image_url: &str) -> &mut Self {
        self.messages.push(ApiCreateThreadMessageDto {
            content: vec![ApiMessageContent::image_url(image_url)],
            ..ApiCreateThreadMessageDto::user()
        });

        self
    }

    pub fn add_text(&mut self, text: &str) -> &mut Self {
        self.messages.push(ApiCreateThreadMessageDto {
            content: vec![ApiMessageContent::text(text)],
            ..ApiCreateThreadMessageDto::user()
        });

        self
    }

    #[allow(dead_code)]
    pub fn build(&self) -> Vec<ApiCreateThreadMessageDto> {
        self.messages.clone()
    }
}

impl From<MessagesBuilder> for Vec<ApiCreateThreadMessageDto> {
    fn from(builder: MessagesBuilder) -> Self {
        builder.build()
    }
}
