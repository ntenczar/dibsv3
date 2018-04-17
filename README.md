# dibsv3

An older robot from a different time has rusted over.


## Developer Setup

Requires the rust nightly, just use the [Rust toolchain
installer](https://www.rustup.rs/).

The database is a combination of Postgres + Diesel ORM

- [Diesel](http://diesel.rs/guides/getting-started/)
- [Postgres](https://www.postgresql.org/)

Once the Diesel CLI is set up, create a `.env` file with the following contents:

```
DATABASE_URL=postgres://dibs:admin@localhost/dibs
SLACK_TOKEN=some_valid_slack_token
```

```
$ diesel migration run
```

With the database all set, start up the Rocket server

```
$ cargo run
```

## Deployment

![](https://m.popkey.co/02ce61/b0y4j.gif)
