use leptos::server;
use leptos::ServerFnError;

//use crate::js::tailwind;
use crate::structs::transcripts_info::TranscriptsInfo;

#[server(ReturnTranscriptsInfo, "/api")]
pub async fn msyql_gene_transcripts(gene_name: String) -> 
    Result<Vec<TranscriptsInfo>, ServerFnError> {

    use mysql::*;
    use mysql::prelude::*;

    let query: String = format!("Select stable_id,transcript_id,biotype,seq_region_end,seq_region_start from transcript where gene_id in (Select gene_id from gene_attrib where value='{}')", gene_name).to_string();
    let url = "mysql://main:pass@172.18.0.3:16/mouse";
    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;

    // Let's select payments from database. Type inference should do the trick here.
    let output_transcript_info = conn
        .query_map(
            query,
            |( stable_transcript_id, transcript_id, biotype, stop, start): (String, String, String, i32, i32)| {
                TranscriptsInfo { gene_name: gene_name.clone(), stable_transcript_id, transcript_id, biotype, length: stop- start}
            },
        )?;

    
    Ok(output_transcript_info)
}

pub async fn load_transcript_info(gene_name: String) -> Vec<TranscriptsInfo> {
    let data: Vec<TranscriptsInfo> = msyql_gene_transcripts(gene_name).await.expect("reason");
    
    return data
}