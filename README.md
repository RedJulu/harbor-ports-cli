# ⚓ harbor

A lightweight CLI tool written in Rust to list active listening ports and kill processes occupying them.

```
⚓ HARBOR (5 active Ports)

┌───────────┬───────┬──────────┬───────┐
│ Adresse   ┆  Port ┆ Prozess  ┆   PID │
╞═══════════╪═══════╪══════════╪═══════╡
│ 0.0.0.0   ┆ 40081 ┆ spotify  ┆ 11848 │
├╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┤
│ 127.0.0.1 ┆  6463 ┆ Discord  ┆  4941 │
└───────────┴───────┴──────────┴───────┘
```

## Installation

### From crates.io
```bash
cargo install harbor-cli
```

### From GitHub
```bash
cargo install --git [https://github.com/RedJulu/harbor-cli](https://github.com/RedJulu/harbor-cli)
```

## Usage

```bash
# List all active listening ports
harbor -a

# Kill process running on port 8080 (SIGTERM)
harbor -k 8080

# Force kill process running on port 8080 (SIGKILL)
harbor -k 8080 --force
```

> **Note:** Depending on your system permissions, you may need `sudo` to view or kill processes owned by other users or root.

## License

[MIT](LICENSE)
