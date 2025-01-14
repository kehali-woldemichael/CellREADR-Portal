// external
use serde::{Serialize, Deserialize};
use leptos_struct_table::*;
// internal
use crate::components::general::tailwind::TailwindClassesPreset;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, TableRow, Clone)]
#[table(
    sortable,
    classes_provider = "TailwindClassesPreset",
    impl_vec_data_provider,
)]
pub struct TranscriptsInfo {
    pub gene_name: String,
    #[table(
        cell_class = "text-red-600 dark:text-red-400",
        head_class = "text-red-700 dark:text-red-300"
    )]
    pub stable_transcript_id: String,
    pub transcript_id: String,
    pub biotype: String,
    pub length: i32,
}
impl TranscriptsInfo {
    pub fn new() -> TranscriptsInfo {
        TranscriptsInfo {
            gene_name: String::from("default"),
            stable_transcript_id: String::from(""),
            transcript_id: String::from(""),
            biotype: String::from(""),
            length: 0
        }
    }
}