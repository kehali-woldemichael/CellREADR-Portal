// external
use leptos::server;
use leptos::ServerFnError;
// internal
use crate::structs::sesrna::{SesRNA};

#[server(ReturnSesRNAs, "/sesrnas")]
#[allow(unused_variables)] 
pub async fn srv_sesrna_search(target_sequence: String, vec_parameters: Vec<String>)-> Result<Vec<SesRNA>, ServerFnError> {
    use crate::functions::return_sesrna_sequence::{return_sesrnas};
    use csv::*;
    use std::io::prelude::*;
    use std::fs::File;

    //log::info!("Target sequence is START:{}:STOP", target_sequence);
    let search_token = &vec_parameters.clone()[5];

    use std::env::current_dir;
    let base_path = (current_dir()?).display().to_string();

    use leptos::*;
    let conf = get_configuration(None).await.unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = &conf.leptos_options;
    let site_root = &leptos_options.site_root;

    let csv_path = format!("{}/{}/data/{}.csv", base_path, site_root, search_token);
    let fasta_path = format!("{}/{}/data/{}.fasta", base_path, site_root, search_token);

    // Create csv handler 
    let mut writer = Writer::from_path(csv_path)?;
    let mut buffer = File::create(fasta_path)?;
    // Headers for csv fiile
    let _ = writer.write_record(["index_tgg", "index_begin", "index_end", "num_mismatches", "num_start", "num_stop", "length", "dna_sequence", "search_token"]);

    let vec_sesrnas: Vec<SesRNA> = return_sesrnas(target_sequence, vec_parameters);
    // iterate over the sesRNA and write to csv & fasta files
    for sesrna in &vec_sesrnas {
        // Write to csv file
        let _ = writer.write_record(&[
            sesrna.index_tgg.to_string(),
            sesrna.index_begin.to_string(),
            sesrna.index_end.to_string(),
            sesrna.num_mismatches.to_string(),
            sesrna.num_start.to_string(),
            sesrna.num_stop.to_string(),
            sesrna.length.to_string(),
            sesrna.dna_sequence.clone(),
            (&search_token).to_string(),
        ]);
        
        // Write to fasta file
        buffer.write(b">")?;
        //buffer.write(sesrna.index_tgg.to_string().as_bytes())?;
        buffer.write(format!("search_token|{}|", search_token).as_bytes())?;
        buffer.write(format!("index_tgg|{}|", sesrna.index_tgg.to_string()).as_bytes())?;
        buffer.write(format!("index_begin|{}|", sesrna.index_begin.to_string()).as_bytes())?;
        buffer.write(format!("index_end|{}|", sesrna.index_end.to_string()).as_bytes())?;
        buffer.write(format!("num_mismatches|{}|", sesrna.num_mismatches.to_string()).as_bytes())?;
        buffer.write(format!("num_start|{}|", sesrna.num_start.to_string()).as_bytes())?;
        buffer.write(format!("num_stop|{}|", sesrna.num_stop.to_string()).as_bytes())?;
        buffer.write(format!("num_length {}|", sesrna.length.to_string()).as_bytes())?;
        buffer.write(b"\n")?;
        buffer.write(sesrna.dna_sequence.clone().as_bytes())?;
        buffer.write(b"\n")?;
    };

    Ok(vec_sesrnas)
}

#[allow(unused_variables)] 
pub async fn load_sesrnas(input: (Option<String>, Option<Vec<String>>) ) -> Option<Vec<SesRNA>> {
    match input {
        (None, ..) => None, 
        (.., None) => None, 
        (Some(sequence), Some(vec_param)) => Some(srv_sesrna_search(sequence, vec_param).await.expect("reason")),
    }
}