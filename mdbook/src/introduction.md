# Introduction

Arrival is a framework for abstract layer communication. Requests descend through layers until an answer is reached.

## What is Arrival?

Arrival provides traits for layered systems:

- Arg: a request that descends through layers
- Target: a response that is returned
- Node: a single layer that processes arguments
- Runtime: the execution engine that manages descent

## Use Cases

- File system abstraction
- Configuration lookup
- Protocol negotiation
- Command routing
- Multi-stage processing

## Status

Arrival is in active development. The core API is stable.
