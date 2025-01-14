use leptos::server;
use leptos::ServerFnError;

// Test parameters for generating sesRNA
#[server(TestSesrnaParameters, "/api")]
#[allow(unused_variables)] 
pub async fn test_if_parameters_valid(vec_sesrna_parameters: Vec<String>) -> Result<String, ServerFnError> {

    Ok("pass".to_string())
}

// For dealing with ServerFnError 
#[allow(unused_variables)] 
pub async fn load_parameter_test(vec_sesrna_parameters: Vec<String>) -> String {
    test_if_parameters_valid(vec_sesrna_parameters).await.expect("reason")
}