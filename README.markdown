# The Supervisionary proof-checking system

*Supervisionary* is an experimental proof-checking system for Gordon's higher-order logic ("HOL").
Rather than using programming language features to isolate and protect the kernel, as in typical LCF-style implementations of HOL like HOL4 and Isabelle, Supervisionary uses *privelege* akin to how an operating system kernel is isolated and protected from untrusted user-space code.

For more information, see the `paper/prisc22/prisc22.tex` two-page abstract, or a pre-built [PDF](https://dominicpm.github.io/publications/mulligan-supervisionary-2022.pdf), of our accepted PriSC 2022 talk on Supervisionary and which relates the main ideas behind the system.

## Authors

Supervisionary was originally written by Dominic Mulligan and Nick Spinale, Systems Research Group, Arm Research, Cambridge within the context of the [Veracruz](https://github.com/veracruz-project/veracruz) project.
Since 2022, the system is exclusively maintained by [Dominic Mulligan](https://dominicpm.github.io).

## Tests

To run the tests (assuming a recent version of Rust's toolchain is installed):

1. Build the Veracruz top-level driver, which loads a WASM binary and executes it: `cargo build --release`.
2. Build the type-former tests, which exercises all of the type-former system calls: `cargo build --release --target wasm32-unknown-unknown`
3. Run the driver with the type-former test binary: `RUST_LOG=info target/release/driver --binary target/wasm32-unknown-unknown/release/type_former.wasm`.
