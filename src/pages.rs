use crate::{AppState, add_movie_form::AddMovieForm, html::render_page};
use axum::{
    Router,
    extract::{Form, Path, State},
    response::{Html, Redirect},
    routing::{get, post},
};

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

    state.add_movie(title);
    Redirect::to("/")
}

async fn toggle_movie(State(state): State<AppState>, Path(id): Path<usize>) -> Redirect {
    state.toggle_movie(id);
    Redirect::to("/")
}
