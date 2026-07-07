# CLI Reference

The Arrival CLI provides a command-line interface.

## Commands

### builtin

Run with built-in nodes.

arrival builtin <RAW> [OPTIONS]

Options:
- --verbose, -v: Show detailed output
- --node-type <TYPE>: custom, string, or cli-return

Example:
arrival builtin hello --verbose
arrival builtin test --node-type string

### toml

Run with TOML configuration.

arrival toml <RAW> [OPTIONS]

Options:
- --verbose, -v: Show detailed output
- --config <PATH>: TOML configuration file path

Example:
arrival toml hello --verbose --config config.toml

### list

List all available nodes.

arrival list

## Logging

Set RUST_LOG to control log level.

RUST_LOG=debug arrival builtin hello
