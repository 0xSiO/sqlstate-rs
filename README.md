# sqlstate-rs
[![CI](https://github.com/lucis-fluxum/sqlstate-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/lucis-fluxum/sqlstate-rs/actions/workflows/ci.yml)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![crates.io](https://img.shields.io/crates/v/sqlstate.svg)](https://crates.io/crates/sqlstate)
[![API docs](https://docs.rs/sqlstate/badge.svg)](https://docs.rs/sqlstate)

Representations and parsing logic for [`SQLSTATE`](https://en.wikipedia.org/wiki/SQLSTATE)
return codes.

`sqlstate` supports all standard SQL error codes as defined in ISO/IEC 9075.

Databases may define custom error codes as well - `sqlstate` supports additional error codes for
the following databases:

- [PostgreSQL](https://www.postgresql.org/docs/current/errcodes-appendix.html)
  (enable with feature `postgres`)

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/lucis-fluxum/sqlstate-rs.

## License

These crates are available as open source under the terms of the
[MIT License](https://opensource.org/licenses/MIT).

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
`sqlstate-rs` by you shall be licensed as MIT, without any additional terms or conditions.
