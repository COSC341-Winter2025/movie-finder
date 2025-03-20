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
