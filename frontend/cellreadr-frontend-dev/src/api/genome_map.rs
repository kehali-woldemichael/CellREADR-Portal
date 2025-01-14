// Internal
use leptos::server;
use leptos::ServerFnError;
// internal
use crate::structs::sesrna::{SesRNA};


#[allow(unused_variables)] 
#[server(LoadMap, "/api")]
pub async fn generate_target_sesrna_map(length_target: usize, vec_sesrnas: Vec<SesRNA>) -> Result<String, ServerFnError> {
    use std::process::Command;

    
    // Begin & end switch since we are reversing the indexing 
    let mut vec_index_end = vec_sesrnas.iter().map(|x| ((x.index_begin as i32) - (length_target as i32)).abs()).collect::<Vec<i32>>();
    vec_index_end.sort();
    let mut vec_index_begin = vec_sesrnas.iter().map(|x| ((x.index_end as i32) - (length_target as i32)).abs()).collect::<Vec<i32>>();
    vec_index_begin.sort();
    let json_index_begin = serde_json::to_string(&vec_index_begin)?;
    let json_index_end = serde_json::to_string(&vec_index_end)?;

    use std::env::current_dir;
    let base_path = (current_dir()?).display().to_string();
    let python_module_path = format!("{}/python-modules/run_pygenomeviz.py", base_path);

    let json_output = Command::new("python3")
        .arg(python_module_path)
        .arg(length_target.to_string().as_str())
        .arg(json_index_begin)
        .arg(json_index_end)
        .output()
        .expect("python command failed to execute");

    //log::info!("JSON Output: {}", String::from_utf8_lossy(&json_output.stdout));
    let url = String::from_utf8_lossy(&json_output.stdout);
    Ok(url.to_string())
}

pub async fn load_genomic_map(length_target: usize, vec_sesrnas: Vec<SesRNA>) -> String {
    let data: String = generate_target_sesrna_map(length_target, vec_sesrnas).await.expect("reason");
    return data
}