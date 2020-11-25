# Covid-Journal Backend ![Build](https://github.com/CovidJournal/backend/workflows/Build/badge.svg)

REST API for Covid-Journal.

## Getting started

To have a working copy of this project, follow the instructions.

### Installation

Setup [Rust](https://www.rust-lang.org).

Define your environment variables as defined in `.env.sample`. You can either manually define these environment variables or use a `.env` file.

Setup a postgresql database (macOS commands).

```
brew install postgresql
createuser --pwprompt --superuser covidjournal # set password to covidjournalpw for instance
createdb --owner=covidjournal covidjournal
```

You can use Docker Compose and run the database:

```
docker-compose up -d postgres
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

## Emails

Because of the usage of `font-size:0px;` by MJML, all emails are normally considered as spam.

When saving emails, remove all occurence of `font-size:0px;` in the exported HTML to avoid that and save it with UTF8-BOM.

## Authors

-   **Julien Blatecky** - [Julien1619](https://twitter.com/Julien1619)

## License

[MIT](LICENSE.md)
