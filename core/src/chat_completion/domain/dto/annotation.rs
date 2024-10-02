use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct FileCitation {
    pub file_id: Option<String>, // The ID of the specific File the citation is from.
    pub quote: Option<String>,   // The specific quote in the file.
}

#[derive(Default, Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct FileCitationDeltaAnnotation {
    pub index: i32,     // The index of the annotation in the text content part.
    pub r#type: String, // Always `file_citation`.
    pub value: String,
    pub end_index: Option<i32>,
    pub file_citation: Option<FileCitation>,
    pub start_index: Option<i32>,
    pub text: Option<String>, // The text in the message content that needs to be replaced.
}

#[derive(Default, Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct FilePath {
    pub file_id: Option<String>, // The ID of the file that was generated.
}

#[derive(Default, Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct FilePathDeltaAnnotation {
    pub index: i32,     // The index of the annotation in the text content part.
    pub r#type: String, // Always `file_path`.
    pub end_index: Option<i32>,
    pub file_path: Option<FilePath>,
    pub start_index: Option<i32>,
    pub text: Option<String>, // The text in the message content that needs to be replaced.
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum MessageAnnotation {
    FileCitationDelta(FileCitationDeltaAnnotation),
    FilePathDelta(FilePathDeltaAnnotation),
}
