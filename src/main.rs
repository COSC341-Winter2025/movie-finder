#[derive(Serialize, Deserialize)]
struct Movie {
    title: String,
    year: String,
    imdb_id: String,
    poster: String,
}

#[derive(Serialize, Deserialize)]
struct MovieApiResponse{
    Search: Vec<Movie>,
    
}

async fn search_movies(query: web::Path<String>) -> impl Responder {
    let api_key = env::var("MOVIE_API_KEY").expect("API key not found in .env");
    let url = format!("http://www.omdbapi.com/?s={}&apikey={}", query,api_key);

    match reqwest::get(&url).await {
        Ok(response) => match response.json::<MovieApiResponse>().await {
            Ok(data) => HttpResponse::Ok().json(data.Search),
            Err(_) => HttpResponse::InternalServerError().body("Error parsing movie data"),
        },
        Err(_) => HttpResponse::InternalServerError().body("Error fetching movie data"),
        }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    
    HttpServer::new(|| {
        App::new()
            .route("/movies/{query}", web::get().to(search_movies))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

