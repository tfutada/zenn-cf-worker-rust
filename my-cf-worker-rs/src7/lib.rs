use worker::*;

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

#[event(fetch)]
async fn main(_req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    // let start = std::time::Instant::now();
    let result = fibonacci(42);
    // let duration = start.elapsed();
    console_log!("Fibonacci(40) = {}", result);
    // println!("Time elapsed in Rust: {:?}", duration);

    Response::ok("Done!")
}