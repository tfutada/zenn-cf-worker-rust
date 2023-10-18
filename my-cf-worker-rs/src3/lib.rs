use worker::*;
use serde::{Deserialize, Serialize};

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {

    // Create an instance of the Router, which can use parameters (/user/:name) or wildcard values
    // (/file/*pathname). Alternatively, use `Router::with_data(D)` and pass in arbitrary data for
    // routes to access and share using the `ctx.data()` method.
    let router = Router::new();

    // useful for JSON APIs
    #[derive(Deserialize, Serialize)]
    struct Account {
        id: String,
    }
    router
        // handle GET requests to /account/:id, return JSON
        .get_async("/account/:id", |_req, ctx| async move {
            if let Some(id) = ctx.param("id") {
                console_log!("Account ID: {}", id);
                return Response::from_json(&Account { id: id.to_string() });
            }
            Response::error("Bad Request", 400)
        })
        // handle files and fields from multipart/form-data requests
        .post_async("/upload", |mut req, _ctx| async move {
            let form = req.form_data().await?;
            // handle files from multipart/form-data requests
            if let Some(entry) = form.get("file") {
                match entry {
                    FormEntry::File(file) => {
                        let bytes = file.bytes().await?;
                        console_log!("file size: {}", bytes.len());
                    }
                    FormEntry::Field(_) => return Response::error("Bad Request: file", 400),
                }
            }
            // handle fields from multipart/form-data requests
            if let Some(caption) = form.get("caption") {
                match caption {
                    FormEntry::Field(caption) => {
                        console_log!("caption: {}", caption);
                    }
                    FormEntry::File(_) => return Response::error("Bad Request: caption", 400),
                }
            }
            Response::ok("done")
        })
        // read/write binary data
        .post_async("/echo-bytes", |mut req, _ctx| async move {
            let data = req.bytes().await?;
            if data.len() < 16 {
                return Response::error("Bad Request", 400);
            }

            Response::from_bytes(data)
        })
        .run(req, env).await
}