use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use actix_files::Files;
use serde::{Deserialize, Serialize};
use std::env;
use reqwest;

#[derive(Serialize, Deserialize, Debug)]
struct MovieSearchResult {
    #[serde(rename = "Title")]
    title: String,

    #[serde(rename = "Year")]
    year: String,

    #[serde(rename = "imdbID")]
    imdb_id: String,

    #[serde(rename = "Poster")]
    poster: String,
}

#[derive(Serialize, Deserialize)]
struct MovieApiResponse{
    #[serde(rename = "Search")]
    search: Vec<MovieSearchResult>,
    
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the Movie Finder API! Use /movies/{query} to search.")
}

async fn search_movies(query: web::Path<String>) -> impl Responder {
    let api_key = env::var("MOVIE_API_KEY").expect("API key not found in .env");
    let url = format!("http://www.omdbapi.com/?s={}&apikey={}", query, api_key);

    println!("Fetching movies from: {}", url); // Log API call

    match reqwest::get(&url).await {
        Ok(response) => match response.json::<MovieApiResponse>().await {
            Ok(data) => {
                println!("Received data: {:?}", data.search);
                HttpResponse::Ok().json(data.search)
            }

            Err(_) =>{
                println!("Error parsing movie data");
                HttpResponse::InternalServerError().body("Error parsing movie data")
            } 
        },
        Err(_) =>{
            println!("Error fetching movie data");
            HttpResponse::InternalServerError().body("Error fetching movie data")
        } 
        }
}

#[derive(Serialize, Deserialize, Debug)]
struct MovieDetail {
    #[serde(rename = "Title")]
    title: String,

    #[serde(rename = "Year")]
    year: String,

    #[serde(rename = "imdbID")]
    imdb_id: String,

    #[serde(rename = "Poster")]
    poster: String,

    #[serde(rename = "Genre")]
    genre: String,

    #[serde(rename = "Writer")]
    writer: String,

    #[serde(rename = "Actors")]
    actors: String,

    #[serde(rename = "Plot")]
    plot: String,

    #[serde(rename = "Language")]
    language: String,

    #[serde(rename = "imdbRating")]
    imdb_rating: String,

    #[serde(rename = "Released")]
    released: String,

    #[serde(rename = "Awards")]
    awards: String,
}

async fn get_movie_by_id(movie_id: web::Path<String>) -> impl Responder {
    let api_key = env::var("MOVIE_API_KEY").expect("API key not found in .env");
    let url = format!("http://www.omdbapi.com/?i={}&apikey={}", movie_id, api_key);

    println!("Fetching movie details from: {}", url); // Log API call

    match reqwest::get(&url).await {
        Ok(response) => match response.json::<MovieDetail>().await {
            Ok(data) => HttpResponse::Ok().json(data),
            Err(_) => HttpResponse::InternalServerError().body("Error parsing movie details"),
        },
        Err(_) => HttpResponse::InternalServerError().body("Error fetching movie details"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    println!("Server is starting..."); 
    
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))  
            .route("/movies/{query}", web::get().to(search_movies))
            .route("/movie/{id}", web::get().to(get_movie_by_id))
            .service(Files::new("/static", "./static").show_files_listing())
    })
    .bind("127.0.0.1:5500")?
    .run()
    .await
}

