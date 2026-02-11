# SMP4 Web API

## Getting started

### Run locally

Install Rust thanks to the tutorial below:  
[Tutorial](https://rust-lang.org/tools/install/)

Start a PostgreSQL docker container by running the following command:
```shell
docker run --name smp4-postgres-db \
    -e POSTGRES_PASSWORD=password \
    -e POSTGRES_USER=postgres \
    -e POSTGRES_DB=smp4 \
    -p 5432:5432 \
    -v smp4-data:/var/lib/postgresql \
    -d postgres
```

Install sqlx-cli to manage your database with the command:
```shell
cargo install sqlx-cli --no-default-features --features native-tls,postgres
```

Update the current database by running:
```shell
sqlx database create
sqlx migrate run
```

Install the project dependencies with the command:
```shell
cargo build
```

Run the project thanks to:
```shell
cargo run
```