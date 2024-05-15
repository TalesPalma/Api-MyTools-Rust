use axum::{
    http::StatusCode, response::IntoResponse, routing::get, Json, Router
};
use serde::{Deserialize,Serialize};
use tokio::io::stdout;



static mut TOOLS: Vec<Tools> = Vec::new();


#[derive(Deserialize,Clone,Serialize)]
struct Tools {
        name: String,
        description: String,
        price: String,
        url: String,
        empresa: String,
        cep:String
}

async fn add_item(Json(tool): Json<Tools>) -> impl IntoResponse {
    unsafe {
        TOOLS.push(tool.clone());
    }
    StatusCode::CREATED
}


async fn tools_get() -> Json<Vec<Tools>> {
    unsafe { Json(TOOLS.clone()) }
}



#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/tools", get(tools_get).post(add_item));
    print!("teste");
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}



