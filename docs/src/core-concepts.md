# Core Concepts

## Arg

Arg is a trait that represents a request or parameter.

pub trait Arg {
    fn to_string(&self) -> String;
}

## Target

Target is a trait that represents a response or result.

pub trait Target {
    fn to_string(&self) -> String;
}

## Node

Node is the core abstraction. Each node has a path and processes an argument.

pub trait Node {
    fn path(&self) -> Path;
    fn process(&self, arg: &dyn Arg) -> NodeResult;
}

## NodeResult

NodeResult has two variants:

pub enum NodeResult {
    Next(Box<dyn Arg>, Path),
    Done(Box<dyn Target>),
}

Next means continue descending.
Done means stop and return the Target.

## Path

Path is a sequence of node names.

pub struct Path {
    pub nodes: Vec<String>,
}

## Runtime

Runtime manages the descent process.

pub struct Runtime {
    nodes: Vec<Box<dyn Node>>,
    path: Path,
}

## Descent Flow

1. User creates an initial Arg
2. Runtime starts at the root path
3. Root node processes the Arg
4. Root node returns Next with new Arg and child path
5. Runtime finds the child node
6. Child node processes the Arg
7. Child node returns Done with a Target
8. Runtime returns the Target to the user
