/**
 * A simple favorite movies list module.
 */

function createMovieList() {
  const movies = [];

  return {
    add(movie) {
      if (!movie || !movie.title) {
        throw new Error("Movie must have a title");
      }
      movies.push(movie);
    },
    remove(title) {
      const index = movies.findIndex((m) => m.title === title);
      if (index === -1) return false;
      movies.splice(index, 1);
      return true;
    },
    getAll() {
      return [...movies];
    },
    findByTitle(title) {
      return movies.find((m) => m.title === title) || null;
    },
  };
}

module.exports = { createMovieList };
