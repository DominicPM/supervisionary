//! # Driver application
//!
//! This module defines a top-level "driver" application which takes a path to a
//! Wasm binary, as a command-line argument, and executes the binary under the
//! Supervisionary kernel's supervision.
//!
//! # Authors
//!
//! [Dominic Mulligan], Systems Research Group, [Arm Research] Cambridge.
//! [Nick Spinale], Systems Research Group, [Arm Research] Cambridge.
//!
//! # Copyright
//!
//! Copyright (c) Arm Limited, 2021.  All rights reserved (r).  Please see the
//! `LICENSE.markdown` file in the *Supervisionary* root directory for licensing
//! information.
//!
//! [Dominic Mulligan]: https://dominicpm.github.io
//! [Nick Spinale]: https://nickspinale.com
//! [Arm Research]: http://www.arm.com/research

use clap::{App, Arg};
use log::info;
use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
    process::exit,
};
use wasmi::{
    ExternVal, ImportsBuilder, MemoryRef, Module, ModuleInstance, ModuleRef,
    RuntimeValue,
};
use wasmi_bindings::runtime_state::WasmiRuntimeState;

////////////////////////////////////////////////////////////////////////////////
// Useful constants.
////////////////////////////////////////////////////////////////////////////////

/// An about message/header for the help menu of the driver application.
const ABOUT_MESSAGE: &str = "Driver application for Supervisionary tests.";
/// Name of the driver application.
const APPLICATION_NAME: &str = "Supervisionary driver.";
/// Authors of the driver application.
const AUTHOR_LIST: &str =
    "Dominic P. Mulligan and Nick Spinale, Arm Research, Cambridge.";
/// The name of the Wasm module's heap.
const LINEAR_MEMORY_NAME: &str = "memory";
/// The version number of the driver application.
const VERSION_NUMBER: &str = "0.1.0";
/// The name of the Wasm entry point.
const WASM_ENTRY_POINT: &str = "main";
/// The name of the module resolved by the Wasmi imports resolver.
const WASMI_MODULE_IMPORTS_RESOLVER_NAME: &str = "env";

////////////////////////////////////////////////////////////////////////////////
// Command-line parsing.
////////////////////////////////////////////////////////////////////////////////

/// Captures the command line arguments passed to the program.
struct CommandLineArguments {
    /// The path of the Wasm binary to load.
    wasm_binary_path: PathBuf,
}

/// Parses the command line arguments of the program, exiting with an error code
/// if this cannot be done successfully.  Otherwise, packs the command line
/// arguments into a `CommandLineArguments` value, which is returned.
fn parse_command_line_arguments() -> CommandLineArguments {
    info!("Parsing command line arguments.");

    let matches = App::new(APPLICATION_NAME)
        .about(ABOUT_MESSAGE)
        .version(VERSION_NUMBER)
        .author(AUTHOR_LIST)
        .arg(
            Arg::new("wasm-binary-path")
                .required(true)
                .short('b')
                .long("binary")
                .takes_value(true)
                .about("Path to the Wasm binary to load"),
        )
        .get_matches();

    if let Some(path) = matches.value_of("wasm-binary-path") {
        info!("Command line arguments successfully parsed.");

        CommandLineArguments {
            wasm_binary_path: PathBuf::from(path),
        }
    } else {
        eprintln!("No Wasm binary path provided as argument.");
        exit(1)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Loading.
////////////////////////////////////////////////////////////////////////////////

/// Loads the byte representation of a Wasm binary, stored as `path`, failing if
/// the binary file does not exist.
fn load_binary<P>(path: P) -> Vec<u8>
where
    P: AsRef<Path>,
{
    info!("Loading Wasm binary {:?}.", path.as_ref());

    let mut file = File::open(path).unwrap_or_else(|e| {
        eprintln!("Failed to open Wasm binary.  Error produced: {}.", e);
        exit(1);
    });

    let mut content = Vec::new();

    if let Err(err) = file.read_to_end(&mut content) {
        eprintln!("Could not read Wasm binary file to completion.  Error produced: {}.", err);
        exit(1);
    }

    info!("Wasm binary read successfully.");

    content
}

/// Finds the linear memory of the WASM module, `module`, and returns it,
/// otherwise creates a fatal error.
fn get_module_memory(module: &ModuleRef) -> MemoryRef {
    match module.export_by_name(LINEAR_MEMORY_NAME) {
        Some(ExternVal::Memory(memory)) => memory,
        _otherwise => {
            eprintln!(
                "Wasm module does not export any memory with name {}.",
                LINEAR_MEMORY_NAME
            );
            exit(1)
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Entry point.
////////////////////////////////////////////////////////////////////////////////

fn main() {
    env_logger::init();

    info!("Driver program initialized.");

    let command_line_args = parse_command_line_arguments();

    let binary = load_binary(&command_line_args.wasm_binary_path);

    let loaded_module = Module::from_buffer(&binary).unwrap_or_else(|e| {
        eprintln!("Failed to load Wasm module.  Error produced: {}.", e);
        exit(1);
    });

    info!("Wasm binary loaded.");

    let mut runtime_state = WasmiRuntimeState::new();

    let imports_resolver = ImportsBuilder::new()
        .with_resolver(WASMI_MODULE_IMPORTS_RESOLVER_NAME, &runtime_state);

    let not_started_module = ModuleInstance::new(
        &loaded_module,
        &imports_resolver,
    )
    .unwrap_or_else(|e| {
        eprintln!("Failed to build module instance.  Error produced: {}.", e);
        exit(1);
    });

    info!("Wasmi environment resolver and module instance created.");

    if not_started_module.has_start() {
        eprintln!("Wasm module contains 'start' function.");
        exit(1);
    };

    let module_ref = not_started_module.assert_no_start();

    let memory = get_module_memory(&module_ref);

    runtime_state.set_memory(memory);

    info!("Wasm module memory registered with Wasmi runtime state.");

    info!("Invoking 'main'...");

    /* TODO: scan the binary for 'main' and find if it actually expects
     *       arguments, or not...
     */
    let return_value = module_ref
        .invoke_export(
            WASM_ENTRY_POINT,
            &[RuntimeValue::I32(0), RuntimeValue::I32(0)],
            &mut runtime_state,
        )
        .unwrap_or_else(|e| {
            eprintln!(
                "Failed to invoke '{}' function.  Error produced: {}.",
                WASM_ENTRY_POINT, e
            );
            exit(1)
        });

    match return_value {
        Some(value) => {
            println!(
                "Wasm module executed successfully.  Returned value {:?}.",
                value
            );
        }
        None => {
            println!("Wasm module executed successfully.");
        }
    }
}
