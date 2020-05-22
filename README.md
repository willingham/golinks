# GoLinks in Rust
This is a dead-simple golinks application written in Rust.  This code was written purely for the fun of it and is not intended to be used in any sort of production environment.

The following tools/frameworks are used:
- [Rocket](https://rocket.rs/): Web framework
- [Diesel](http://diesel.rs/): Database ORM
- [SQLite](https://www.sqlite.org/index.html): Database

## Getting started
- Install Rust & Clone the repository
- Install diesel_cli `cargo install diesel_cli`
- Setup environment variables based on the `.envrc.tpl`
- Create/Migrate the DB `diesel migration run`
- Start the development server `cargo run`

## Capabilities
The following endpoints are available:
- `GET /links`: Returns a list of available golinks
- `POST /<go_slug> <destination>`: Creates a golink
- `GET /<go_slug>`: Redirects to a golink if it exists
- `DELETE /<go_slug>`: Deletes an existing golink if it exists
