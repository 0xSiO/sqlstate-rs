# sqlstate-rs
[![CI](https://github.com/0xSiO/sqlstate-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/0xSiO/sqlstate-rs/actions/workflows/ci.yml)
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

- Contributions to this project must be submitted under the [project's license](./LICENSE).
- Contributors to this project must attest to the [Developer Certificate of Origin](https://developercertificate.org/) by including a `Signed-off-by` statement in all commit messages.
- All commits must have a valid digital signature.
