# SslScan #

SslScan is port of the SslLabs automated scanner for Rust. It outputs only the Grade of each ip for
the hostname by default, but you can turn on debug logging, and get the entire ssllabs response printed.
For SslScan there is only two commands.

* `scan_new` - Which forces a new Scan regardless of old results. Just give it a hostname, and you're
off to the races.
* `get_old` - Which will return results that are still in ssllabs cache. If there are now results in
ssllabs cache the app will close for having a bad request, or not be able to decode the response, and
also fail.

## Installing SslScan from Source ##

1. Make sure you have rust installed, and ready to build. Go Here, or use this one-line bash statement:
`curl -sSf https://static.rust-lang.org/rustup.sh | sh`.
2. Run `cargo install sslscan`.
3. All done!

## Contributing ##

Contributing to this project is actively welcomed! Just read `CONTRIBUTING.md` before actually contributing.

## License ##

SslScan is licensed under MIT.

[pull_request_link]: https://github.com/rust-lang/rust/pull/16971
