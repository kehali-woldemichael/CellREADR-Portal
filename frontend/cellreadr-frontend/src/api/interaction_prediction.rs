// Internal
use leptos::server;
use leptos::ServerFnError;
// internal
//use crate::structs::sesrna::{SesRNA};

#[allow(unused_variables)] 
#[server(PredictInteraction, "/api")]
pub async fn srv_predict_interaction() -> Result<String, ServerFnError> {
    Ok("peace".to_string())
}