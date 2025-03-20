document.addEventListener("DOMContentLoaded", () => {
  const searchBox = document.getElementById("movie-search-box");
  const resultGrid = document.getElementById("result-grid");

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
