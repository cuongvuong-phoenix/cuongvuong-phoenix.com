use self::schema::AppSchema;
use crate::State;
use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::{Extension, OriginalUri},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::sync::Arc;

mod resolvers;
mod schema;
mod shared;

pub fn init_routes(state: Arc<State>) -> Router {
    let schema = schema::init_schema(state);

    Router::new().nest(
        "/graphql",
        Router::new()
            .route("/", get(graphiql).post(graphql_handler))
            .layer(Extension(schema)),
    )
}

async fn graphiql(OriginalUri(uri): OriginalUri) -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint(uri.path()).finish())
}

async fn graphql_handler(
    Extension(schema): Extension<AppSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}
