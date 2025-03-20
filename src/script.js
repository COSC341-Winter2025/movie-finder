document.addEventListener("DOMContentLoaded", () => {
  const searchBox = document.getElementById("movie-search-box");
  const resultGrid = document.getElementById("result-grid");

  // Event listener for the search box
  searchBox.addEventListener("keyup", async() => {
    let query = searchBox.value.trim();
    if (query.length > 2) {
      let response = await fetch(`/movies/${query}`);
      let movies = await response.json();
      displayMovies(movies);
    }
  });
});

// Function to display movies in the result grid
function 
