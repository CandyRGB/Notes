use axum::{body::Body, response::Response, routing::get, Router};

#[tokio::main]
async fn main() {
    let api_router = Router::new()
        .route("/shipping/orders", get(orders_handler))
        .fallback(fallback_handler);
    let route = Router::new()
        .nest("/api", api_router)
        .route("/", get(index_handler))
        .route("/health", get(health_handler))
        .fallback(fallback_handler);

    let addr = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Running on {}", addr);

    axum::serve(listener, route).await.unwrap();
}

async fn orders_handler() -> Response {
    let content = include_str!("data/orders.json");
    Response::new(Body::from(content))
}

async fn index_handler() -> Response {
    let content = include_str!("public/index.html");
    Response::new(Body::from(content))
}

async fn health_handler() -> Response {
    let content = include_str!("public/health.html");
    Response::new(Body::from(content))
}

async fn fallback_handler() -> Response {
    let content = include_str!("public/404.html");
    Response::new(Body::from(content))
}