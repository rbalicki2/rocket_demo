# Rocket Demo Project

Write servers in Rust. Because Rust is awesome.

## Installation and starting

```bash
pg_ctl start
# press CTRL-c
createdb rocket_demo
cargo run
```

## Testing

```bash
curl localhost:8000/todos | jq
curl localhost:8000/todos/1 | jq
curl -XPOST localhost:8000/todos -d '{ "text": "Fight the borrow checker", "completed": true }' -H "Content-Type: application/json"
curl -XPATCH localhost:8000/todos/2 -d '{ "text": "Thank the compiler", "completed": true }' -H "Content-Type: application/json"
```

## Other commands to play around with

```bash
diesel print-schema > src/schema.rs
diesel migration generate 'create_user_table'
diesel migration run
```

