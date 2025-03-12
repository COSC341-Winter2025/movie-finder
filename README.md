## **Movie Finder Website (Rust)**
### **Project Proposal**

### **Group Members**
- **Taven Livingston** - [tliving6@emich.edu](mailto:tliving6@emich.edu)  
- **Long Nguyen Thanh Le** - [tlenguye@emich.edu](mailto:tlenguye@emich.edu)  
- **Alex Boyd** - [aboyd34@emich.edu](mailto:aboyd34@emich.edu)  

---

## **Programming Language**
### **Rust**
Rust is known for its performance, safety, and concurrency, making it ideal for web applications.  

**Typical Usage of Rust:**
- Systems programming  
- Web development (Actix, Axum)  
- Embedded systems  
- Game development  
- Performance-critical applications  

**Language Characteristics:**
- **Typing:** Statically typed  
- **Paradigms:** Multi-paradigm (supports functional, procedural, and object-oriented programming)  
- **Compilation:** Compiled language using LLVM for optimized machine code  

---

## **Application Overview**
The **Movie Finder Website** will allow users to:
- Search for movies by title, genre, or release year  
- View detailed information, including cast, synopsis, and ratings  
- Save favorite movies (if authentication is implemented)  

This project will showcase Rust’s strengths in **web development, concurrency, and safety**.  

---

## **Major Features**
✅ **Search Functionality** – Users can search for movies by title, genre, or release year.  
✅ **Movie Details Page** – Displays movie synopsis, ratings, cast, and release date.  
✅ **User Authentication (Optional)** – Allows users to log in and save favorite movies.  
✅ **API Integration** – Fetches movie data from an external API (OMDB API, TMDB API).  
✅ **Backend** – Built using **Rust** with **Actix Web** or **Axum**.  
✅ **Frontend** – Either a Rust-based framework like **Yew** or a separate frontend in **TSX**.  

---

## **Application Structure**
📌 **Home Page** – Displays a search bar and trending movies.  
📌 **Search Results Page** – Shows a list of movies matching the search query.  
📌 **Movie Details Page** – Provides in-depth information about the selected movie.  
📌 **Favorites Page (if authentication is implemented)** – Displays a user’s saved movies.  

---

## **Milestones & Deliverables**
| **Milestone**  | **Deliverables** |
|---------------|----------------|
| **Milestone 1 (March 23)** | ✅ Backend setup with Rust (Actix Web or Axum) <br> ✅ API integration for fetching movie data <br> ✅ Basic search functionality implemented <br> ✅ Preliminary UI layout |
| **Milestone 2 (April 15)** | ✅ Fully functional search and movie details page <br> ✅ UI improvements and styling <br> ✅ Favorites feature (if included) <br> ✅ Final testing and optimizations |

---

## **Setup Instructions**
### **Prerequisites**
- Install Rust: [Rust Official Website](https://www.rust-lang.org/)
- Install Cargo (comes with Rust)
- Install a Rust web framework (Actix Web or Axum)
- Set up a database (if needed)
- API key for movie data (OMDB/TMDB)

### **Run the Project**
1. Clone the repository:  
   ```sh
   git clone https://github.com/COSC341-Winter2025/movie-finder.git
   cd movie-finder
   ```
2. Install dependencies:  
   ```sh
   cargo build
   ```
3. Run the backend server:  
   ```sh
   cargo run
   ```
4. Open the frontend (if separate):  
   ```sh
   npm start
   ```

---

## **License**
This project is licensed under the **MIT License**.
