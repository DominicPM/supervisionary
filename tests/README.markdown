# Tests

Unit-level and module-level integration tests for the various Supervisionary
components.  Directory contents:

- `type_former`: tests of the Supervisionary type-former system calls and
  preprovisioned handles to primitive type-formers.
- `type`: tests of the Supervisionary type system calls and preprovisioned
  handles to primitive types.
- `constant`: tests of the Supervisionary constant system calls and
  prprovisioned handles to primitive constants.
- `system`: tests of the Supervisionary system access and manipulation system
  calls.
- `term`: tests of the Supervisionary term system calls and preprovisioned
  handles to primitive terms.
- `theorem`: tests of the Supervisionary theorem system calls.

## Building

To build, enter the relevant test directory and then:

```shell
λ > cargo build --release --target wasm32-unknown-unknown
```

The resulting binary is stored in `../target/wasm32-unknown-unknown/release/`.
