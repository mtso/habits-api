# habits service

Habits microservice.

## Extra `Rocket.toml` Configs

- `rocksdb_path`

## Environment Variables

- `ROCKET_ENV` determines the `Rocket.toml` environment section.

## Build

```bash
make build version=[version]
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