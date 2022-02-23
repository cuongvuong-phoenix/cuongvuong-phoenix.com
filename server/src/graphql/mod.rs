use self::schema::AppSchema;
use crate::State;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::{Extension, OriginalUri},
    response::{Html, IntoResponse},
    routing::get,
    AddExtensionLayer, Router,
};
use std::sync::Arc;

mod schema;

pub fn init_routes(state: Arc<State>) -> Router {
    let schema = schema::init_schema(state);

    Router::new().nest(
        "graphql",
        Router::new()
            .route("/", get(graphql_playground).post(graphql_handler))
            .layer(AddExtensionLayer::new(schema)),
    )
}

async fn graphql_playground(OriginalUri(uri): OriginalUri) -> impl IntoResponse {
    Html(playground_source(
        GraphQLPlaygroundConfig::new(uri.path()).with_setting("schema.polling.enable", false),
    ))
}

async fn graphql_handler(
    Extension(schema): Extension<AppSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}
