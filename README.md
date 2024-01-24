[![Built with ink!](https://raw.githubusercontent.com/paritytech/ink/master/.images/badge.svg)](https://github.com/paritytech/ink)

# CTRL-Health

## Disclaimer

The project is under active development.


1. Dev logs - https://github.com/orgs/CTRL-Health/projects

## Description

### Key Features of PatientZero Solution

## Development Roadmap





```sh
cargo watch -q -c -w src/ -w .cargo/ -x run
```

```sh
cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"
```

```sh
docker run --rm --name pg -p 5432:5432 \
   -e POSTGRES_PASSWORD=welcome \
   postgres:15
```