use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Suggestion {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub score: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SuggestionResponse {
    pub suggestions: Vec<Suggestion>,
}