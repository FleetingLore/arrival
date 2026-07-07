# Configuration

Arrival supports TOML configuration files.

## Basic Structure

[[nodes]]
path = { nodes = ["root"] }
next = { nodes = ["root/child"] }

[[nodes]]
path = { nodes = ["root/child"] }
result = "Hello, world!"

## Loading from TOML

use arrival_toml::from_str;

let toml = "
[[nodes]]
path = { nodes = ['root'] }
next = { nodes = ['root/child'] }

[[nodes]]
path = { nodes = ['root/child'] }
result = 'Hello'
";

let runtime = from_str(toml).unwrap();

## Loading from a File

use arrival_toml::from_file;

let runtime = from_file("config.toml").unwrap();
