#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("bind listener");

    axum::serve(listener, copilot_example_movies::app())
        .await
        .expect("serve application");
}
