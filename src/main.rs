use warp::Filter;
use chrono::{NaiveDateTime};
use serde::{Deserialize, Serialize};

// Define the ExpiryQuery struct
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ExpiryQuery {
    pub expires_at: Option<NaiveDateTime>,
}

#[tokio::main]
async fn main() {
    // Define a route that takes the ExpiryQuery struct as a query parameter
    let api_route = warp::path("api-key")
        .and(warp::query::<ExpiryQuery>())
        .and_then(handle_request);

    // Start the server
    warp::serve(api_route).run(([127, 0, 0, 1], 3030)).await;
}

async fn handle_request(query: ExpiryQuery) -> Result<impl warp::Reply, warp::Rejection> {
    match query.expires_at {
        Some(expiration) => {
            // Here you can do something with `expiration`
            Ok(format!("Received expiration: {}", expiration))
        }
        None => Ok("No expiration date provided".to_string()),
    }
}
