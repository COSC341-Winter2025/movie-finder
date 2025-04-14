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
use actix_web::HttpRequest;
use actix_web::get;

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
            println!("Received Authorization header: {}", auth_str);
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..];
                let result = decode::<Claims>(
                    token,
                    &DecodingKey::from_secret(jwt_secret.as_bytes()),
                    &Validation::default(),
                );

                match result {
                    Ok(token_data) => {
                        println!("✅ Valid token for {}", token_data.claims.sub);
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

#[get("/protected")]
async fn protected_api(req: HttpRequest) -> impl Responder {
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    if let Some(auth_value) = req.headers().get("Authorization") {
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
                        println!("✅ Token valid in /protected API: {}", token_data.claims.sub);
                        return HttpResponse::Ok().body(token_data.claims.sub.clone());
                    }
                    Err(err) => {
                        println!("❌ Invalid token in /protected: {:?}", err);
                        return HttpResponse::Unauthorized().body("Invalid token");
                    }
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
    title: Option<String>,
    year: Option<String>,
    poster: Option<String>,
}

async fn get_favorites(req: HttpRequest, db: web::Data<MySqlPool>) -> impl Responder {
    let jwt_secret = env::var("JWT_SECRET").unwrap();

    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..];
                if let Ok(token_data) = decode::<Claims>(
                    token,
                    &DecodingKey::from_secret(jwt_secret.as_bytes()),
                    &Validation::default(),
                ) {
                    let username = token_data.claims.sub;

                    // get user_id from username
                    let user = sqlx::query!(
                        "SELECT id FROM users WHERE username = ?",
                        username
                    )
                    .fetch_one(db.get_ref())
                    .await
                    .unwrap();

                    let favorites = sqlx::query_as!(
                        Favorite,
                        "SELECT imdb_id, title, year, poster FROM favorites WHERE user_id = ?",
                        user.id
                    )
                    .fetch_all(db.get_ref())
                    .await
                    .unwrap();

                    return HttpResponse::Ok().json(favorites);
                }
            }
        }
    }

    HttpResponse::Unauthorized().body("Invalid or missing token")
}

#[derive(Deserialize)]
struct FavoriteData {
    imdb_id: String,
    title: String,
    year: String,
    poster: String,
}

async fn add_favorite(
    req: HttpRequest,
    data: web::Json<FavoriteData>,
    db: web::Data<MySqlPool>,
) -> impl Responder {
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    // ✅ Extract and verify token
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..];

                let token_data = decode::<Claims>(
                    token,
                    &DecodingKey::from_secret(jwt_secret.as_bytes()),
                    &Validation::default(),
                );

                match token_data {
                    Ok(claims) => {
                        let username = claims.claims.sub;

                        // ✅ Look up user_id
                        let user = sqlx::query!(
                            "SELECT id FROM users WHERE username = ?",
                            username
                        )
                        .fetch_one(db.get_ref())
                        .await;

                        if let Ok(user) = user {
                            // ✅ Insert favorite movie
                            let insert_result = sqlx::query!(
                                "INSERT IGNORE INTO favorites (user_id, imdb_id, title, year, poster) VALUES (?, ?, ?, ?, ?)",
                                user.id,
                                data.imdb_id,
                                data.title,
                                data.year,
                                data.poster
                            )
                            .execute(db.get_ref())
                            .await;

                            match insert_result {
                                Ok(_) => HttpResponse::Ok().body("✅ Movie added to favorites"),
                                Err(e) => {
                                    println!("DB error on insert: {:?}", e);
                                    HttpResponse::InternalServerError().body("❌ Failed to save favorite")
                                }
                            }
                        } else {
                            HttpResponse::Unauthorized().body("User not found")
                        }
                    }
                    Err(_) => HttpResponse::Unauthorized().body("Invalid token"),
                }
            } else {
                HttpResponse::Unauthorized().body("Malformed Authorization header")
            }
        } else {
            HttpResponse::Unauthorized().body("Invalid header string")
        }
    } else {
        HttpResponse::Unauthorized().body("Missing Authorization header")
    }
}

async fn remove_favorite(
    req: HttpRequest,
    path: web::Path<String>,
    db: web::Data<MySqlPool>,
) -> impl Responder {
    let imdb_id = path.into_inner();
    let jwt_secret = env::var("JWT_SECRET").unwrap();

    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..];

                let token_data = decode::<Claims>(
                    token,
                    &DecodingKey::from_secret(jwt_secret.as_bytes()),
                    &Validation::default(),
                );

                if let Ok(claims) = token_data {
                    let username = claims.claims.sub;

                    let user = sqlx::query!("SELECT id FROM users WHERE username = ?", username)
                        .fetch_one(db.get_ref())
                        .await;

                    if let Ok(user) = user {
                        let result = sqlx::query!(
                            "DELETE FROM favorites WHERE user_id = ? AND imdb_id = ?",
                            user.id,
                            imdb_id
                        )
                        .execute(db.get_ref())
                        .await;

                        return match result {
                            Ok(_) => HttpResponse::Ok().body("Removed from favorites"),
                            Err(e) => {
                                println!("Delete error: {:?}", e);
                                HttpResponse::InternalServerError().body("Failed to remove")
                            }
                        };
                    }
                }
            }
        }
    }

    HttpResponse::Unauthorized().body("Unauthorized")
}

#[get("/favorite")]
async fn favorite_page(req: HttpRequest) -> actix_web::Result<NamedFile> {
    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or("secret".to_string());
    
    println!("🔐 Requesting /favorite");
    
    // First check Authorization header
    if let Some(header_value) = req.headers().get("Authorization") {
        if let Ok(auth_str) = header_value.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..];
                
                if let Some(claims) = verify_token(token, &jwt_secret) {
                    println!("✅ Valid token for {}", claims.sub);
                    let path: PathBuf = "./protected/favorite.html".parse().unwrap();
                    return Ok(NamedFile::open(path)?);
                }
            }
        }
    }
    
    // If no valid Authorization header, check query parameters
    if let Some(query) = req.uri().query() {
        if let Some(token) = web::Query::<std::collections::HashMap<String, String>>::from_query(query)
            .ok()
            .and_then(|q| q.get("token").cloned())
        {
            if let Some(claims) = verify_token(&token, &jwt_secret) {
                println!("✅ Valid token from query for {}", claims.sub);
                let path: PathBuf = "./protected/favorite.html".parse().unwrap();
                return Ok(NamedFile::open(path)?);
            }
        }
    }

    println!("⚠️ Unauthorized access to /favorite");
    Ok(NamedFile::open("./protected/unauthorized.html")?)
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
            //.route("/", web::get().to(index))  
            .route("/movies/{movie_name}", web::get().to(search_movies))
            .route("/movie/{id}", web::get().to(get_movie_by_id))
            .route("/signup", web::post().to(signup))
            .route("/login", web::post().to(login))
            //.route("/dashboard", web::get().to(dashboard))
            .route("/dashboard", web::get().to(|| async {
                NamedFile::open("./protected/index.html")
            }))
            .route("/api/favorites", web::get().to(get_favorites))
            .route("/api/add-favorite", web::post().to(add_favorite))
            .route("/api/favorites/{imdb_id}", web::delete().to(remove_favorite))
            //.route("/favorite", web::get().to(favorite_page))
            .service(Files::new("/static", "./static").show_files_listing())
            .service(protected_api)
            .service(favorite_page)
            .route("/", web::get().to(|| async {
                NamedFile::open("./static/index.html")
            }))
            
            
    })
    .bind("127.0.0.1:5500")?
    .run()
    .await
}

