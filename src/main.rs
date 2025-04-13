use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use actix_files::Files;
use serde::{Deserialize, Serialize};
use std::env;
use reqwest;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;
use dotenv::dotenv;
use bcrypt::{hash, DEFAULT_COST};
use bcrypt::verify;
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use chrono::{Utc, Duration};
use actix_files::NamedFile;
use std::path::PathBuf;

// JWT secret key
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

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
    HttpResponse::Ok().body("Welcome to the Movie Finder API! Use /movies/{movie_name} to search by movie name or Use /movie/{movie_id} to search by movie by movie id.")

}

async fn search_movies(movie_name: web::Path<String>) -> impl Responder {
    let api_key = env::var("MOVIE_API_KEY").expect("API key not found in .env");
    let url = format!("http://www.omdbapi.com/?s={}&apikey={}", movie_name, api_key);

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

#[derive(Deserialize)]
struct SignupData {
    username: String,
    email: String,
    password: String,
}

async fn signup(
    pool: web::Data<MySqlPool>,
    form: web::Json<SignupData>,
) -> HttpResponse {
    // Hash the password
    let hashed_password = match hash(&form.password, DEFAULT_COST) {
        Ok(hash) => hash,
        Err(_) => return HttpResponse::InternalServerError().body("Error hashing password"),
    };

    let result = sqlx::query!(
        "INSERT INTO users (username, email, password) VALUES (?, ?, ?)",
        form.username,
        form.email,
        hashed_password
    )
    .execute(pool.get_ref())
    .await;
    match result {
        Ok(_) => HttpResponse::Ok().body("Signup success"),
        Err(_) => HttpResponse::InternalServerError().body("Error creating user"),
    }
}

#[derive(Deserialize)]
struct LoginData {
    username: String,
    password: String,
}

async fn login(
    pool: web::Data<MySqlPool>,
    form: web::Json<LoginData>,
) -> HttpResponse {
    let result = sqlx::query!(
        "SELECT password FROM users WHERE username = ?",
        form.username
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(row) => {
            let is_valid = verify(&form.password, &row.password).unwrap_or(false);

            if is_valid {
                let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set in .env");

                let expiration = Utc::now()
                    .checked_add_signed(Duration::days(1))
                    .expect("valid timestamp")
                    .timestamp() as usize;

                let claims = Claims {
                    sub: form.username.clone(),
                    exp: expiration,
                };

                let token = encode(
                    &Header::default(),
                    &claims,
                    &EncodingKey::from_secret(jwt_secret.as_bytes()),
                )
                .expect("Failed to encode JWT");

                HttpResponse::Ok().body(token)
            } else{
                HttpResponse::Unauthorized().body("Invalid password")
            }
        }
        Err(sqlx::Error::RowNotFound) => {
            println!("User not found");
            HttpResponse::Unauthorized().body("User not found")
        }
        Err(e) => {
            println!("DB error: {:?}", e);
            HttpResponse::InternalServerError().body("Login failed")
        }
    }
}

async fn protected(req: actix_web::HttpRequest) -> HttpResponse {
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let auth_header = req.headers().get("Authorization");

    if let Some(auth_value) = auth_header {
        if let Ok(auth_str) = auth_value.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..];
                let result = decode::<Claims>(
                    token,
                    &DecodingKey::from_secret(jwt_secret.as_bytes()),
                    &Validation::default(),
                );

                match result {
                    Ok(token_data) => {
                        return HttpResponse::Ok()
                        .body(token_data.claims.sub.clone());
                    }
                    Err(_) => return HttpResponse::Unauthorized().body("Invalid token"),
                }
            }
        }
    }

    HttpResponse::Unauthorized().body("Authorization header missing or malformed")
}

fn verify_token(token: &str, secret: &str) -> Option<Claims> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .ok()
}

async fn dashboard(req: actix_web::HttpRequest) -> actix_web::Result<NamedFile> {
    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or("secret".to_string());

    // Get Authorization: Bearer <token>
    let auth_header = req.headers().get("Authorization");

    if let Some(header_value) = auth_header {
        if let Ok(auth_str) = header_value.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..];

                // ✅ Verify JWT token
                if verify_token(token, &jwt_secret).is_some() {
                    let path: PathBuf = "./protected/index.html".parse().unwrap();
                    return Ok(NamedFile::open(path)?);
                }
            }
        }
    }

    // Invalid/missing token → show unauthorized.html
    let path: PathBuf = "./protected/unauthorized.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}

#[derive(Serialize)]
struct Favorite {
    imdb_id: String,
    title: String,
    year: String,
    poster: String,
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    println!("Server is starting..."); 

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

    // Connect to MySQL
    let pool = MySqlPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    println!("✅ Connected to MySQL");

    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(index))  
            .route("/movies/{movie_name}", web::get().to(search_movies))
            .route("/movie/{id}", web::get().to(get_movie_by_id))
            .route("/signup", web::post().to(signup))
            .route("/login", web::post().to(login))
            .route("/dashboard", web::get().to(dashboard))
            .service(Files::new("/static", "./static").show_files_listing())
            
    })
    .bind("127.0.0.1:5500")?
    .run()
    .await
}

