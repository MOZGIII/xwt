<div align="center">

  <h1>
    <code>xwt</code>
  </h1>

  <p>
    <i>
      a.k.a. <code>xwebtransport</code> and Cross WebTransport.
    </i>
  </p>

  <p>
    <strong>
      A common WebTransport interface for browser and native.
    </strong>
  </p>

</div>

---

Work in progress. The project is slowly maturing.

You can still peek around, but be aware that I might *force-push `master`*
occasionally.

If you don't mind that - feel free to join the fun right away!

---

## Development

### Requirements

We use a small tool called `cargo-advrunner` under the hood for a bit more
advanced compatibility of the `cargo run` / `cargo test` commands.
Install it via `cargo install -f cargo-advrunner`.

Next, the following tools are also required:

- `wasm-bindgen-test-runner` - from the `wasm-bindgen-cli` crate;
   the runner harness for running the WASM tests in a browser environment

   Install with: `cargo install -f wasm-bindgen-cli`

   You might have to adjust the command above to install a particular version
   if you see a message to do so down the road.
   Also, there is sometimes a need to bump the `wasm-bindgen` in the lockfile -
   usually when the `wasm-server-runner` is too far in
   the future `wasm-server-runner`-version wise from us.

- `wasm-server-runner` - from the `wasm-server-runner` crate;
   the runner harness for running a dev server to server the WASM modules in
   a browser

   Install with: `cargo install -f wasm-server-runner`

Then you should be able to intuitively use `cargo`:

- `cargo build` - native build
- `cargo test` - native tests
- `cargo run --bin ...` - run a native executable
- `cargo build --target wasm32-unknown-unknown` - WASM build
- `cargo test --target wasm32-unknown-unknown` - WASM tests
- `cargo run --target wasm32-unknown-unknown --bin ...` - run a WASM module in a dev server

## Testing

### E2E test suite

We have an E2E test suite that is supposed to ensure the drivers behave more or
less the same.

#### Setup

To set up the transient state (generate the certs and etc) do:

```shell
cargo build --tests
```

You might have to run this command again in the future to regenerate
the certificates that have a lifetime of two weeks (the WebTransport max for
the sha256-authenticated certificates, i.e. when not using WebPKI).

> Also, the `bin/clear-certs` command might come in handy.

#### Running the tests

1. Run the test server.

   ```shell
   cargo run --bin xwt-test-server
   ```

2. Run the native tests.

   ```shell
   cargo test
   ```

3. Run the WASM tests.

   ```shell
   bin/wasm-test
   ```

   For this step, you might have to install some extra dependencies;
   you can use `bin/wasm-test-setup` to help with that.

### Microapp Example

See [examples/microapp](examples/microapp/) for more info.
