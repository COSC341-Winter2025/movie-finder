const movieSearchBox = document.getElementById("movie-search-box");
const searchList = document.getElementById("search-list");
const resultGrid = document.getElementById("result-grid");

// load movies from API
async function loadMovies(searchTerm) {
  const query = searchTerm.trim();
  const res = await fetch(`/movies/${query}`);
  const data = await res.json();
  console.log(data);
  if (data.length > 0) {
    displayMovieList(data);
  } else {
    searchList.innerHTML = "<p>No movies found.</p>";
  }
}

function findMovies() {
  let searchTerm = movieSearchBox.value;
  if (searchTerm.length > 0) {
    searchList.classList.remove("hide-search-list");
    loadMovies(searchTerm);
  } else {
    searchList.classList.add("hide-search-list");
  }
}

function displayMovieList(movies) {
  searchList.innerHTML = "";
  for (let idx = 0; idx < movies.length; idx++) {
    let movieListItem = document.createElement("div");
    movieListItem.dataset.id = movies[idx].imdbID;
    movieListItem.classList.add("search-list-item");
    if (movies[idx].Poster != "N/A") {
      moviePoster = movies[idx].Poster;
    } else {
      moviePoster = "poster_not_found.jpg";
    }
    movieListItem.innerHTML = `
    <div class="search-list-thumbnail">
      <img src="${moviePoster}" />
    </div>
    <div class="search-item-info">
      <h3>${movies[idx].Title}</h3>
      <p>${movies[idx].Year}</p>
    </div>`;
    searchList.appendChild(movieListItem);
  }
  loadMovieDetails();
}

let currentMovie = null;
function loadMovieDetails() {
  const searchListMovies = searchList.querySelectorAll(".search-list-item");
  searchListMovies.forEach((movie) => {
    movie.addEventListener("click", async () => {
      //console.log(movie.dataset.id);
      searchList.classList.add("hide-search-list");
      movieSearchBox.value = "";
      const res = await fetch(`/movie/${movie.dataset.id}`);
      const data = await res.json();
      currentMovie = data;
      //console.log(data);

      displayMovieDetails(data);
    });
  });
}
async function checkFavoriteStatus(imdbID) {
  const token = localStorage.getItem("token");
  if (!token) return false;

  const res = await fetch("/api/favorites", {
    headers: {
      Authorization: "Bearer " + token,
    },
  });

  const favorites = await res.json();
  return favorites.some((movie) => movie.imdb_id === imdbID);
}

async function displayMovieDetails(movie) {
  resultGrid.innerHTML = `
  <div class="movie-poster">
              <img src="${
                movie.Poster != "N/A" ? movie.Poster : "poster_not_found.jpg"
              }"  alt="poster" />
            </div>
            <div class="movie-info">
              <h3 class="movie-title">${movie.Title}</h3>
              <ul class="movie-misc-info">
                <li class="year">Year: ${movie.Year}</li>

                <li class="rated">Rating: ${movie.imdbRating}</li>
                <li class="released">Released: ${movie.Released}</li>
              </ul>
              <p class="genre"><b>Genre: ${movie.Genre}</b></p>
              <p class="writer"><b>Writer: ${movie.Writer}</b></p>
              <p class="actors"><b>Actors: ${movie.Actors}</b></p>
              <p class="plot"><b>Plot: ${movie.Plot}</b></p>
              <p class="language"><b>Language: ${movie.Language}</b></p>
              <p class="awards">
                <b><i class="fas fa-award"></i>${movie.Awards}</b>
              </p>
            </div>
            `;
  const favBtn = document.getElementById("fav-btn");
  favBtn.style.display = "inline-block";
  isFavorite = await checkFavoriteStatus(movie.imdbID);
  updateFavButton();
}

// Wait for DOM to be fully loaded before adding event listeners
document.addEventListener("DOMContentLoaded", function () {
  // Add event listener for the search box
  movieSearchBox.addEventListener("keyup", findMovies);
});

if (localStorage.getItem("token")) {
  document.getElementById("fav-btn").style.display = "inline-block";
}

let isFavorite = false;

function toggleFavorite() {
  const token = localStorage.getItem("token");
  if (!token || !currentMovie) return;

  if (!isFavorite) {
    // Add to favorites
    fetch("/api/add-favorite", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: "Bearer " + token,
      },
      body: JSON.stringify({
        imdb_id: currentMovie.imdbID,
        title: currentMovie.Title,
        year: currentMovie.Year,
        poster: currentMovie.Poster,
      }),
    })
      .then((res) => res.text())
      .then((message) => {
        isFavorite = true;
        updateFavButton();
        alert(message);
      });
  } else {
    // Remove from favorites
    fetch(`/api/favorites/${currentMovie.imdbID}`, {
      method: "DELETE",
      headers: {
        Authorization: "Bearer " + token,
      },
    })
      .then((res) => res.text())
      .then((message) => {
        isFavorite = false;
        updateFavButton();
        alert(message);
      });
  }
}

function updateFavButton() {
  const btn = document.getElementById("fav-btn");
  btn.innerText = isFavorite ? "❤️ Liked" : "🤍 Like";
  btn.classList.remove("pulse");
  void btn.offsetWidth;
  btn.classList.add("pulse");
}

// cmd+shift+r if the page is not refreshing

// // Event listener for the search box
// searchBox.addEventListener("keyup", async () => {
//   let query = searchBox.value.trim();
//   if (query.length > 2) {
//     let response = await fetch(`/movies/${query}`);
//     let movies = await response.json();
//     displayMovies(movies);
//   }
// });

// // Function to display movies in the result grid
// function displayMovies(movies) {
//   resultGrid.innerHTML = "";
//   movies.forEach((movie) => {
//     let movieCard = document.createElement("div");
//     movieCard.classList.add("movie-card");
//     movieCard.innerHTML = `
//     <div class="movie-poster">
//       <img src="${movie.poster}" alt="${movie.title}">
//     </div>
//     <div class="movie-info">
//       <h3>${movie.title}</h3>
//       <p><b>Year:</b> ${movie.year}</p>
//     </div>
//   `;
//     resultGrid.appendChild(movieCard);
//   });;
