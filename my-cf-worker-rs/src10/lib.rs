use std::time::Duration;
use worker::*;
use reqwest;
use serde_json::json;

#[event(fetch)]
async fn main(req: Request, env: Env, ctx: Context) -> Result<Response> {
    // Make a GET request to Yahoo and Google
    let req1 = reqwest::get("https://www.yahoo.co.jp");
    let req2 = reqwest::get("https://www.google.com");

    // Send both requests concurrently
    let (yahoo_response, google_response) = tokio::join!(req1, req2);

    // Check if Yahoo request succeeded
    let yahoo_response = match yahoo_response {
        Ok(resp) => resp,
        Err(e) => return Response::error(&format!("Yahoo request failed: {}", e), 500),
    };
    // check if google request succeeded
    let google_response = match google_response {
        Ok(resp) => resp,
        Err(e) => return Response::error(&format!("Google request failed: {}", e), 500),
    };

    // Convert the reqwest response to a worker response
    let body_yahoo = yahoo_response.text().await.unwrap_or_else(|_| "Failed to parse Yahoo response".to_string());
    let body_google = google_response.text().await.unwrap_or_else(|_| "Failed to parse Google response".to_string());

    // Return a json with body from yahoo and google, truncate the body to 100 characters
    Response::ok(json!({
        "yahoo": &body_yahoo[..100],
        "google": &body_google[..100],
    }).to_string())
}
