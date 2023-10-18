# Cloudflare Workers - Rust WASM POC project

This is a companion project of a Zenn article at 

src/lib.rs - copy any of the codes in srcN/lib.rs to run it as a worker

src2 - Hello, World!
src3 - Router
src4 - reqwest
src5 - KV
src6 - D1
src7 - fibonacci to test CPU intensive tasks to see 10ms cap
src8 - Hyper to test TCP Socket connections 
src9 - TCP Socket connection to a server (no HTTP handshaking)
src10 - tokio::join! for multiple reqwest calls

main.rs - standalone

# how to run locally

```bash
pnpm run dev
```

# how to deploy to cloudflare

```bash
pnpm run deploy
```

tweak wrangler.toml for KV and D1.


[workers-rs](https://github.com/cloudflare/workers-rs/tree/main)

