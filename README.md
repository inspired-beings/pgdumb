# pgDumb

A minimal desktop GUI for running SQL queries and `psql` meta-commands (`\d`, `\l`, `\dt`, …) against PostgreSQL, staying as close as possible to the raw `psql` workflow.

Unlike full admin tools such as pgAdmin or DBeaver, pgDumb doesn't manage your database for you — no query builder, no visual schema/ER designer, no data-editing grid. It's a thin, distraction-free layer over `psql`: connect, run a command, read the raw output.

## Features

- [x] Connection management and selection
- [ ] Connection-level meta-commands (`\l`, `\du`, `\conninfo`, …) for the selected connection, with raw output
- [ ] Database list and selection
- [x] SQL query execution with raw output
- [ ] Database-level meta-commands (`\d*`) for the selected database, with raw output
- [ ] SQL syntax highlighting
- [ ] SQL auto-formatting

## Develop

See [CONTRIBUTING.md](./CONTRIBUTING.md).

## License

[AGPL-3.0](./LICENSE.md).
