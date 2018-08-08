# habits service

A REST API written in Rust and the Rocket web framework
that persists data in a rocksdb embedded key/value store.
Follows resource/processor/externals tiered application structure.

## Requirements

```
cargo 1.29.0-nightly (506eea76e 2018-07-17)
rocksdb v5.14.2
```

## Extra `Rocket.toml` Configs

- `rocksdb_path`: Filepath location of rocksdb sst file.

## Environment Variables

- `ROCKET_ENV` determines the `Rocket.toml` environment section.

## Running

### In Development

```bash
cargo run
```

For now, the sst database files need to be dropped when adding/removing column families.

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
- Handle errors at all layers better.
