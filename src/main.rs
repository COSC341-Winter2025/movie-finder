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
    let url = format!("http://www.omdbapi.com/?apikey={}&s={}", api_key, query);
}

