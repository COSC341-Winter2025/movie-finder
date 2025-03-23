const movieSearchBox = document.getElementById("movie-search-box");
const searchList = document.getElementById("search-list");
const resultGrid = document.getElementById("result-grid");

// load movies from API
async function loadMovies(searchTerm) {
  const query = searchTerm.trim();
  const res = await fetch(`/movies/${query}`);
  const data = await res.json();
  console.log(data);
  if (data.Response === "True") {
    displayMovies(data.Search);
  }
}

function findMovies() {
  let searchTerm = movieSearchBox.value;
  console.log(searchTerm);
}

// Wait for DOM to be fully loaded before adding event listeners
document.addEventListener("DOMContentLoaded", function () {
  // Add event listener for the search box
  movieSearchBox.addEventListener("keyup", findMovies);
});

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
