# habits service

Habits microservice.

## Extra `Rocket.toml` Configs

- `rocksdb_path`

## Environment Variables

- `ROCKET_ENV` determines the `Rocket.toml` environment section.

## Build

1. Build (or use pre-built) build image.

```bash
docker build -t mtso/rust-rocksdb-build -f Dockerfile-build .
```

2. Build project binary.

```bash
docker run --rm -itd -v `pwd`/build:/root/target --name rust-build mtso/rust-rocksdb-build
docker exec -it rust-build /bin/bash
/install-rust.sh
cargo build --release
# Ctrl+D to exit container
docker stop rust-build
```

3. Build deploy image.

```bash
docker build -t mtso/token-service .
```

## API

### `POST /habits`

```json
{
  "user_id": "abc123"
}
```

Response:
```json
{
  "id": "abc123",
  "user_id": "abc123",
  "created_at": "{Timestamp}",
  "updated_at": "{Timestamp}",
  "checks": []
}
```

### `POST /habits/{id}/check`

Response:
```json
{
  "id": "abc123",
  "user_id": "abc123",
  "created_at": "{Timestamp}",
  "updated_at": "{Timestamp}",
  "checks": ["2018-08-05"]
}
```

### `POST /habits/{id}/uncheck`

Response:
```json
{
  "id": "abc123",
  "user_id": "abc123",
  "created_at": "{Timestamp}",
  "updated_at": "{Timestamp}",
  "checks": []
}
```

### `GET /habits/{id}`

Response:
```json
{
  "id": "abc123",
  "user_id": "abc123",
  "created_at": "{Timestamp}",
  "updated_at": "{Timestamp}",
  "count": 2
}
```

### `DELETE /habits/{id}`

Response:
```json
{
  "id": "abc123",
  "user_id": "abc123",
  "created_at": "{Timestamp}",
  "updated_at": "{Timestamp}",
  "count": 2
}
```