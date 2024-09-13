# The `microapp` example

Here is an example of an app that uses `xwt` for its communication.

The app models an abstract game where players have a nickname and can chat and
move.

The client-side works works in the native and the web environments.

The server-side works works in the native environment only.

## Running the example

### Client

Native:

```shell
cargo run --bin xwt-example-client
```

Web:

```shell
cargo run --bin xwt-example-client --target wasm32-unknown-unknown
```

### Server

Native:

```shell
cargo run --bin xwt-example-server
```

Web is not supported.
