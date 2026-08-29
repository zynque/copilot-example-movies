const { createMovieList } = require("./movies");

describe("createMovieList", () => {
  let list;

  beforeEach(() => {
    list = createMovieList();
  });

  test("starts empty", () => {
    expect(list.getAll()).toEqual([]);
  });

  test("adds a movie and retrieves it", () => {
    list.add({ title: "Inception", year: 2010 });
    expect(list.getAll()).toHaveLength(1);
    expect(list.getAll()[0].title).toBe("Inception");
  });

  test("findByTitle returns the correct movie", () => {
    list.add({ title: "The Matrix", year: 1999 });
    list.add({ title: "Interstellar", year: 2014 });
    const movie = list.findByTitle("The Matrix");
    expect(movie).not.toBeNull();
    expect(movie.year).toBe(1999);
  });

  test("findByTitle returns null for unknown title", () => {
    expect(list.findByTitle("Unknown Movie")).toBeNull();
  });

  test("removes a movie by title", () => {
    list.add({ title: "Dune", year: 2021 });
    const removed = list.remove("Dune");
    expect(removed).toBe(true);
    expect(list.getAll()).toHaveLength(0);
  });

  test("remove returns false when movie is not in the list", () => {
    const removed = list.remove("Not Here");
    expect(removed).toBe(false);
  });

  test("throws when adding a movie without a title", () => {
    expect(() => list.add({ year: 2020 })).toThrow("Movie must have a title");
  });
});
