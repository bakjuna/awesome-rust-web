# Awesome Rust Full Stack Web Framework
With Axum && shaku && yew, this repository represents how can we construct new web server used by rust. Still in progress, not fully done yet.

Because it's not ready yet, still developing!

# Key Features
- DI with `shaku`
- All-in-one backend
- react-like frontend

# How to run this server
## Build
### dev
```
make build-dev
```
### prod
```
make build-prod
```

## Lint
```
make lint
```

## frontend
### build
```
rustup target add wasm32-unknown-unknown
cargo run --package frontend --bin frontend --target wasm32-unknown-unknown
```
### run local
```
brew install trunk
trunk serve
```