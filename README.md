# httpdt

A datetime library for HTTP clients and servers.

Generates timestamps for use in the HTTP Date header, the only format required for implementation of HTTP.

Calculates with a focus on clarity from `SystemTime`, with no external dependencies, and provides for updates to previously generated datetimes for speed.

## Why?

For simplicity and fuller comprehension when implementing a client or server. No need to audit a more extensive datetime crate to generate a single relatively straightforward output.

## How?

Instantiate a `Datetime` struct with the `new` method, then get the current timestamp for the 'Date' header field with `for_header`:

```rust
use httpdt::Datetime;

let timestamp = Datetime::new()?
  .for_header();
```

To reduce computation, an initial instance can be used as the basis for successive new timestamps via the `now` method:

```rust
use httpdt::Datetime;

let dt = Datetime::new()?;

let ts_initial = dt
  .for_header();
// ...
let ts_updated = dt
  .now()?
  .for_header();
```

The `default` method provides a `Datetime` instance corresponding to the Unix epoch, the `raw` method the number of seconds since the epoch.

### Docs

The documentation can be built and viewed in the browser with the following command:

```shell
cargo doc --open
```

## Making changes

Running the tests after making changes and adding tests to cover new behaviour is recommended.

### Tests

The unit tests and documentation example can be run with the following command:

```shell
cargo test
```

The unit test cases for each component are in the test module at the base of the corresponding source file.

## Development plan

The following are the expected next steps in the development of the code base. The general medium-term aim is a clear, robust and efficient datetime resource for fuller HTTP implementations. Pull requests are welcome for these and other potential improvements.

- implement a top-level error type
- revisit cross-component integer typing
- document and expose the individual components
- handle timezones, allowing for generation of server log entries in Common Log Format
- revise `SystemTime`-dependent testing
- extend test modules
