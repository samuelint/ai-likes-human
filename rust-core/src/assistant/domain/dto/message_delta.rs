use serde::{Deserialize, Serialize};

use super::annotation::MessageAnnotation;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MessageContentDelta {
    Text(TextDeltaDto),
    ImageUrl(ImageURLDeltaBlock),
    ImageFile(ImageFileDeltaBlock),
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct TextDeltaDto {
    pub value: Option<String>,
    pub annotations: Vec<MessageAnnotation>,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct TextDeltaBlockDto {
    pub index: i32,
    pub r#type: String,
    pub text: TextDeltaDto,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct ImageURLDeltaBlock {
    pub index: i32,
    pub r#type: String,
    pub image_url: Option<ImageURLDelta>,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct ImageURLDelta {
    pub detail: Option<String>,
    pub url: Option<String>,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct ImageFileDeltaBlock {
    /// The index of the content part in the message.
    pub index: i32,
    /// Always `image_file`.
    pub r#type: String,
    /// The image file detail.
    pub image_file: Option<ImageFileDelta>,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct ImageFileDelta {
    /// Specifies the detail level of the image if specified by the user.
    ///
    /// `low` uses fewer tokens, you can opt in to high resolution using `high`.
    pub detail: Option<String>,
    /// The [File](https://platform.openai.com/docs/api-reference/files) ID of the image
    /// in the message content. Set `purpose="vision"` when uploading the File if you
    /// need to later display the file content.
    pub file_id: Option<String>,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct MessageDeltaDto {
    pub role: String,
    pub content: Vec<MessageContentDelta>,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct ThreadMessageDeltaDto {
    pub id: String,
    pub delta: MessageDeltaDto,
    pub object: String,
}
