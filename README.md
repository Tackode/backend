# Covid-Journal Backend

REST API for Covid-Journal.

## Getting started

To have a working copy of this project, follow the instructions.

### Installation

Setup [Rust](https://www.rust-lang.org).

Define your environment variables as defined in `.env.sample`. You can either manually define these environment variables or use a `.env` file.

Setup a postgresql database (macOS commands).

```
brew install postgresql
createuser --pwprompt covidjournal # set password to covidjournalpw for instance
createdb --owner=covidjournal covidjournal
```

## Documentation

### HTTP API

```
GET /
```

## Tests

```
cargo test
```

## Authors

-   **Julien Blatecky** - [Julien1619](https://twitter.com/Julien1619)

## License

[MIT](LICENSE.md)
