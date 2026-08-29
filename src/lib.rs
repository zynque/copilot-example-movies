use axum::{
    Router,
    extract::{Form, Path, State},
    response::{Html, Redirect},
    routing::{get, post},
};
use maud::{DOCTYPE, Markup, html};
use serde::Deserialize;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Movie {
    id: usize,
    title: String,
    seen: bool,
}

impl Movie {
    fn new(id: usize, title: String) -> Self {
        Self {
            id,
            title,
            seen: false,
        }
    }
}

#[derive(Clone, Default)]
pub struct AppState {
    movies: Arc<Mutex<Vec<Movie>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self::default()
    }

    fn movies(&self) -> Vec<Movie> {
        self.movies.lock().expect("movie store poisoned").clone()
    }
}

#[derive(Deserialize)]
struct AddMovieForm {
    title: String,
}

pub fn app() -> Router {
    app_with_state(AppState::new())
}

pub fn app_with_state(state: AppState) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/movies", post(add_movie))
        .route("/movies/{id}/toggle", post(toggle_movie))
        .with_state(state)
}

async fn index(State(state): State<AppState>) -> Html<String> {
    Html(render_page(&state.movies()).into_string())
}

async fn add_movie(State(state): State<AppState>, Form(form): Form<AddMovieForm>) -> Redirect {
    let title = form.title.trim();
    if title.is_empty() {
        return Redirect::to("/");
    }

    let mut movies = state.movies.lock().expect("movie store poisoned");
    let next_id = movies.last().map(|movie| movie.id + 1).unwrap_or(1);
    movies.push(Movie::new(next_id, title.to_owned()));

    Redirect::to("/")
}

async fn toggle_movie(State(state): State<AppState>, Path(id): Path<usize>) -> Redirect {
    let mut movies = state.movies.lock().expect("movie store poisoned");
    if let Some(movie) = movies.iter_mut().find(|movie| movie.id == id) {
        movie.seen = !movie.seen;
    }

    Redirect::to("/")
}

fn render_page(movies: &[Movie]) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { "Movie Watch List" }
                style {
                    r#"
                    body { font-family: system-ui, sans-serif; margin: 2rem auto; max-width: 42rem; padding: 0 1rem; }
                    form.inline { display: inline; margin-right: 0.75rem; }
                    ul { padding: 0; }
                    li { list-style: none; margin: 0.75rem 0; }
                    .seen { color: #555; text-decoration: line-through; }
                    input, button { font: inherit; }
                    input { padding: 0.45rem; width: 70%; max-width: 20rem; }
                    button { padding: 0.45rem 0.75rem; }
                    "#
                }
            }
            body {
                h1 { "Movie Watch List" }
                p { "Track movies you want to watch and mark them as seen." }
                form method="post" action="/movies" {
                    label for="title" { "Add a movie" }
                    br;
                    input id="title" name="title" placeholder="The Iron Giant" required;
                    button type="submit" { "Save" }
                }

                @if movies.is_empty() {
                    p { "No movies saved yet." }
                } @else {
                    ul {
                        @for movie in movies {
                            li {
                                form.inline method="post" action={ "/movies/" (movie.id) "/toggle" } {
                                    button type="submit" {
                                        @if movie.seen {
                                            "Mark unseen"
                                        } @else {
                                            "Mark seen"
                                        }
                                    }
                                }
                                span class={(if movie.seen { "seen" } else { "pending" })} {
                                    (movie.title)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{AppState, Movie, app_with_state};
    use axum::{
        body::{Body, to_bytes},
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn home_page_renders_empty_state() {
        let response = app_with_state(AppState::new())
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let html = String::from_utf8(body.to_vec()).unwrap();
        assert!(html.contains("Movie Watch List"));
        assert!(html.contains("No movies saved yet."));
    }

    #[tokio::test]
    async fn adding_a_movie_redirects_and_shows_it_on_the_page() {
        let state = AppState::new();
        let app = app_with_state(state.clone());

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/movies")
                    .header("content-type", "application/x-www-form-urlencoded")
                    .body(Body::from("title=Arrival"))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::SEE_OTHER);

        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let html = String::from_utf8(body.to_vec()).unwrap();
        assert!(html.contains("Arrival"));
        assert!(!html.contains("No movies saved yet."));

        let movies = state.movies();
        assert_eq!(movies.len(), 1);
        assert_eq!(movies[0], Movie::new(1, "Arrival".to_owned()));
    }

    #[tokio::test]
    async fn toggling_a_movie_updates_seen_state() {
        let state = AppState {
            movies: std::sync::Arc::new(std::sync::Mutex::new(vec![Movie::new(
                1,
                "Arrival".to_owned(),
            )])),
        };
        let app = app_with_state(state.clone());

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/movies/1/toggle")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::SEE_OTHER);
        let movies = state.movies();
        assert_eq!(movies.len(), 1);
        assert!(movies[0].seen);
    }
}
