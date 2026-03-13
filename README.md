# ITMO Team Site (Rust Edition 🦀)

Welcome to the **B L A Z I N G L Y \ F A S T** rewrite of the ITMO Team Site. 
Why stick with Python and Django when you can have a compiled, type-safe, memory-safe, fearless-concurrency masterpiece? This is a fully functioning Unix terminal simulator built with **pure Rust** and **Leptos** on the front-end, supported by an **Axum** backend serving up your data.

## Prerequisites

You'll need the Rust toolchain and a couple of extra tools to handle the WebAssembly and CSS orchestration:
1. [Rust](https://rustup.rs/) (obviously)
2. WebAssembly target: `rustup target add wasm32-unknown-unknown`
3. Trunk (frontend bundler): `cargo install trunk`

## How to Start

We've provided a small but mighty `Makefile` so you don't have to memorize commands.

### 1. Start the Backend
To spin up the Axum API server (make sure you have your `db.sqlite3` placed exactly at `../db.sqlite3` relative to this folder, or modify the path in `itmo-backend/src/main.rs`):
```bash
make run-backend
```
*(Runs on port 3000)*

### 2. Start the Frontend
In a **separate terminal**, serve the Leptos frontend App:
```bash
make run-frontend
```
*(Trunk will usually bind to port 8080 and open your browser automatically)*

Now open up `http://localhost:8080` and bask in the ⚡️ blazingly fast ⚡️ glory. 

## Structure

- `./src/` - The Leptos Client-Side Rendered (CSR) app, intercepting keys and simulating OS pathways.
- `./itmo-backend/` - The lightweight Axum/rusqlite API pulling stats and rankings.
- `./index.html` - The raw entrypoint for Trunk.

May your compilations be successful and your binaries be small! 🦀🚀
