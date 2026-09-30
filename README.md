# ⚓ harbor

A lightweight CLI tool written in Rust to list active ports and kill processes occupying them.

```
     ⚓ HARBOR (3 active Ports)

┌───────────┬───────┬──────────┬───────────┬───────┐
│  Address  ┆  Port ┆ Protocol ┆  Process  ┆  PID  │
╞═══════════╪═══════╪══════════╪═══════════╪═══════╡
│ 127.0.0.1 ┆ 6463  ┆    TCP   ┆ Discord   ┆ 4941  │
├╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┤
│  0.0.0.0  ┆ 40081 ┆    TCP   ┆ spotify   ┆ 11848 │
├╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┤
│  0.0.0.0  ┆ 57621 ┆    UDP   ┆ spotify   ┆ 11848 │
└───────────┴───────┴──────────┴───────────┴───────┘

⚓ » harbor -a
```

## Installation

### From crates.io
```bash
cargo install harbor-ports-cli
```

### From GitHub
```bash
cargo install --git https://github.com/RedJulu/harbor-ports-cli
```

## Usage


### Options

| Flag | Description |
|------|-------------|
| `-a`, `--all` | List all active ports |
| `-f`, `--filter <PORT>` | Show listeners on the given port, process name or PID |
| `-p`, `--protocol <tcp\|udp\|all>` | Filter by protocol (default: `all`, requires `--all`) |
| `-n`, `--amount <N>` | Show only the first `N` entries (minimum 1) |
| `--full-addr` | Don't shorten long addresses (requires `--all`) |
| `-k`, `--kill <PORT>` | Kill the process using the given port |
| `--force` | Use SIGKILL instead of SIGTERM (requires `--kill`) |
| `--json`  | Prints the Output in Json format  |

Long IPv6 addresses are shortened by default (e.g. `2001:9e8:4...d19:2754`). The wildcard address `::` is displayed as `[::]`.

> **Note:** Depending on your system permissions, you may need `sudo` to view or kill processes owned by other users or root.

## License

[MIT](LICENSE)
