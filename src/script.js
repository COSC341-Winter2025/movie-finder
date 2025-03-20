document.addEventListener("DOMContentLoaded", () => {
  const movieSearchBox = document.getElementById("movie-search-box");
  const searchList = document.getElementById("search-list");
  const resultGrid = document.getElementById("result-grid");

  // load movies from API
  async function loadMovies(searchTerm) {
    const URL = `https://omdbapi.com/?s=${searchTerm}&page=1&apikey=bc8d5ea4`;
    const res = await fetch(`${URL}`);
    const data = await res.json();
    if (data.Response === "True") {
      displayMovies(data.Search);
    }
  }

  // Event listener for the search box
  searchBox.addEventListener("keyup", async () => {
    let query = searchBox.value.trim();
    if (query.length > 2) {
      let response = await fetch(`/movies/${query}`);
      let movies = await response.json();
      displayMovies(movies);
    }
  });

  // Function to display movies in the result grid
  function displayMovies(movies) {
    resultGrid.innerHTML = "";
    movies.forEach((movie) => {
      let movieCard = document.createElement("div");
      movieCard.classList.add("movie-card");
      movieCard.innerHTML = `
      <div class="movie-poster">
        <img src="${movie.poster}" alt="${movie.title}">
      </div>
      <div class="movie-info">
        <h3>${movie.title}</h3>
        <p><b>Year:</b> ${movie.year}</p>
      </div>
    `;
      resultGrid.appendChild(movieCard);
    });
  }
});
