mod add_movie_form;
mod app_state;
mod html;
mod movie;
mod pages;

pub use app_state::AppState;
pub use movie::Movie;
pub use pages::{app, app_with_state};

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
        let state = AppState::with_movies(vec![Movie::new(1, "Arrival".to_owned())]);
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
        assert!(movies[0].seen());
    }
}
