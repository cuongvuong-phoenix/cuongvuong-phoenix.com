use axum::{
    extract::Extension,
    http::{header, HeaderValue, Method},
    routing::get,
    Router, Server,
};
use config::*;
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

mod config;
mod graphql;

pub struct State {
    pub db_pool: Pool<Postgres>,
}

#[tokio::main]
async fn main() {
    config::load_env();
    tracing_subscriber::fmt::init();

    let db_pool: Pool<Postgres> = Pool::connect(&DATABASE_URL).await.unwrap();

    sqlx::migrate!().run(&db_pool).await.unwrap();

    // State.
    let state = Arc::new(State { db_pool });

    // Middlewares.
    let middlewares = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(
            CorsLayer::new()
                .allow_methods(vec![Method::POST, Method::GET])
                .allow_headers(vec![header::CONTENT_TYPE, header::ACCEPT])
                .allow_origin(
                    ALLOWED_ORIGIN
                        .iter()
                        .map(|origin| origin.parse().unwrap())
                        .collect::<Vec<HeaderValue>>(),
                ),
        )
        .layer(Extension(state.clone()));

    // App.
    let app = Router::new()
        .route("/health", get(|| async { "Healthy!" }))
        .nest("/api", graphql::init_routes(state))
        .layer(middlewares);

    tracing::info!("Server running on: {}", *ADDRESS);

    Server::bind(&ADDRESS.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
