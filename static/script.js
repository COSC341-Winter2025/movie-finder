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

function loadMovieDetails() {
  const searchListMovies = searchList.querySelectorAll(".search-list-item");
  searchListMovies.forEach((movie) => {
    movie.addEventListener("click", async () => {
      //console.log(movie.dataset.id);
      searchList.classList.add("hide-search-list");
      movieSearchBox.value = "";
      const res = await fetch(`/movie/${movie.dataset.id}`);
      const data = await res.json();
      console.log(data);
    });
  });
}

// Wait for DOM to be fully loaded before adding event listeners
document.addEventListener("DOMContentLoaded", function () {
  // Add event listener for the search box
  movieSearchBox.addEventListener("keyup", findMovies);
});

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
