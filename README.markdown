# The Supervisionary proof-checking system

*Supervisionary* is an experimental proof-checking system for Gordon's
higher-order logic ("HOL").  Rather than using programming language features to
isolate and protect the kernel, as in typical LCF-style implementations of HOL
like HOL4 and Isabelle, Supervisionary uses *privilege* akin to how an operating
system kernel is isolated and protected from untrusted user-space code, with the
two communicating across a defined system call interface.

For more information, see the `paper/prisc22/prisc22.tex` two-page abstract, or
a pre-built
[PDF](https://dominicpm.github.io/publications/mulligan-supervisionary-2022.pdf),
of our accepted PriSC 2022 talk on Supervisionary and which relates the main
ideas behind the system.

## Authors

Supervisionary was originally written by Dominic Mulligan and Nick Spinale,
Systems Research Group, Arm Research, Cambridge within the context of the
[Veracruz](https://github.com/veracruz-project/veracruz) project.  Since 2022,
the system is exclusively developed and maintained by [Dominic
Mulligan](https://dominicpm.github.io).

## System components

The repository contains the following components in similarly-named directories:

- *Kernel*: this is the core of the Supervisionary kernel, and is completely
  execution-engine independent (well, almost).
- *Wasmi bindings*: this is the binding of the Supervisionary kernel to the
  Wasmi execution engine so that Supervisionary can execute Wasm binaries.
- *libsupervisionary*: this is a user-space Rust support library that abstracts
  over the raw Supervisionary system call interface.
- *Driver*: this is a command line application that reads a Wasm binary,
  supplied on the command line, loads it, then executes it under the
  Supervisionary virtual machine.  Wasm binaries must be compiled using the
  `wasm32-unknown-unknown` target and linked against `libsupervisionary`.
- *System interface*: provides a generic interface that Supervisionary can use
  to query or manipulate aspects of the system, for example the filesystem, or
  clock-related functions.  Also provides a "pass through" implementation of
  this interface which calls through to the underlying system executing
  Supervisionary.
- *Tests*: these are test binaries linked against `libsupervisionary` that
  exercise the different aspects of the Supervisionary system call interface.
These can be executed using `driver`, as explained below.

## License and copyright

See the `LICENSE` file in the Supervisionary root directory for full details of
the MIT open-source license that Supervisionary is licensed under.

## Tests

To execute the Supervisionary tests, perform the following steps:

### Compile `driver`

Perform the following steps, starting from the Supervisionary root directory:

```shell
λ > cd driver
λ > cargo build --release
```

### Compile the tests

Perform the following steps, starting from the Supervisionary root directory:

```shell
λ > cd tests/type
λ > cargo build --release --target wasm32-unknown-unknown
```

Note that this requires that the `wasm32-unknown-unknown` target is installed.
To do this:

```shell
λ > rustup target add wasm32-unknown-unknown
```

### Run the tests

All built binaries are installed in the `target` directory which is created
under the Supervisionary root directory.  Run `driver` as follows:

```shell
λ > ./target/release/driver --binary target/wasm32-unknown-unknown/release/type.wasm
```

The `driver` application will load the binary and begin executing the `main`
function within.  This should print "Wasm module executed successfully" and
produce a `0` return code.  To produce more verbose output, turn the Rust
logging level up as follows:

```shell
λ > RUST_LOG=info ./target/release/driver --binary target/wasm32-unknown-unknown/release/type.wasm
```

This should produce a full trace of the kernel's activity.
