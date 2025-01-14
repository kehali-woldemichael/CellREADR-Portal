use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TranscriptSeq {
    pub transcript: String,
    pub sequence: Option<String>
}