// external
use leptos::server;
use leptos::ServerFnError;
// internal
use crate::structs::transcript_sequence::TranscriptSeq;


#[server(ReturnTranscriptSeq, "/sequence")]
pub async fn msyql_transcript_sequence(transcript_id: String) ->
    Result<Vec<TranscriptSeq>, ServerFnError> {

    use mysql::*;
    use mysql::prelude::*;

    let query: String = format!("Select sequence from cds where id like '{}%'", transcript_id).to_string();
    let url = "mysql://main:pass@172.18.0.3:16/mouse";
    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;

    let output_transcript_seq = conn
        .query_map(
            query,
            | sequence | {
                TranscriptSeq { transcript: transcript_id.clone(), sequence }
            },
        )?;

    Ok(output_transcript_seq)
}

pub async fn load_transcript_sequence(transcript_id: String) -> Result<String, ServerFnError> {
    let data: Vec<TranscriptSeq> = msyql_transcript_sequence(transcript_id).await?;
    let data_string = data[0].sequence.clone().expect("reason");
    Ok(data_string)
    
}