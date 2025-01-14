use leptos::server;
use leptos::ServerFnError;


// Test if entered sequence is valid
// Not empty 
// Composed of A/T/G/C 
#[server(TestSequence, "/api")]
pub async fn test_if_sequence_valid(sequence: std::option::Option<String>) -> Result<std::option::Option<String>, ServerFnError> {
    let output = match sequence {
        // check if string is composed of only a, t, g, and c 
        Some(sequence) if test_if_nucleotide(sequence.clone()) => Some("Not a valid DNA sequence. Contains characters other than A/T/G/C".to_string()),
        Some(_) => Some("pass".to_string()),
        None => Some("waiting".to_string()),
    };
    Ok(output)
}


// For dealing with ServerFnError 
pub async fn load_sequence_test(sequence: std::option::Option<String>) -> std::option::Option<String> {
    let output: std::option::Option<String> = test_if_sequence_valid(sequence).await.expect("reason");
    
    return output
}

pub fn test_if_nucleotide(mut sequence: String) -> bool {
    // Possible implementations
    // Chars -> unique chars -> ...
    // String -> remove a/t/g/c -> count length 
    sequence.retain(|c| c != 't' && c != 'a' && c != 'g' && c != 'c');
    if sequence.len() == 0 { 
        return false;
    }
    else { 
        return true;
    }
}