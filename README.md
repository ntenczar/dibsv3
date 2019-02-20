# dibsv3

It's basically just a Queue, for slack. Form an orderly line.

## Developer Setup

Requires the rust nightly, just use the [Rust toolchain
installer](https://www.rustup.rs/).

Create a `.env` file with the following contents (or specify them as args before
`cargo run`):

```
DB_FILENAME=/valid/path/to/a/file.json
SLACK_TOKEN=some_valid_slack_token
```

```
$ cargo run
```

## Deployment

![](https://media1.tenor.com/images/16477435c4907e9d53b43ff8980e7fa2/tenor.gif?itemid\u003d11824828)
