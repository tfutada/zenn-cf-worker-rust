use worker::*;
use reqwest;

// proxy to an upstream api with reqwest
#[event(fetch)]
async fn main(req: Request, env: Env, ctx: Context) -> Result<Response> {
    // Make a GET request to Google
    let response = reqwest::get("https://www.yahoo.co.jp").await;

    match response {
        Ok(res) => {
            let body = res.text().await.unwrap_or_else(|_| "Failed to parse Google response".to_string());
            Response::ok(body)
        },
        Err(_) => Response::ok("Failed to fetch Google"),
    }
}
