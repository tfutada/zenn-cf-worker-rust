use worker::*;

#[event(fetch)]
async fn main(req: Request, env: Env, ctx: Context) -> Result<Response> {
    let  a = req.headers().get("x-forwarded-for").unwrap();

    Response::ok("Hello, World222!")
}
