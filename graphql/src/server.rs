use axum::extract::State;
use axum::http::HeaderMap;
use axum::{serve, Router};
use axum::routing::{get};
use log::{info};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::response::{Html, IntoResponse};
use tower::{Layer};
use crate::schema::{GQLSchema, ServiceSchema};
use tower_http;
use tower_http::cors;
use crate::ide::altair::AltairGraphQL;

pub struct Server;

impl Server {
    pub fn new() -> Self {
        Self
    }

    pub async fn start(&self) {
        let app_state = AppState {
            schema: GQLSchema::new()
        };

        let app = self.router(app_state);

        let address = "127.0.0.1:8001";
        info!("Starting server at {}", address);
        let listener = tokio::net::TcpListener::bind(address).await.unwrap();
        serve(listener, app.into_make_service()).await.unwrap();
    }

    fn router(&self, app_state: AppState) -> Router {
        let cors_layer = cors::CorsLayer::new()
            .allow_origin(cors::Any)
            .allow_headers(cors::Any)
            .allow_methods(cors::Any);

        Router::new()
            .layer(cors_layer)
            .route("/", get(graphql_playground).post(graphql_handler))
            .with_state(app_state)
    }
}

#[derive(Clone)]
pub struct AppState {
    schema: ServiceSchema
}

async fn graphql_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    gql_req: GraphQLRequest,
) -> GraphQLResponse {
    let request = gql_req.into_inner();

    state.schema.execute(request).await.into()
}

async fn graphql_playground(State(state): State<AppState>) -> impl IntoResponse {
    let endpoint = "http://localhost:8001";
    Html(
        AltairGraphQL::build()
            .endpoint(&endpoint)
            .title("GQL Explorer")
            .finish(),
    )
}

