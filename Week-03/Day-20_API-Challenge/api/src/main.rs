use actix_web::{web, App, HttpServer, Responder};
use serde::Deserialize;
use reqwest::Client;

mod models;
use models::{Suggestion, SuggestionResponse};

#[derive(Deserialize)]
struct SuggestionQuery {
    q: String,
    latitude: Option<f64>,
    longitude: Option<f64>,
}

async fn get_city_suggestions(query: &str, latitude: Option<f64>, longitude: Option<f64>) -> Vec<Suggestion> {
    let suggestions = vec![
        Suggestion {
            name: format!("{}, ON, Canada", query),
            latitude: 42.98339,
            longitude: -81.23304,
            score: 0.9,
        },
    ];
    suggestions
}

async fn suggestions(query: web::Query<SuggestionQuery>) -> impl Responder {
    // 
    let suggestions = get_city_suggestions(&query.q, query.latitude, query.longitude).await;
    web::Json(SuggestionResponse { suggestions })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting Server");
    HttpServer::new(|| {
        App::new().service(web::resource("/suggestions").to(suggestions))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
