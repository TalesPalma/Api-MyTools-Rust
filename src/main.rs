use axum::{
    body::Body,
    http::{status, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(root))
        .route("/tools",get(tools));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

}

#[derive(Serialize)]
struct Tools{
    name: String,
    description: String,
    url: String,
    empresa: String
}

// async fn create_tools() -> impl IntoResponse{
//     Response::builder()
//         .status(StatusCode::CREATED)
//         .body(Body::from("create tools"))
//         .unwrap()
// }

async fn tools() -> Json<Vec<Tools>>{
    let tools = vec![
        Tools{
            name: "name".to_string(),
            description: "description".to_string(),
            url: "url".to_string(),
            empresa: "empresa".to_string()
        },
        Tools{
            name: "name".to_string(),
            description: "description".to_string(),
            url: "url".to_string(),
            empresa: "empresa".to_string()
        }
    ];
        Json(tools)
}


async fn root() -> &'static str {
    "Hello, World!"
}

