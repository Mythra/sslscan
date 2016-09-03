# SslScan #

SslScan is port of the SslLabs automated scanner for Rust. It outputs only the Grade of each ip for
the hostname by default, but you can turn on debug logging, and get the entire ssllabs response printed.
For SslScan there is only two commands.

* `scan_new` - Which forces a new Scan regardless of old results. Just give it a hostname, and you're
off to the races.
* `get_old` - Which will return results that are still in ssllabs cache. If there are now results in
ssllabs cache the app will close for having a bad request, or not be able to decode the response, and
also fail.

## Building SslScan from Source ##

Right now the only way to build SslScan is too build it from source. This is due to the fact that
rust-nightly is required to build SslScan. Why is rust-nightly required to build you ask? Well due
to [THIS][pull_request_link]. Basically the built in `#[derive(RustcDeserialize)]` doesn't properly
parse Optional Values (i.e. ones that don't exist all the time). Which is a required part of reading
from ssllabs, and I didn't feel like building my own deserializer to handle this.

Anyway! Let's get started with the build process:

Install rust nightly, and building. I recommend doing this by installing through rustup (and not leaving it as
your default):
  - Run the following one liner to install rustup: `curl https://sh.rustup.rs -sSf | sh`
  - Run: `rustup install nightly`
  - Then you can build the project by doing: `rustup run nightly cargo build --release`.

Then you can run: `RUST_LOG=info target/release/sslscan scan_new --host google.com`.

## Contributing ##

Contributing to this project is actively welcomed! Just read `CONTRIBUTING.md` before actually contributing.

## License ##

SslScan is licensed under MIT.

[pull_request_link]: https://github.com/rust-lang/rust/pull/16971
