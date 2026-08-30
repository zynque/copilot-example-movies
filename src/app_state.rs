use crate::Movie;
use std::sync::{Arc, Mutex};

#[derive(Clone, Default)]
pub struct AppState {
    movies: Arc<Mutex<Vec<Movie>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn with_movies(movies: Vec<Movie>) -> Self {
        Self {
            movies: Arc::new(Mutex::new(movies)),
        }
    }

    pub(crate) fn movies(&self) -> Vec<Movie> {
        self.movies.lock().expect("movie store poisoned").clone()
    }

    pub(crate) fn add_movie(&self, title: &str) {
        let mut movies = self.movies.lock().expect("movie store poisoned");
        let next_id = movies.last().map(|movie| movie.id() + 1).unwrap_or(1);
        movies.push(Movie::new(next_id, title.to_owned()));
    }

    pub(crate) fn toggle_movie(&self, id: usize) {
        let mut movies = self.movies.lock().expect("movie store poisoned");
        if let Some(movie) = movies.iter_mut().find(|movie| movie.id() == id) {
            movie.toggle_seen();
        }
    }
}
