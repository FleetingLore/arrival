# Arrival

A framework for abstract layer communication.

## Overview

Arrival allows you to build systems where requests descend through layers of abstraction until an answer is reached.

## Features

- Simple trait-based API
- Path-based node routing
- TOML configuration support
- Built-in node types
- CLI for testing
- Detailed descent path tracking

## Quick Start

Add to Cargo.toml:

[dependencies]
arrival-core = "0.1"

## Built-in Nodes

- StringNode: Returns a fixed string
- CliReturnNode: Executes a CLI command
- SerdeNode: TOML-defined nodes

## Documentation

Full documentation is available at https://arrival.rs

## License

MIT or Apache-2.0
