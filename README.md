# Explanation

My config is expecting a secret and it is of type string, but if the secret entered is numbers it is expected to accept it anyway

If run with the toml file it works normally

```toml
# App.toml
secret = "123"
```

But if executed via env, it does not recognize and generates an error

```sh
# Windows
$env:APP_SECRET = '123'; cargo run
```

```sh
# Linux
APP_SECRET=123 cargo run

OR

APP_SECRET="123" cargo run
```

```
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Error { tag: Tag(Global, 1), profile: Some(Profile(Uncased { string: "global" })), metadata: Some(Metadata { name: "`APP_` environment variable(s)", source: None, provide_location: Some(Location { file: "src/main.rs", line: 21, col: 14 }), interpolater:  }), path: ["secret"], kind: InvalidType(Unsigned(123), "a string"), prev: None }', src/main.rs:8:54
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

If any letters are added, it works normally

```sh
# Windows
$env:APP_SECRET = '123a'; cargo run
```

```sh
# Linux
APP_SECRET=123a cargo run

OR

APP_SECRET="123a" cargo run
```

```
Config {
    secret: "123a",
}
```
