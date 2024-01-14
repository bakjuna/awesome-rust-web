# Awesome Rust Full Stack Web Framework

With Axum && shaku && yew, this repository represents how can we construct new web server used by rust. Still in progress, not fully done yet.

Because it's not ready yet, still developing!

# Key Features

- DI with `shaku`
- All-in-one backend
- react-like frontend

# Commands

```bash
# lint
make lint
```

## Dockerizing

If you want to isolate all envs such as database, run docker-compose. You can simply start dockerizing with this command.

```bash
make dcu
```

If you want to stop it, then type

```bash
make dcs
```

If you want to remove docker images, then type

```bash
make dcd
```

## backend

```bash
# build
make build-backend
```

## frontend

```bash
# build
make build-frontend
rustup target add wasm32-unknown-unknown
cargo run --package frontend --bin frontend --target wasm32-unknown-unknown

# run local
brew install trunk
trunk serve # Navigate to the frontend directory and run
```
