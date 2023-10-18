use worker::*;
use reqwest;

// proxy to upstream with reqwest
#[event(fetch, respond_with_errors)]
async fn main(req: Request, env: Env, ctx: Context) -> Result<Response> {
    let kv = env.kv("myrustkv")?;

    kv.put("key4", "value6")?.execute().await?;

    let val = kv.get("key9").text().await?;
    console_log!("key4: {:?}", val);

    Response::empty()
}
