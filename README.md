# Rust Actix & Postgres API
Simple Todo Backend Application made with Rust and Actix for performance benefits [source](https://www.techempower.com/benchmarks/).

## Installation

You need to install cargo package available for your system. 

### Database

Run docker-compose (in background):

```bash
docker-compose up -d
```

Command to populate postgres db:

```bash
psql -h 127.0.0.1 -p 5432 -U rust rust < database.sql
```


## Usage

### Running development version in command line

```bash
cargo run
```


### Building for production

```bash
cargo build --release
```

## Performance benchmarks


Homepage:
```bash
ab -n 10000 -k -c 30 -q http://localhost:8080/
```

Todos page (includes postgres reads):

```bash
ab -n 10000 -k -c 30 -q http://localhost:8080/todos
```

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License
[MIT](https://choosealicense.com/licenses/mit/)