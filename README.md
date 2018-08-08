# habits service

Habits microservice. A REST API written in Rust and the Rocket web framework
that persists data in a rocksdb embedded key/value store.
Follows resource/processor/externals tiered application structure.

## Extra `Rocket.toml` Configs

- `rocksdb_path`: Filepath location of rocksdb sst file.

## Environment Variables

- `ROCKET_ENV` determines the `Rocket.toml` environment section.

## Running

### In Development

```bash
cargo run
```

### In Production

```bash
make build version=[version]
docker run -d -p 8000:8000 \
    -v /mnt:/volumes \ # Or wherever the persistent block storage is mounted
    --name habits \
    -e ROCKET_ENV=production \
    mtso/habits:[version]
```

## API

[Postman API Docs](https://documenter.getpostman.com/view/1742549/RWThV1u9)

## TODO

- Run API docs collection as tests with newman.
