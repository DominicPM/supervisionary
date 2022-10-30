//! # Wasmi runtime state
//!
//! This binds the kernel code, proper, to the guest WASM program-facing ABI
//! interface, routing host-calls as appropriate.
//!
//! # Ideas for changes
//!
//! - Remove `Theorem.Register.TruthIntroduction` in place of a pre-allocated
//!   theorem pre-provisioned in the runtime state at boot?
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
//! [Dominic Mulligan]: https://dominic-mulligan.co.uk
//! [Nick Spinale]: https://nickspinale.com
//! [Arm Research]: http://www.arm.com/research

use std::{borrow::Borrow, cell::RefCell, fmt::Debug, mem::size_of};

use byteorder::{ByteOrder, LittleEndian};
use log::{error, info};
use wasmi::{
    Error as WasmiError, Externals, FuncInstance, FuncRef, MemoryRef,
    ModuleImportResolver, RuntimeArgs, RuntimeValue, Signature, Trap,
};

use kernel::{
    error_code::ErrorCode as KernelErrorCode,
    handle::{tags, Handle},
    name::Name,
    runtime_state::RuntimeState as KernelRuntimeState,
};

use crate::{
    runtime_trap,
    runtime_trap::RuntimeTrap,
    system_call_numbers::{
        ABI_CONSTANT_IS_REGISTERED_INDEX, ABI_CONSTANT_IS_REGISTERED_NAME,
        ABI_CONSTANT_REGISTER_INDEX, ABI_CONSTANT_REGISTER_NAME,
        ABI_CONSTANT_RESOLVE_INDEX, ABI_CONSTANT_RESOLVE_NAME,
        ABI_TERM_FREE_VARIABLES_INDEX, ABI_TERM_FREE_VARIABLES_NAME,
        ABI_TERM_IS_REGISTERED_INDEX, ABI_TERM_IS_REGISTERED_NAME,
        ABI_TERM_REGISTER_APPLICATION_INDEX,
        ABI_TERM_REGISTER_APPLICATION_NAME,
        ABI_TERM_REGISTER_CONJUNCTION_INDEX,
        ABI_TERM_REGISTER_CONJUNCTION_NAME, ABI_TERM_REGISTER_CONSTANT_INDEX,
        ABI_TERM_REGISTER_CONSTANT_NAME, ABI_TERM_REGISTER_DISJUNCTION_INDEX,
        ABI_TERM_REGISTER_DISJUNCTION_NAME, ABI_TERM_REGISTER_EQUALITY_INDEX,
        ABI_TERM_REGISTER_EQUALITY_NAME, ABI_TERM_REGISTER_EXISTS_INDEX,
        ABI_TERM_REGISTER_EXISTS_NAME, ABI_TERM_REGISTER_FORALL_INDEX,
        ABI_TERM_REGISTER_FORALL_NAME, ABI_TERM_REGISTER_IMPLICATION_INDEX,
        ABI_TERM_REGISTER_IMPLICATION_NAME, ABI_TERM_REGISTER_LAMBDA_INDEX,
        ABI_TERM_REGISTER_LAMBDA_NAME, ABI_TERM_REGISTER_NEGATION_INDEX,
        ABI_TERM_REGISTER_NEGATION_NAME, ABI_TERM_REGISTER_VARIABLE_INDEX,
        ABI_TERM_REGISTER_VARIABLE_NAME, ABI_TERM_SPLIT_APPLICATION_INDEX,
        ABI_TERM_SPLIT_APPLICATION_NAME, ABI_TERM_SPLIT_CONJUNCTION_INDEX,
        ABI_TERM_SPLIT_CONJUNCTION_NAME, ABI_TERM_SPLIT_CONSTANT_INDEX,
        ABI_TERM_SPLIT_CONSTANT_NAME, ABI_TERM_SPLIT_DISJUNCTION_INDEX,
        ABI_TERM_SPLIT_DISJUNCTION_NAME, ABI_TERM_SPLIT_EQUALITY_INDEX,
        ABI_TERM_SPLIT_EQUALITY_NAME, ABI_TERM_SPLIT_EXISTS_INDEX,
        ABI_TERM_SPLIT_EXISTS_NAME, ABI_TERM_SPLIT_FORALL_INDEX,
        ABI_TERM_SPLIT_FORALL_NAME, ABI_TERM_SPLIT_IMPLICATION_INDEX,
        ABI_TERM_SPLIT_IMPLICATION_NAME, ABI_TERM_SPLIT_LAMBDA_INDEX,
        ABI_TERM_SPLIT_LAMBDA_NAME, ABI_TERM_SPLIT_NEGATION_INDEX,
        ABI_TERM_SPLIT_NEGATION_NAME, ABI_TERM_SPLIT_VARIABLE_INDEX,
        ABI_TERM_SPLIT_VARIABLE_NAME, ABI_TERM_SUBSTITUTE_INDEX,
        ABI_TERM_SUBSTITUTE_NAME, ABI_TERM_TEST_APPLICATION_INDEX,
        ABI_TERM_TEST_APPLICATION_NAME, ABI_TERM_TEST_CONJUNCTION_INDEX,
        ABI_TERM_TEST_CONJUNCTION_NAME, ABI_TERM_TEST_CONSTANT_INDEX,
        ABI_TERM_TEST_CONSTANT_NAME, ABI_TERM_TEST_DISJUNCTION_INDEX,
        ABI_TERM_TEST_DISJUNCTION_NAME, ABI_TERM_TEST_EQUALITY_INDEX,
        ABI_TERM_TEST_EQUALITY_NAME, ABI_TERM_TEST_EXISTS_INDEX,
        ABI_TERM_TEST_EXISTS_NAME, ABI_TERM_TEST_FORALL_INDEX,
        ABI_TERM_TEST_FORALL_NAME, ABI_TERM_TEST_IMPLICATION_INDEX,
        ABI_TERM_TEST_IMPLICATION_NAME, ABI_TERM_TEST_LAMBDA_INDEX,
        ABI_TERM_TEST_LAMBDA_NAME, ABI_TERM_TEST_NEGATION_INDEX,
        ABI_TERM_TEST_NEGATION_NAME, ABI_TERM_TEST_VARIABLE_INDEX,
        ABI_TERM_TEST_VARIABLE_NAME, ABI_TERM_TYPE_INFER_INDEX,
        ABI_TERM_TYPE_INFER_NAME, ABI_TERM_TYPE_IS_PROPOSITION_INDEX,
        ABI_TERM_TYPE_IS_PROPOSITION_NAME, ABI_TERM_TYPE_SUBSTITUTE_INDEX,
        ABI_TERM_TYPE_SUBSTITUTE_NAME, ABI_TERM_TYPE_VARIABLES_INDEX,
        ABI_TERM_TYPE_VARIABLES_NAME, ABI_THEOREM_IS_REGISTERED_INDEX,
        ABI_THEOREM_IS_REGISTERED_NAME, ABI_THEOREM_REGISTER_APPLICATION_INDEX,
        ABI_THEOREM_REGISTER_APPLICATION_NAME,
        ABI_THEOREM_REGISTER_ASSUMPTION_INDEX,
        ABI_THEOREM_REGISTER_ASSUMPTION_NAME, ABI_THEOREM_REGISTER_BETA_INDEX,
        ABI_THEOREM_REGISTER_BETA_NAME,
        ABI_THEOREM_REGISTER_CONJUNCTION_INTRODUCTION_INDEX,
        ABI_THEOREM_REGISTER_CONJUNCTION_INTRODUCTION_NAME,
        ABI_THEOREM_REGISTER_CONJUNCTION_LEFT_ELIMINATION_INDEX,
        ABI_THEOREM_REGISTER_CONJUNCTION_LEFT_ELIMINATION_NAME,
        ABI_THEOREM_REGISTER_CONJUNCTION_RIGHT_ELIMINATION_INDEX,
        ABI_THEOREM_REGISTER_CONJUNCTION_RIGHT_ELIMINATION_NAME,
        ABI_THEOREM_REGISTER_DISJUNCTION_ELIMINATION_INDEX,
        ABI_THEOREM_REGISTER_DISJUNCTION_ELIMINATION_NAME,
        ABI_THEOREM_REGISTER_DISJUNCTION_LEFT_INTRODUCTION_INDEX,
        ABI_THEOREM_REGISTER_DISJUNCTION_LEFT_INTRODUCTION_NAME,
        ABI_THEOREM_REGISTER_DISJUNCTION_RIGHT_INTRODUCTION_INDEX,
        ABI_THEOREM_REGISTER_DISJUNCTION_RIGHT_INTRODUCTION_NAME,
        ABI_THEOREM_REGISTER_ETA_INDEX, ABI_THEOREM_REGISTER_ETA_NAME,
        ABI_THEOREM_REGISTER_EXISTS_ELIMINATION_INDEX,
        ABI_THEOREM_REGISTER_EXISTS_ELIMINATION_NAME,
        ABI_THEOREM_REGISTER_EXISTS_INTRODUCTION_INDEX,
        ABI_THEOREM_REGISTER_EXISTS_INTRODUCTION_NAME,
        ABI_THEOREM_REGISTER_FALSITY_ELIMINATION_INDEX,
        ABI_THEOREM_REGISTER_FALSITY_ELIMINATION_NAME,
        ABI_THEOREM_REGISTER_FORALL_ELIMINATION_INDEX,
        ABI_THEOREM_REGISTER_FORALL_ELIMINATION_NAME,
        ABI_THEOREM_REGISTER_FORALL_INTRODUCTION_INDEX,
        ABI_THEOREM_REGISTER_FORALL_INTRODUCTION_NAME,
        ABI_THEOREM_REGISTER_IFF_INTRODUCTION_INDEX,
        ABI_THEOREM_REGISTER_IFF_INTRODUCTION_NAME,
        ABI_THEOREM_REGISTER_IFF_LEFT_ELIMINATION_INDEX,
        ABI_THEOREM_REGISTER_IFF_LEFT_ELIMINATION_NAME,
        ABI_THEOREM_REGISTER_IMPLICATION_ELIMINATION_INDEX,
        ABI_THEOREM_REGISTER_IMPLICATION_ELIMINATION_NAME,
        ABI_THEOREM_REGISTER_IMPLICATION_INTRODUCTION_INDEX,
        ABI_THEOREM_REGISTER_IMPLICATION_INTRODUCTION_NAME,
        ABI_THEOREM_REGISTER_LAMBDA_INDEX, ABI_THEOREM_REGISTER_LAMBDA_NAME,
        ABI_THEOREM_REGISTER_NEGATION_ELIMINATION_INDEX,
        ABI_THEOREM_REGISTER_NEGATION_ELIMINATION_NAME,
        ABI_THEOREM_REGISTER_NEGATION_INTRODUCTION_INDEX,
        ABI_THEOREM_REGISTER_NEGATION_INTRODUCTION_NAME,
        ABI_THEOREM_REGISTER_REFLEXIVITY_INDEX,
        ABI_THEOREM_REGISTER_REFLEXIVITY_NAME,
        ABI_THEOREM_REGISTER_SUBSTITUTE_INDEX,
        ABI_THEOREM_REGISTER_SUBSTITUTE_NAME,
        ABI_THEOREM_REGISTER_SYMMETRY_INDEX,
        ABI_THEOREM_REGISTER_SYMMETRY_NAME,
        ABI_THEOREM_REGISTER_TRANSITIVITY_INDEX,
        ABI_THEOREM_REGISTER_TRANSITIVITY_NAME,
        ABI_THEOREM_REGISTER_TRUTH_INTRODUCTION_INDEX,
        ABI_THEOREM_REGISTER_TRUTH_INTRODUCTION_NAME,
        ABI_THEOREM_REGISTER_TYPE_SUBSTITUTE_INDEX,
        ABI_THEOREM_REGISTER_TYPE_SUBSTITUTE_NAME,
        ABI_THEOREM_SPLIT_CONCLUSION_INDEX, ABI_THEOREM_SPLIT_CONCLUSION_NAME,
        ABI_THEOREM_SPLIT_HYPOTHESES_INDEX, ABI_THEOREM_SPLIT_HYPOTHESES_NAME,
        ABI_TYPE_FORMER_IS_REGISTERED_INDEX,
        ABI_TYPE_FORMER_IS_REGISTERED_NAME, ABI_TYPE_FORMER_REGISTER_INDEX,
        ABI_TYPE_FORMER_REGISTER_NAME, ABI_TYPE_FORMER_RESOLVE_INDEX,
        ABI_TYPE_FORMER_RESOLVE_NAME, ABI_TYPE_IS_REGISTERED_INDEX,
        ABI_TYPE_IS_REGISTERED_NAME, ABI_TYPE_REGISTER_COMBINATION_INDEX,
        ABI_TYPE_REGISTER_COMBINATION_NAME, ABI_TYPE_REGISTER_FUNCTION_INDEX,
        ABI_TYPE_REGISTER_FUNCTION_NAME, ABI_TYPE_REGISTER_VARIABLE_INDEX,
        ABI_TYPE_REGISTER_VARIABLE_NAME, ABI_TYPE_SIZE_INDEX,
        ABI_TYPE_SIZE_NAME, ABI_TYPE_SPLIT_COMBINATION_INDEX,
        ABI_TYPE_SPLIT_COMBINATION_NAME, ABI_TYPE_SPLIT_FUNCTION_INDEX,
        ABI_TYPE_SPLIT_FUNCTION_NAME, ABI_TYPE_SPLIT_VARIABLE_INDEX,
        ABI_TYPE_SPLIT_VARIABLE_NAME, ABI_TYPE_SUBSTITUTE_INDEX,
        ABI_TYPE_SUBSTITUTE_NAME, ABI_TYPE_TEST_COMBINATION_INDEX,
        ABI_TYPE_TEST_COMBINATION_NAME, ABI_TYPE_TEST_FUNCTION_INDEX,
        ABI_TYPE_TEST_FUNCTION_NAME, ABI_TYPE_TEST_VARIABLE_INDEX,
        ABI_TYPE_TEST_VARIABLE_NAME, ABI_TYPE_VARIABLES_INDEX,
        ABI_TYPE_VARIABLES_NAME,
    },
    system_interface_types::semantic_types,
    type_checking,
};

////////////////////////////////////////////////////////////////////////////////
// Errors and traps.
////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////
// The Wasmi runtime state.
////////////////////////////////////////////////////////////////////////////////

/// The Wasmi runtime state, which is a thin wrapper around the kernel's own
/// runtime state, adding a reference to the guest WASM program's memory module,
/// to enable host functions to read-from and write-to the memory module
/// directly.
#[derive(Debug)]
pub struct WasmiRuntimeState {
    /// The kernel's runtime state.
    kernel: RefCell<KernelRuntimeState>,
    /// The memory instance of the executing WASM guest program.
    memory: Option<RefCell<MemoryRef>>,
}

impl Default for WasmiRuntimeState {
    #[inline]
    fn default() -> Self {
        Self {
            kernel: RefCell::new(Default::default()),
            memory: None,
        }
    }
}

impl WasmiRuntimeState {
    /// Constructs a new instance of a `WasmiRuntimeState` with the kernel state
    /// initialised to its correct initial state, and the reference to the Wasm
    /// guest's memory set to `None`.
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    ////////////////////////////////////////////////////////////////////////////
    // Memory-related functionality.
    ////////////////////////////////////////////////////////////////////////////

    /// Returns `true` iff the WASM guest's memory module has been registered.
    #[inline]
    pub fn is_memory_registered(&self) -> bool {
        self.memory.is_some()
    }

    /// Registers the WASM guest's memory module with the runtime state.
    #[inline]
    pub fn set_memory(&mut self, instance: MemoryRef) -> &mut Self {
        self.memory = Some(RefCell::new(instance));
        self
    }

    /// Writes a buffer of byte values, `bytes`, to the WASM guest program's
    /// memory starting at the provided `address`.
    ///
    /// # Errors
    ///
    /// Returns `Err(RuntimeTrap::MemoryNotRegistered)` if the WASM guest's
    /// memory module has not been registered with the runtime state.
    ///
    /// Returns `Err(RuntimeTrap::MemoryWriteFailed)` if the write to memory at
    /// address, `address`, failed.
    fn write_bytes<T>(
        &self,
        address: T,
        bytes: &[u8],
    ) -> Result<(), RuntimeTrap>
    where
        T: Into<semantic_types::Pointer>,
    {
        let address = address.into();

        info!("Writing {:?} bytes at address {:#x}.", bytes, address);

        let memory = match &self.memory {
            None => return Err(RuntimeTrap::MemoryNotRegistered),
            Some(memory) => memory,
        };

        memory
            .borrow_mut()
            .set(address.into(), bytes)
            .map_err(|_e| RuntimeTrap::MemoryWriteFailed)?;

        Ok(())
    }

    /// Writes a `u64` value to the WASM guest's memory module at a specified
    /// address.
    ///
    /// # Errors
    ///
    /// Returns `Err(RuntimeTrap::MemoryNotRegistered)` if the WASM guest's
    /// memory module has not been registered with the runtime state.
    ///
    /// Returns `Err(RuntimeTrap::MemoryWriteFailed)` if the write to memory at
    /// address, `address`, failed.
    fn write_u64<T, U>(&self, address: T, value: U) -> Result<(), RuntimeTrap>
    where
        T: Into<semantic_types::Pointer>,
        U: Into<u64>,
    {
        let address = address.into();
        let value = value.into();

        info!("Writing u64 value {} at address {:#x}.", value, address);

        let mut buffer = vec![0u8; 8];
        LittleEndian::write_u64(&mut buffer, value);

        self.write_bytes(address, &buffer)
    }

    /// Writes a collection of `u64` values to the WASM guest's memory
    /// module starting at a specified address.
    ///
    /// # Errors
    ///
    /// Returns `Err(RuntimeTrap::MemoryNotRegistered)` if the WASM guest's
    /// memory module has not been registered with the runtime state.
    ///
    /// Returns `Err(RuntimeTrap::MemoryWriteFailed)` if the write to memory at
    /// address, `address`, failed.
    fn write_u64s<T, U>(
        &self,
        address: T,
        values: Vec<U>,
    ) -> Result<(), RuntimeTrap>
    where
        T: Into<semantic_types::Pointer>,
        U: Into<u64> + Clone,
    {
        let mut address = address.into();

        info!(
            "Writing {} u64 values starting at address {:#x}.",
            values.len(),
            address
        );

        for v in values.iter().cloned() {
            self.write_u64(address, v)?;
            address += 8;
        }

        Ok(())
    }

    /// Writes a `bool` value to the WASM guest's memory module at a specified
    /// address.
    ///
    /// # Errors
    ///
    /// Returns `Err(RuntimeTrap::MemoryNotRegistered)` if the WASM guest's
    /// memory module has not been registered with the runtime state.
    ///
    /// Returns `Err(RuntimeTrap::MemoryWriteFailed)` if the write to memory at
    /// address, `address`, failed.
    fn write_bool<T, U>(&self, address: T, value: U) -> Result<(), RuntimeTrap>
    where
        T: Into<semantic_types::Pointer>,
        U: Into<bool>,
    {
        let mut buffer = vec![0u8; 4];
        let address = address.into();
        let value = value.into();

        info!("Writing bool value {} at address {:#x}.", value, address);

        LittleEndian::write_u32(&mut buffer, value as u32);

        self.write_bytes(address, &buffer)
    }

    /// Writes a handle to the WASM guest's memory module at a specified
    /// address.
    ///
    /// # Errors
    ///
    /// Returns `Err(RuntimeTrap::MemoryNotRegistered)` if the WASM guest's
    /// memory module has not been registered with the runtime state.
    ///
    /// Returns `Err(RuntimeTrap::MemoryWriteFailed)` if the write to memory at
    /// address, `address`, failed.
    #[inline]
    fn write_handle<T, U, V>(
        &self,
        address: T,
        handle: U,
    ) -> Result<(), RuntimeTrap>
    where
        T: Into<semantic_types::Pointer>,
        U: Into<Handle<V>>,
        V: tags::IsTag + Debug,
    {
        let handle = handle.into();
        let address = address.into();

        info!("Writing handle {:?} at address {:#x}.", handle, address);

        self.write_u64(address, *handle as u64)
    }

    /// Writes a collection of handles to the WASM guest's memory module
    /// starting at a specified address.
    ///
    /// # Errors
    ///
    /// Returns `Err(RuntimeTrap::MemoryNotRegistered)` if the WASM guest's
    /// memory module has not been registered with the runtime state.
    ///
    /// Returns `Err(RuntimeTrap::MemoryWriteFailed)` if the write to memory at
    /// address, `address`, failed.
    fn write_handles<T, U, V>(
        &self,
        address: T,
        handles: Vec<U>,
    ) -> Result<(), RuntimeTrap>
    where
        T: Into<semantic_types::Pointer>,
        U: Into<Handle<V>>,
        V: tags::IsTag + Debug,
    {
        let mut address = address.into();

        info!(
            "Writing {} handles starting at address {:#x}.",
            handles.len(),
            address
        );

        for handle in handles {
            self.write_handle(address, handle)?;
            address += 8;
        }

        Ok(())
    }

    /// Reads a fixed `byte_count` of bytes from the WASM guest's memory module
    /// at a specified `address`.
    ///
    /// # Errors
    ///
    /// Returns `Err(RuntimeTrap::MemoryNotRegistered)` if the WASM guest's
    /// memory module has not been registered with the runtime state.
    ///
    /// Returns `Err(RuntimeTrap::MemoryReadFailed)` if the read from memory at
    /// address, `address`, failed.
    fn read_bytes<T, U>(
        &self,
        address: T,
        byte_count: U,
    ) -> Result<Vec<u8>, RuntimeTrap>
    where
        T: Into<semantic_types::Pointer>,
        U: Into<usize>,
    {
        let address = address.into();
        let byte_count = byte_count.into();

        info!("Reading {} bytes at address {:#x}.", byte_count, address);

        let memory = match &self.memory {
            None => return Err(RuntimeTrap::MemoryNotRegistered),
            Some(memory) => memory,
        };

        let bytes = memory
            .borrow()
            .get(address.into(), byte_count.into())
            .map_err(|_e| RuntimeTrap::MemoryReadFailed)?;

        Ok(bytes)
    }

    /// Reads a `u64` value from the WASM guest's memory module at a specified
    /// `address`.
    ///
    /// # Errors
    ///
    /// Returns `Err(RuntimeTrap::MemoryNotRegistered)` if the WASM guest's
    /// memory module has not been registered with the runtime state.
    ///
    /// Returns `Err(RuntimeTrap::MemoryReadFailed)` if the read from memory at
    /// address, `address`, failed.
    #[inline]
    fn read_u64<T>(&self, address: T) -> Result<u64, RuntimeTrap>
    where
        T: Into<semantic_types::Pointer>,
    {
        let address = address.into();

        info!("Reading u64 value at address {:#x}.", address);

        let buffer = self.read_bytes(address, size_of::<u64>())?;
        Ok(LittleEndian::read_u64(&buffer))
    }

    /// Reads multiple `u64` values, as described by `count`, from the WASM
    /// guest's memory module at a specified `address`.
    ///
    /// # Errors
    ///
    /// Returns `Err(RuntimeTrap::MemoryNotRegistered)` if the WASM guest's
    /// memory module has not been registered with the runtime state.
    ///
    /// Returns `Err(RuntimeTrap::MemoryReadFailed)` if the read from memory at
    /// address, `address`, failed.
    fn read_u64s<T, U>(
        &self,
        address: T,
        count: U,
    ) -> Result<Vec<u64>, RuntimeTrap>
    where
        T: Into<u32>,
        U: Into<usize>,
    {
        let mut accumulator = Vec::new();
        let mut address = address.into();
        let count = count.into();

        info!("Reading {} u64 values at address {:#x}.", count, address);

        for _c in 0..count {
            let handle = self.read_u64(address)?;
            accumulator.push(handle);
            address += 8;
        }

        Ok(accumulator)
    }

    /// Reads a `Handle` from the WASM guest's memory module at a specified
    /// `address`.
    ///
    /// # Errors
    ///
    /// Returns `Err(RuntimeTrap::MemoryNotRegistered)` if the WASM guest's
    /// memory module has not been registered with the runtime state.
    ///
    /// Returns `Err(RuntimeTrap::MemoryReadFailed)` if the read from memory at
    /// address, `address`, failed.
    #[inline]
    fn read_handle<T, U>(&self, address: U) -> Result<Handle<T>, RuntimeTrap>
    where
        T: tags::IsTag,
        U: Into<u32>,
    {
        let address = address.into();

        info!("Reading handle at address {:#x}.", address);

        Ok(Handle::from(self.read_u64(address)? as usize))
    }

    /// Reads multiple `Handle` values, as described by `count`, from the WASM
    /// guest's memory module at a specified `address`.
    ///
    /// # Errors
    ///
    /// Returns `Err(RuntimeTrap::MemoryNotRegistered)` if the WASM guest's
    /// memory module has not been registered with the runtime state.
    ///
    /// Returns `Err(RuntimeTrap::MemoryReadFailed)` if the read from memory at
    /// address, `address`, failed.
    fn read_handles<T, U, V>(
        &self,
        address: U,
        count: V,
    ) -> Result<Vec<Handle<T>>, RuntimeTrap>
    where
        T: tags::IsTag,
        U: Into<u32>,
        V: Into<usize>,
    {
        let mut accumulator = Vec::new();
        let mut address = address.into();
        let count = count.into();

        info!("Reading {} handles at address {:#x}.", count, address);

        for _c in 0..count {
            let handle = self.read_handle(address)?;
            accumulator.push(handle);
            address += 8;
        }

        Ok(accumulator)
    }

    ////////////////////////////////////////////////////////////////////////////
    // Kernel-related functionality.
    ////////////////////////////////////////////////////////////////////////////

    /// Lifting of the `type_former_resolve` function.
    #[inline]
    fn type_former_resolve<T>(&self, handle: T) -> Option<usize>
    where
        T: Borrow<Handle<tags::TypeFormer>>,
    {
        self.kernel.borrow().type_former_resolve(handle).cloned()
    }

    /// Lifting of the `type_former_is_registered` function.
    #[inline]
    fn type_former_is_registered<T>(&self, handle: T) -> bool
    where
        T: Borrow<Handle<tags::TypeFormer>>,
    {
        self.kernel.borrow().type_former_is_registered(handle)
    }

    /// Lifting of the `type_former_register` function.
    #[inline]
    fn type_former_register<T>(&self, arity: T) -> Handle<tags::TypeFormer>
    where
        T: Into<usize> + Clone,
    {
        self.kernel.borrow_mut().type_former_register(arity)
    }

    /// Lifting of the `type_register_variable` function.
    #[inline]
    fn type_register_variable<T>(&self, name: T) -> Handle<tags::Type>
    where
        T: Into<Name> + Clone,
    {
        self.kernel.borrow_mut().type_register_variable(name)
    }

    /// Lifting of the `type_register_combination` function.
    #[inline]
    fn type_register_combination<T, U>(
        &self,
        type_former: T,
        arguments: Vec<U>,
    ) -> Result<Handle<tags::Type>, KernelErrorCode>
    where
        T: Into<Handle<tags::TypeFormer>> + Clone,
        U: Into<Handle<tags::Type>> + Clone,
    {
        self.kernel.borrow_mut().type_register_combination(
            type_former.into(),
            arguments.iter().cloned().map(|a| a.into()).collect(),
        )
    }

    /// Lifting of the `type_register_function` function.
    #[inline]
    fn type_register_function<T>(
        &self,
        domain: T,
        range: T,
    ) -> Result<Handle<tags::Type>, KernelErrorCode>
    where
        T: Into<Handle<tags::Type>>,
    {
        self.kernel
            .borrow_mut()
            .type_register_function(domain.into(), range.into())
    }

    /// Lifting of the `type_is_registered` function.
    #[inline]
    fn type_is_registered<T>(&self, handle: T) -> bool
    where
        T: Borrow<Handle<tags::Type>>,
    {
        self.kernel.borrow().type_is_registered(handle)
    }

    /// Lifting of the `type_split_variable` function.
    #[inline]
    fn type_split_variable<T>(&self, handle: T) -> Result<Name, KernelErrorCode>
    where
        T: Borrow<Handle<tags::Type>>,
    {
        self.kernel.borrow().type_split_variable(handle).map(|n| *n)
    }

    /// Lifting of the `type_split_combination` function.
    #[inline]
    fn type_split_combination<T>(
        &self,
        handle: T,
    ) -> Result<
        (Handle<tags::TypeFormer>, Vec<Handle<tags::Type>>),
        KernelErrorCode,
    >
    where
        T: Borrow<Handle<tags::Type>>,
    {
        self.kernel
            .borrow()
            .type_split_combination(handle)
            .map(|(f, a)| (f.clone(), a.clone()))
    }

    /// Lifting of the `type_split_function` function.
    #[inline]
    fn type_split_function<T>(
        &self,
        handle: T,
    ) -> Result<(Handle<tags::Type>, Handle<tags::Type>), KernelErrorCode>
    where
        T: Borrow<Handle<tags::Type>>,
    {
        self.kernel
            .borrow()
            .type_split_function(handle)
            .map(|(d, r)| (d.clone(), r.clone()))
    }

    /// Lifting of the `type_test_variable` function.
    #[inline]
    fn type_test_variable<T>(&self, handle: T) -> Result<bool, KernelErrorCode>
    where
        T: Borrow<Handle<tags::Type>>,
    {
        self.kernel.borrow().type_test_variable(handle)
    }

    /// Lifting of the `type_test_combination` function.
    #[inline]
    fn type_test_combination<T>(
        &self,
        handle: T,
    ) -> Result<bool, KernelErrorCode>
    where
        T: Borrow<Handle<tags::Type>>,
    {
        self.kernel.borrow().type_test_combination(handle)
    }

    /// Lifting of the `type_test_function` function.
    #[inline]
    fn type_test_function<T>(&self, handle: T) -> Result<bool, KernelErrorCode>
    where
        T: Borrow<Handle<tags::Type>>,
    {
        self.kernel.borrow().type_test_function(handle)
    }

    /// Lifting of the `type_size` function.
    #[inline]
    fn type_size<T>(&self, handle: T) -> Result<u64, KernelErrorCode>
    where
        T: Borrow<Handle<tags::Type>>,
    {
        self.kernel.borrow().type_size(handle)
    }

    /// Lifting of the `type_variables` function.
    #[inline]
    fn type_variables<T>(&self, handle: T) -> Result<Vec<Name>, KernelErrorCode>
    where
        T: Borrow<Handle<tags::Type>>,
    {
        self.kernel
            .borrow_mut()
            .type_variables(handle)
            .map(|v| v.iter().map(|e| **e).collect())
    }

    /// Lifting of the `type_substitute` function.
    #[inline]
    fn type_substitute<T, U, V>(
        &self,
        handle: T,
        sigma: Vec<(U, V)>,
    ) -> Result<Handle<tags::Type>, KernelErrorCode>
    where
        T: Borrow<Handle<tags::Type>> + Clone,
        U: Into<Name> + Clone + Debug,
        V: Into<Handle<tags::Type>> + Clone + Debug,
    {
        self.kernel.borrow_mut().type_substitute(handle, sigma)
    }

    /// Lifting of the `constant_register` function.
    #[inline]
    fn constant_register<T>(
        &self,
        handle: T,
    ) -> Result<Handle<tags::Constant>, KernelErrorCode>
    where
        T: Into<Handle<tags::Type>> + Clone,
    {
        self.kernel.borrow_mut().constant_register(handle)
    }

    /// Lifting of the `constant_resolve` function.
    #[inline]
    fn constant_resolve<T>(
        &self,
        handle: T,
    ) -> Result<Handle<tags::Type>, KernelErrorCode>
    where
        T: Borrow<Handle<tags::Constant>>,
    {
        self.kernel
            .borrow()
            .constant_resolve(handle)
            .map(|e| e.clone())
    }

    /// Lifting of the `constant_is_registered` function.
    #[inline]
    fn constant_is_registered<T>(&self, handle: T) -> bool
    where
        T: Borrow<Handle<tags::Constant>>,
    {
        self.kernel.borrow().constant_is_registered(handle)
    }

    /// Lifting of the `term_is_registered` function.
    #[inline]
    fn term_is_registered<T>(&self, handle: T) -> bool
    where
        T: Borrow<Handle<tags::Term>>,
    {
        self.kernel.borrow().term_is_registered(handle)
    }

    /// Lifting of the `term_register_variable` function.
    #[inline]
    fn term_register_variable<T, U>(
        &self,
        name: T,
        tau: U,
    ) -> Result<Handle<tags::Term>, KernelErrorCode>
    where
        T: Into<Name> + Clone,
        U: Into<Handle<tags::Type>> + Clone,
    {
        self.kernel.borrow_mut().term_register_variable(name, tau)
    }

    /// Lifting of the `term_register_constant` function.
    #[inline]
    fn term_register_constant<T, U, V>(
        &self,
        constant: T,
        substitution: Vec<(U, V)>,
    ) -> Result<Handle<tags::Term>, KernelErrorCode>
    where
        T: Into<Handle<tags::Constant>> + Clone,
        U: Into<Name> + Clone + Debug,
        V: Into<Handle<tags::Type>> + Clone + Debug,
    {
        self.kernel
            .borrow_mut()
            .term_register_constant(constant, substitution)
    }

    /// Lifting of the `term_register_application` function.
    #[inline]
    fn term_register_application<T, U>(
        &self,
        left: T,
        right: U,
    ) -> Result<Handle<tags::Term>, KernelErrorCode>
    where
        T: Into<Handle<tags::Term>> + Clone,
        U: Into<Handle<tags::Term>> + Clone,
    {
        self.kernel
            .borrow_mut()
            .term_register_application(left, right)
    }

    /// Lifting of the `term_register_lambda` function.
    #[inline]
    fn term_register_lambda<T, U, V>(
        &self,
        name: T,
        tau: U,
        body: V,
    ) -> Result<Handle<tags::Term>, KernelErrorCode>
    where
        T: Into<Name> + Clone,
        U: Into<Handle<tags::Type>> + Clone,
        V: Into<Handle<tags::Term>> + Clone,
    {
        self.kernel
            .borrow_mut()
            .term_register_lambda(name, tau, body)
    }

    /// Lifting of the `term_register_negation` function.
    #[inline]
    fn term_register_negation<T>(
        &self,
        body: T,
    ) -> Result<Handle<tags::Term>, KernelErrorCode>
    where
        T: Into<Handle<tags::Term>> + Clone,
    {
        self.kernel.borrow_mut().term_register_negation(body)
    }

    /// Lifting of the `term_register_conjunction` function.
    #[inline]
    fn term_register_conjunction<T, U>(
        &self,
        left: T,
        right: U,
    ) -> Result<Handle<tags::Term>, KernelErrorCode>
    where
        T: Into<Handle<tags::Term>> + Clone,
        U: Into<Handle<tags::Term>> + Clone,
    {
        self.kernel
            .borrow_mut()
            .term_register_conjunction(left, right)
    }

    /// Lifting of the `term_register_disjunction` function.
    #[inline]
    fn term_register_disjunction<T, U>(
        &self,
        left: T,
        right: U,
    ) -> Result<Handle<tags::Term>, KernelErrorCode>
    where
        T: Into<Handle<tags::Term>> + Clone,
        U: Into<Handle<tags::Term>> + Clone,
    {
        self.kernel
            .borrow_mut()
            .term_register_disjunction(left, right)
    }

    /// Lifting of the `term_register_implication` function.
    #[inline]
    fn term_register_implication<T, U>(
        &self,
        left: T,
        right: U,
    ) -> Result<Handle<tags::Term>, KernelErrorCode>
    where
        T: Into<Handle<tags::Term>> + Clone,
        U: Into<Handle<tags::Term>> + Clone,
    {
        self.kernel
            .borrow_mut()
            .term_register_implication(left, right)
    }

    /// Lifting of the `term_register_equality` function.
    #[inline]
    fn term_register_equality<T, U>(
        &self,
        left: T,
        right: U,
    ) -> Result<Handle<tags::Term>, KernelErrorCode>
    where
        T: Into<Handle<tags::Term>> + Clone,
        U: Into<Handle<tags::Term>> + Clone,
    {
        self.kernel.borrow_mut().term_register_equality(left, right)
    }

    /// Lifting of the `term_register_forall` function.
    #[inline]
    fn term_register_forall<T, U, V>(
        &self,
        name: T,
        tau: U,
        body: V,
    ) -> Result<Handle<tags::Term>, KernelErrorCode>
    where
        T: Into<Name> + Clone,
        U: Into<Handle<tags::Type>> + Clone + Debug,
        V: Into<Handle<tags::Term>> + Clone,
    {
        self.kernel
            .borrow_mut()
            .term_register_forall(name, tau, body)
    }

    /// Lifting of the `term_register_exists` function.
    #[inline]
    fn term_register_exists<T, U, V>(
        &self,
        name: T,
        tau: U,
        body: V,
    ) -> Result<Handle<tags::Term>, KernelErrorCode>
    where
        T: Into<Name> + Clone,
        U: Into<Handle<tags::Type>> + Clone + Debug,
        V: Into<Handle<tags::Term>> + Clone,
    {
        self.kernel
            .borrow_mut()
            .term_register_exists(name, tau, body)
    }

    /// Lifting of the `term_split_variable` function.
    #[inline]
    fn term_split_variable<T>(
        &self,
        handle: T,
    ) -> Result<(Name, Handle<tags::Type>), KernelErrorCode>
    where
        T: Borrow<Handle<tags::Term>>,
    {
        self.kernel
            .borrow()
            .term_split_variable(handle)
            .map(|(n, t)| (*n, t.clone()))
    }

    /// Lifting of the `term_split_constant` function.
    #[inline]
    fn term_split_constant<T>(
        &self,
        handle: T,
    ) -> Result<(Handle<tags::Constant>, Handle<tags::Type>), KernelErrorCode>
    where
        T: Borrow<Handle<tags::Term>>,
    {
        self.kernel
            .borrow()
            .term_split_constant(handle)
            .map(|(c, t)| (c.clone(), t.clone()))
    }

    /// Lifting of the `term_split_application` function.
    #[inline]
    fn term_split_application<T>(
        &self,
        handle: T,
    ) -> Result<(Handle<tags::Term>, Handle<tags::Term>), KernelErrorCode>
    where
        T: Borrow<Handle<tags::Term>>,
    {
        self.kernel
            .borrow()
            .term_split_application(handle)
            .map(|(l, r)| (l.clone(), r.clone()))
    }

    /// Lifting of the `term_split_lambda` function.
    #[inline]
    fn term_split_lambda<T>(
        &self,
        handle: T,
    ) -> Result<(Name, Handle<tags::Type>, Handle<tags::Term>), KernelErrorCode>
    where
        T: Borrow<Handle<tags::Term>>,
    {
        self.kernel
            .borrow()
            .term_split_lambda(handle)
            .map(|(n, t, b)| (*n, t.clone(), b.clone()))
    }

    /// Lifting of the `term_split_negation` function.
    #[inline]
    fn term_split_negation<T>(
        &self,
        handle: T,
    ) -> Result<Handle<tags::Term>, KernelErrorCode>
    where
        T: Borrow<Handle<tags::Term>>,
    {
        self.kernel
            .borrow()
            .term_split_negation(handle)
            .map(|n| n.clone())
    }

    /// Lifting of the `term_split_conjunction` function.
    #[inline]
    fn term_split_conjunction<T>(
        &self,
        handle: T,
    ) -> Result<(Handle<tags::Term>, Handle<tags::Term>), KernelErrorCode>
    where
        T: Borrow<Handle<tags::Term>>,
    {
        self.kernel
            .borrow()
            .term_split_conjunction(handle)
            .map(|(l, r)| (l.clone(), r.clone()))
    }

    /// Lifting of the `term_split_disjunction` function.
    #[inline]
    fn term_split_disjunction<T>(
        &self,
        handle: T,
    ) -> Result<(Handle<tags::Term>, Handle<tags::Term>), KernelErrorCode>
    where
        T: Borrow<Handle<tags::Term>>,
    {
        self.kernel
            .borrow()
            .term_split_disjunction(handle)
            .map(|(l, r)| (l.clone(), r.clone()))
    }

    /// Lifting of the `term_split_implication` function.
    #[inline]
    fn term_split_implication<T>(
        &self,
        handle: T,
    ) -> Result<(Handle<tags::Term>, Handle<tags::Term>), KernelErrorCode>
    where
        T: Borrow<Handle<tags::Term>>,
    {
        self.kernel
            .borrow()
            .term_split_implication(handle)
            .map(|(l, r)| (l.clone(), r.clone()))
    }

    /// Lifting of the `term_split_equality` function.
    #[inline]
    fn term_split_equality<T>(
        &self,
        handle: T,
    ) -> Result<(Handle<tags::Term>, Handle<tags::Term>), KernelErrorCode>
    where
        T: Borrow<Handle<tags::Term>>,
    {
        self.kernel
            .borrow()
            .term_split_equality(handle)
            .map(|(l, r)| (l.clone(), r.clone()))
    }

    /// Lifting of the `term_split_forall` function.
    #[inline]
    fn term_split_forall<T>(
        &self,
        handle: T,
    ) -> Result<(Name, Handle<tags::Type>, Handle<tags::Term>), KernelErrorCode>
    where
        T: Borrow<Handle<tags::Term>>,
    {
        self.kernel
            .borrow()
            .term_split_forall(handle)
            .map(|(n, t, b)| (*n, t.clone(), b.clone()))
    }

    /// Lifting of the `term_split_exists` function.
    #[inline]
    fn term_split_exists<T>(
        &self,
        handle: T,
    ) -> Result<(Name, Handle<tags::Type>, Handle<tags::Term>), KernelErrorCode>
    where
        T: Borrow<Handle<tags::Term>>,
    {
        self.kernel
            .borrow()
            .term_split_exists(handle)
            .map(|(n, t, b)| (*n, t.clone(), b.clone()))
    }

    /// Lifting of the `term_test_variable` function.
    #[inline]
    fn term_test_variable<T>(&self, handle: T) -> Result<bool, KernelErrorCode>
    where
        T: Borrow<Handle<tags::Term>>,
    {
        self.kernel.borrow().term_test_variable(handle)
    }

    /// Lifting of the `term_test_constant` function.
    #[inline]
    fn term_test_constant<T>(&self, handle: T) -> Result<bool, KernelErrorCode>
    where
        T: Borrow<Handle<tags::Term>>,
    {
        self.kernel.borrow().term_test_constant(handle)
    }

    /// Lifting of the `term_test_application` function.
    #[inline]
    fn term_test_application<T>(
        &self,
        handle: T,
    ) -> Result<bool, KernelErrorCode>
    where
        T: Borrow<Handle<tags::Term>>,
    {
        self.kernel.borrow().term_test_application(handle)
    }

    /// Lifting of the `term_test_lambda` function.
    #[inline]
    fn term_test_lambda<T>(&self, handle: T) -> Result<bool, KernelErrorCode>
    where
        T: Borrow<Handle<tags::Term>>,
    {
        self.kernel.borrow().term_test_lambda(handle)
    }

    /// Lifting of the `term_test_negation` function.
    #[inline]
    fn term_test_negation<T>(&self, handle: T) -> Result<bool, KernelErrorCode>
    where
        T: Borrow<Handle<tags::Term>>,
    {
        self.kernel.borrow().term_test_negation(handle)
    }

    /// Lifting of the `term_test_conjunction` function.
    #[inline]
    fn term_test_conjunction<T>(
        &self,
        handle: T,
    ) -> Result<bool, KernelErrorCode>
    where
        T: Borrow<Handle<tags::Term>>,
    {
        self.kernel.borrow().term_test_conjunction(handle)
    }

    /// Lifting of the `term_test_disjunction` function.
    #[inline]
    fn term_test_disjunction<T>(
        &self,
        handle: T,
    ) -> Result<bool, KernelErrorCode>
    where
        T: Borrow<Handle<tags::Term>>,
    {
        self.kernel.borrow().term_test_disjunction(handle)
    }

    /// Lifting of the `term_test_implication` function.
    #[inline]
    fn term_test_implication<T>(
        &self,
        handle: T,
    ) -> Result<bool, KernelErrorCode>
    where
        T: Borrow<Handle<tags::Term>>,
    {
        self.kernel.borrow().term_test_implication(handle)
    }

    /// Lifting of the `term_test_equality` function.
    #[inline]
    fn term_test_equality<T>(&self, handle: T) -> Result<bool, KernelErrorCode>
    where
        T: Borrow<Handle<tags::Term>>,
    {
        self.kernel.borrow().term_test_equality(handle)
    }

    /// Lifting of the `term_test_forall` function.
    #[inline]
    fn term_test_forall<T>(&self, handle: T) -> Result<bool, KernelErrorCode>
    where
        T: Borrow<Handle<tags::Term>>,
    {
        self.kernel.borrow().term_test_forall(handle)
    }

    /// Lifting of the `term_test_exists` function.
    #[inline]
    fn term_test_exists<T>(&self, handle: T) -> Result<bool, KernelErrorCode>
    where
        T: Borrow<Handle<tags::Term>>,
    {
        self.kernel.borrow().term_test_exists(handle)
    }

    /// Lifting of the `term_free_variables` function.
    #[inline]
    fn term_free_variables<T>(
        &self,
        handle: T,
    ) -> Result<Vec<(Name, Handle<tags::Type>)>, KernelErrorCode>
    where
        T: Borrow<Handle<tags::Term>>,
    {
        self.kernel
            .borrow()
            .term_free_variables(handle)
            .map(|v| v.iter().cloned().map(|(n, t)| (*n, t.clone())).collect())
    }

    /// Lifting of the `term_type_variables` function.
    #[inline]
    fn term_type_variables<T>(
        &self,
        handle: T,
    ) -> Result<Vec<Name>, KernelErrorCode>
    where
        T: Borrow<Handle<tags::Term>>,
    {
        self.kernel
            .borrow()
            .term_type_variables(handle)
            .map(|v| v.iter().cloned().cloned().collect())
    }

    /// Lifting of the `term_substitution` function.
    #[inline]
    fn term_substitute<T, N, U, V>(
        &self,
        handle: T,
        substitution: Vec<((N, U), V)>,
    ) -> Result<Handle<tags::Term>, KernelErrorCode>
    where
        T: Into<Handle<tags::Term>>,
        N: Into<Name> + Clone,
        U: Into<Handle<tags::Type>> + Clone,
        V: Into<Handle<tags::Term>> + Clone,
    {
        self.kernel
            .borrow_mut()
            .term_substitution(handle, substitution)
    }

    /// Lifting of the `term_type_substitute` function.
    #[inline]
    fn term_type_substitute<T, U, V>(
        &self,
        handle: T,
        substitution: Vec<(U, V)>,
    ) -> Result<Handle<tags::Term>, KernelErrorCode>
    where
        T: Into<Handle<tags::Term>>,
        U: Into<Name> + Clone + Debug,
        V: Into<Handle<tags::Type>> + Clone + Debug,
    {
        self.kernel
            .borrow_mut()
            .term_type_substitute(handle, substitution)
    }

    /// Lifting of the `term_type_infer` function.
    #[inline]
    fn term_type_infer<T>(
        &self,
        handle: T,
    ) -> Result<Handle<tags::Type>, KernelErrorCode>
    where
        T: Borrow<Handle<tags::Term>>,
    {
        self.kernel.borrow_mut().term_type_infer(handle)
    }

    /// Lifting of the `term_type_is_proposition` function.
    #[inline]
    fn term_type_is_proposition<T>(
        &self,
        handle: T,
    ) -> Result<bool, KernelErrorCode>
    where
        T: Borrow<Handle<tags::Term>>,
    {
        self.kernel.borrow_mut().term_type_is_proposition(handle)
    }

    /// Lifting of the `theorem_is_registered` function.
    #[inline]
    fn theorem_is_registered<T>(&self, handle: T) -> bool
    where
        T: Borrow<Handle<tags::Theorem>>,
    {
        self.kernel.borrow().theorem_is_registered(handle)
    }

    /// Lifting of the `theorem_register_assumption` function.
    #[inline]
    fn theorem_register_assumption<T>(
        &self,
        term_handle: T,
    ) -> Result<Handle<tags::Theorem>, KernelErrorCode>
    where
        T: Into<Handle<tags::Term>> + Clone,
    {
        self.kernel
            .borrow_mut()
            .theorem_register_assumption(term_handle)
    }

    /// Lifting of the `theorem_register_reflexivity` function.
    #[inline]
    fn theorem_register_reflexivity<T>(
        &self,
        term_handle: T,
    ) -> Result<Handle<tags::Theorem>, KernelErrorCode>
    where
        T: Into<Handle<tags::Term>> + Clone,
    {
        self.kernel
            .borrow_mut()
            .theorem_register_reflexivity(term_handle)
    }

    /// Lifting of the `theorem_register_symmetry` function.
    #[inline]
    fn theorem_register_symmetry<T>(
        &self,
        theorem_handle: T,
    ) -> Result<Handle<tags::Theorem>, KernelErrorCode>
    where
        T: Borrow<Handle<tags::Theorem>>,
    {
        self.kernel
            .borrow_mut()
            .theorem_register_symmetry(theorem_handle)
    }

    /// Lifting of the `theorem_register_transitivity` function.
    #[inline]
    fn theorem_register_transitivity<T, U>(
        &self,
        left_handle: T,
        right_handle: U,
    ) -> Result<Handle<tags::Theorem>, KernelErrorCode>
    where
        T: Borrow<Handle<tags::Theorem>>,
        U: Borrow<Handle<tags::Theorem>>,
    {
        self.kernel
            .borrow_mut()
            .theorem_register_transitivity(left_handle, right_handle)
    }

    /// Lifting of the `theorem_register_beta` function.
    #[inline]
    fn theorem_register_beta<T>(
        &self,
        term_handle: T,
    ) -> Result<Handle<tags::Theorem>, KernelErrorCode>
    where
        T: Into<Handle<tags::Term>> + Clone,
    {
        self.kernel.borrow_mut().theorem_register_beta(term_handle)
    }

    /// Lifting of the `theorem_register_eta` function.
    #[inline]
    fn theorem_register_eta<T>(
        &self,
        term_handle: T,
    ) -> Result<Handle<tags::Theorem>, KernelErrorCode>
    where
        T: Into<Handle<tags::Term>> + Clone,
    {
        self.kernel.borrow_mut().theorem_register_eta(term_handle)
    }

    /// Lifting of the `theorem_register_substitute` function.
    #[inline]
    fn theorem_register_substitute<T, U, V>(
        &self,
        theorem_handle: T,
        substitution: Vec<((Name, U), V)>,
    ) -> Result<Handle<tags::Theorem>, KernelErrorCode>
    where
        T: Borrow<Handle<tags::Theorem>>,
        U: Into<Handle<tags::Type>> + Clone,
        V: Into<Handle<tags::Term>> + Clone,
    {
        self.kernel
            .borrow_mut()
            .theorem_register_substitute(theorem_handle, substitution)
    }

    /// Lifting of the `theorem_register_type_substitute` function.
    #[inline]
    fn theorem_register_type_substitute<T, U>(
        &self,
        theorem_handle: T,
        substitution: Vec<(Name, U)>,
    ) -> Result<Handle<tags::Theorem>, KernelErrorCode>
    where
        T: Borrow<Handle<tags::Theorem>>,
        U: Into<Handle<tags::Type>> + Clone + Debug,
    {
        self.kernel
            .borrow_mut()
            .theorem_register_type_substitute(theorem_handle, substitution)
    }

    /// Lifting of the `theorem_register_application` function.
    #[inline]
    fn theorem_register_application<T, U>(
        &self,
        left_handle: T,
        right_handle: U,
    ) -> Result<Handle<tags::Theorem>, KernelErrorCode>
    where
        T: Borrow<Handle<tags::Theorem>>,
        U: Borrow<Handle<tags::Theorem>>,
    {
        self.kernel
            .borrow_mut()
            .theorem_register_application(left_handle, right_handle)
    }

    /// Lifting of the `theorem_register_lambda` function.
    #[inline]
    fn theorem_register_lambda<T, U, V>(
        &self,
        name: T,
        type_handle: U,
        body_handle: V,
    ) -> Result<Handle<tags::Theorem>, KernelErrorCode>
    where
        T: Into<Name> + Clone,
        U: Into<Handle<tags::Type>> + Clone,
        V: Borrow<Handle<tags::Theorem>>,
    {
        self.kernel.borrow_mut().theorem_register_lambda(
            name,
            type_handle,
            body_handle,
        )
    }

    /// Lifting of the `theorem_register_truth_introduction` function.
    #[inline]
    fn theorem_register_truth_introduction(
        &self,
    ) -> Result<Handle<tags::Theorem>, KernelErrorCode> {
        self.kernel
            .borrow_mut()
            .theorem_register_truth_introduction()
    }

    /// Lifting of the `theorem_register_falsity_elimination` function.
    #[inline]
    fn theorem_register_falsity_elimination<T, U>(
        &self,
        theorem_handle: T,
        term_handle: U,
    ) -> Result<Handle<tags::Theorem>, KernelErrorCode>
    where
        T: Borrow<Handle<tags::Theorem>>,
        U: Into<Handle<tags::Term>> + Clone,
    {
        self.kernel
            .borrow_mut()
            .theorem_register_falsity_elimination(theorem_handle, term_handle)
    }

    /// Lifting of the `theorem_register_conjunction_introduction` function.
    #[inline]
    fn theorem_register_conjunction_introduction<T, U>(
        &self,
        left_handle: T,
        right_handle: U,
    ) -> Result<Handle<tags::Theorem>, KernelErrorCode>
    where
        T: Borrow<Handle<tags::Theorem>>,
        U: Borrow<Handle<tags::Theorem>>,
    {
        self.kernel
            .borrow_mut()
            .theorem_register_conjunction_introduction(
                left_handle,
                right_handle,
            )
    }

    /// Lifting of the `theorem_register_conjunction_left_elimination` function.
    #[inline]
    fn theorem_register_conjunction_left_elimination<T>(
        &self,
        theorem_handle: T,
    ) -> Result<Handle<tags::Theorem>, KernelErrorCode>
    where
        T: Borrow<Handle<tags::Theorem>>,
    {
        self.kernel
            .borrow_mut()
            .theorem_register_conjunction_left_elimination(theorem_handle)
    }

    /// Lifting of the `theorem_register_conjunction_right_elimination` function.
    #[inline]
    fn theorem_register_conjunction_right_elimination<T>(
        &self,
        theorem_handle: T,
    ) -> Result<Handle<tags::Theorem>, KernelErrorCode>
    where
        T: Borrow<Handle<tags::Theorem>>,
    {
        self.kernel
            .borrow_mut()
            .theorem_register_conjunction_right_elimination(theorem_handle)
    }

    /// Lifting of the `theorem_register_disjunction_left_introduction`
    /// function.
    #[inline]
    fn theorem_register_disjunction_left_introduction<T, U>(
        &self,
        theorem_handle: T,
        term_handle: U,
    ) -> Result<Handle<tags::Theorem>, KernelErrorCode>
    where
        T: Borrow<Handle<tags::Theorem>>,
        U: Into<Handle<tags::Term>> + Clone,
    {
        self.kernel
            .borrow_mut()
            .theorem_register_disjunction_left_introduction(
                theorem_handle,
                term_handle,
            )
    }

    /// Lifting of the `theorem_register_disjunction_right_introduction`
    /// function.
    #[inline]
    fn theorem_register_disjunction_right_introduction<T, U>(
        &self,
        theorem_handle: T,
        term_handle: U,
    ) -> Result<Handle<tags::Theorem>, KernelErrorCode>
    where
        T: Borrow<Handle<tags::Theorem>>,
        U: Into<Handle<tags::Term>> + Clone,
    {
        self.kernel
            .borrow_mut()
            .theorem_register_disjunction_right_introduction(
                theorem_handle,
                term_handle,
            )
    }

    /// Lifting of the `theorem_register_disjunction_right_introduction`
    /// function.
    #[inline]
    fn theorem_register_disjunction_elimination<T, U, V>(
        &self,
        left_handle: T,
        mid_handle: U,
        right_handle: V,
    ) -> Result<Handle<tags::Theorem>, KernelErrorCode>
    where
        T: Borrow<Handle<tags::Theorem>>,
        U: Borrow<Handle<tags::Theorem>>,
        V: Borrow<Handle<tags::Theorem>>,
    {
        self.kernel
            .borrow_mut()
            .theorem_register_disjunction_elimination(
                left_handle,
                mid_handle,
                right_handle,
            )
    }

    /// Lifting of the `theorem_register_negation_introduction` function.
    #[inline]
    fn theorem_register_negation_introduction<T, U>(
        &self,
        theorem_handle: T,
        term_handle: U,
    ) -> Result<Handle<tags::Theorem>, KernelErrorCode>
    where
        T: Borrow<Handle<tags::Theorem>>,
        U: Into<Handle<tags::Term>> + Clone,
    {
        self.kernel
            .borrow_mut()
            .theorem_register_negation_introduction(theorem_handle, term_handle)
    }

    /// Lifting of the `theorem_register_negation_elimination` function.
    #[inline]
    fn theorem_register_negation_elimination<T, U>(
        &self,
        left_handle: T,
        right_handle: U,
    ) -> Result<Handle<tags::Theorem>, KernelErrorCode>
    where
        T: Borrow<Handle<tags::Theorem>>,
        U: Borrow<Handle<tags::Theorem>>,
    {
        self.kernel
            .borrow_mut()
            .theorem_register_negation_elimination(left_handle, right_handle)
    }

    /// Lifting of the `theorem_register_implication_introduction` function.
    #[inline]
    fn theorem_register_implication_introduction<T, U>(
        &self,
        theorem_handle: T,
        term_handle: U,
    ) -> Result<Handle<tags::Theorem>, KernelErrorCode>
    where
        T: Borrow<Handle<tags::Theorem>>,
        U: Into<Handle<tags::Term>> + Clone,
    {
        self.kernel
            .borrow_mut()
            .theorem_register_implication_introduction(
                theorem_handle,
                term_handle,
            )
    }

    /// Lifting of the `theorem_register_implication_elimination` function.
    #[inline]
    fn theorem_register_implication_elimination<T, U>(
        &self,
        left_handle: T,
        right_handle: U,
    ) -> Result<Handle<tags::Theorem>, KernelErrorCode>
    where
        T: Borrow<Handle<tags::Theorem>>,
        U: Borrow<Handle<tags::Theorem>>,
    {
        self.kernel
            .borrow_mut()
            .theorem_register_implication_elimination(left_handle, right_handle)
    }

    /// Lifting of the `theorem_register_iff_introduction` function.
    #[inline]
    fn theorem_register_iff_introduction<T, U>(
        &self,
        left_handle: T,
        right_handle: U,
    ) -> Result<Handle<tags::Theorem>, KernelErrorCode>
    where
        T: Borrow<Handle<tags::Theorem>>,
        U: Borrow<Handle<tags::Theorem>>,
    {
        self.kernel
            .borrow_mut()
            .theorem_register_iff_introduction(left_handle, right_handle)
    }

    /// Lifting of the `theorem_register_iff_left_elimination` function.
    #[inline]
    fn theorem_register_iff_left_elimination<T>(
        &self,
        theorem_handle: T,
    ) -> Result<Handle<tags::Theorem>, KernelErrorCode>
    where
        T: Borrow<Handle<tags::Theorem>>,
    {
        self.kernel
            .borrow_mut()
            .theorem_register_iff_left_elimination(theorem_handle)
    }

    /// Lifting of the `theorem_register_forall_introduction` function.
    #[inline]
    fn theorem_register_forall_introduction<T, U, V>(
        &self,
        theorem_handle: T,
        name: U,
        _type: V,
    ) -> Result<Handle<tags::Theorem>, KernelErrorCode>
    where
        T: Borrow<Handle<tags::Theorem>>,
        U: Into<Name>,
        V: Into<Handle<tags::Type>> + Clone + Debug,
    {
        self.kernel
            .borrow_mut()
            .theorem_register_forall_introduction(theorem_handle, name, _type)
    }

    /// Lifting of the `theorem_register_forall_elimination` function.
    #[inline]
    fn theorem_register_forall_elimination<T, U>(
        &self,
        theorem_handle: T,
        term_handle: U,
    ) -> Result<Handle<tags::Theorem>, KernelErrorCode>
    where
        T: Borrow<Handle<tags::Theorem>>,
        U: Into<Handle<tags::Term>>,
    {
        self.kernel
            .borrow_mut()
            .theorem_register_forall_elimination(theorem_handle, term_handle)
    }

    /// Lifting of the `theorem_register_exists_introduction` function.
    #[inline]
    fn theorem_register_exists_introduction<T, U>(
        &self,
        theorem_handle: T,
        term_handle: U,
    ) -> Result<Handle<tags::Theorem>, KernelErrorCode>
    where
        T: Borrow<Handle<tags::Theorem>>,
        U: Into<Handle<tags::Term>>,
    {
        self.kernel
            .borrow_mut()
            .theorem_register_exists_introduction(theorem_handle, term_handle)
    }

    /// Lifting of the `theorem_register_exists_elimination` function.
    #[inline]
    fn theorem_register_exists_elimination<T, U>(
        &self,
        left_handle: T,
        right_handle: U,
    ) -> Result<Handle<tags::Theorem>, KernelErrorCode>
    where
        T: Borrow<Handle<tags::Theorem>>,
        U: Borrow<Handle<tags::Theorem>>,
    {
        self.kernel
            .borrow_mut()
            .theorem_register_exists_elimination(left_handle, right_handle)
    }

    /// Lifting of the `theorem_split_conclusion` function.
    #[inline]
    fn theorem_split_conclusion<T>(
        &self,
        handle: T,
    ) -> Result<Handle<tags::Term>, KernelErrorCode>
    where
        T: Borrow<Handle<tags::Theorem>>,
    {
        self.kernel.borrow().theorem_split_conclusion(handle)
    }

    /// Lifting of the `theorem_split_premisses` function.
    #[inline]
    fn theorem_split_premisses<T>(
        &self,
        handle: T,
    ) -> Result<Vec<Handle<tags::Term>>, KernelErrorCode>
    where
        T: Borrow<Handle<tags::Theorem>>,
    {
        self.kernel.borrow().theorem_split_premisses(handle)
    }
}

////////////////////////////////////////////////////////////////////////////////
// ABI binding.
////////////////////////////////////////////////////////////////////////////////

/// Dispatches on an ABI host-call number, and calls the respective function on
/// the machine's runtime state.
impl Externals for WasmiRuntimeState {
    fn invoke_index(
        &mut self,
        index: usize,
        args: RuntimeArgs,
    ) -> Result<Option<RuntimeValue>, Trap> {
        match index {
            ABI_TYPE_FORMER_RESOLVE_INDEX => {
                let handle = args.nth::<semantic_types::Handle>(0);
                let result_address = args.nth::<semantic_types::Pointer>(1);

                let arity = match self
                    .type_former_resolve(&Handle::from(handle as usize))
                {
                    None => {
                        return Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::NoSuchTypeFormerRegistered.into(),
                        )))
                    }
                    Some(arity) => arity,
                };

                self.write_u64(result_address, arity as u64)?;

                Ok(Some(RuntimeValue::I32(KernelErrorCode::Success.into())))
            }
            ABI_TYPE_FORMER_IS_REGISTERED_INDEX => {
                let handle = args.nth::<semantic_types::Handle>(0);
                let result = self
                    .type_former_is_registered(&Handle::from(handle as usize));

                Ok(Some(RuntimeValue::I32(result.into())))
            }
            ABI_TYPE_FORMER_REGISTER_INDEX => {
                let arity = args.nth::<semantic_types::Arity>(0);
                let result = self.type_former_register(arity as usize);

                Ok(Some(RuntimeValue::I64(*result as i64)))
            }
            ABI_TYPE_REGISTER_VARIABLE_INDEX => {
                let name = args.nth::<semantic_types::Name>(0);
                let result = self.type_register_variable(name);

                Ok(Some(RuntimeValue::I64(*result as i64)))
            }
            ABI_TYPE_REGISTER_COMBINATION_INDEX => {
                let former_handle: Handle<tags::TypeFormer> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let argument_base = args.nth::<semantic_types::Pointer>(1);
                let argument_length = args.nth::<semantic_types::Size>(2);
                let result_ptr = args.nth::<semantic_types::Pointer>(3);

                let arguments =
                    self.read_handles(argument_base, argument_length as usize)?;

                match self.type_register_combination(former_handle, arguments) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_handle(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_TYPE_REGISTER_FUNCTION_INDEX => {
                let domain_handle: Handle<tags::Type> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let range_handle: Handle<tags::Type> = Handle::from(
                    args.nth::<semantic_types::Handle>(1) as usize,
                );
                let result_ptr = args.nth::<semantic_types::Pointer>(2);

                match self.type_register_function(domain_handle, range_handle) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_handle(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_TYPE_IS_REGISTERED_INDEX => {
                let type_handle: Handle<tags::Type> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                Ok(Some(RuntimeValue::I32(
                    self.type_is_registered(type_handle).into(),
                )))
            }
            ABI_TYPE_SPLIT_VARIABLE_INDEX => {
                let type_handle: Handle<tags::Type> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let result_ptr = args.nth::<semantic_types::Pointer>(1);

                match self.type_split_variable(type_handle) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_u64(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_TYPE_SPLIT_COMBINATION_INDEX => {
                let type_handle: Handle<tags::Type> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let former_result_ptr = args.nth::<semantic_types::Pointer>(1);
                let arguments_result_ptr =
                    args.nth::<semantic_types::Pointer>(2);
                let arguments_length_result_ptr =
                    args.nth::<semantic_types::Pointer>(3);

                match self.type_split_combination(type_handle) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok((former_handle, arguments)) => {
                        self.write_handle(former_result_ptr, former_handle)?;
                        self.write_u64(
                            arguments_length_result_ptr,
                            arguments.len() as u64,
                        )?;
                        self.write_handles(arguments_result_ptr, arguments)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_TYPE_SPLIT_FUNCTION_INDEX => {
                let type_handle: Handle<tags::Type> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let domain_result_ptr = args.nth::<semantic_types::Pointer>(1);
                let range_result_ptr = args.nth::<semantic_types::Pointer>(2);

                match self.type_split_function(type_handle) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok((domain_handle, range_handle)) => {
                        self.write_handle(domain_result_ptr, domain_handle)?;
                        self.write_handle(range_result_ptr, range_handle)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_TYPE_TEST_VARIABLE_INDEX => {
                let type_handle: Handle<tags::Type> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let result_ptr = args.nth::<semantic_types::Pointer>(1);

                match self.type_test_variable(type_handle) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_bool(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_TYPE_TEST_COMBINATION_INDEX => {
                let type_handle: Handle<tags::Type> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let result_ptr = args.nth::<semantic_types::Pointer>(1);

                match self.type_test_combination(type_handle) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_bool(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_TYPE_TEST_FUNCTION_INDEX => {
                let type_handle: Handle<tags::Type> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let result_ptr = args.nth::<semantic_types::Pointer>(1);

                match self.type_test_function(type_handle) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_bool(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_TYPE_SIZE_INDEX => {
                let type_handle: Handle<tags::Type> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let result_ptr = args.nth::<semantic_types::Pointer>(1);

                match self.type_size(type_handle) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_u64(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_TYPE_VARIABLES_INDEX => {
                let type_handle: Handle<tags::Type> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let variable_result_ptr =
                    args.nth::<semantic_types::Pointer>(1);
                let variable_len_ptr = args.nth::<semantic_types::Pointer>(2);

                match self.type_variables(type_handle) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_u64s(variable_result_ptr, result.clone())?;
                        self.write_u64(variable_len_ptr, result.len() as u64)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_TYPE_SUBSTITUTE_INDEX => {
                let type_handle: Handle<tags::Type> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let dom_ptr = args.nth::<semantic_types::Pointer>(1);
                let dom_len = args.nth::<semantic_types::Size>(2);
                let rng_ptr = args.nth::<semantic_types::Pointer>(3);
                let rng_len = args.nth::<semantic_types::Size>(4);
                let result_ptr = args.nth::<semantic_types::Pointer>(5);

                let domains = self.read_u64s(dom_ptr, dom_len as usize)?;
                let ranges = self.read_handles(rng_ptr, rng_len as usize)?;

                let subst =
                    domains.iter().zip(ranges).map(|(d, r)| (*d, r)).collect();

                match self.type_substitute(type_handle, subst) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_handle(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_CONSTANT_REGISTER_INDEX => {
                let type_handle: Handle<tags::Type> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let result_ptr = args.nth::<semantic_types::Pointer>(1);

                match self.constant_register(type_handle) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_handle(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_CONSTANT_IS_REGISTERED_INDEX => {
                let constant_handle: Handle<tags::Constant> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );

                Ok(Some(RuntimeValue::I32(
                    self.constant_is_registered(constant_handle).into(),
                )))
            }
            ABI_CONSTANT_RESOLVE_INDEX => {
                let constant_handle: Handle<tags::Constant> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let result_ptr = args.nth::<semantic_types::Pointer>(1);

                match self.constant_resolve(constant_handle) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_handle(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_TERM_IS_REGISTERED_INDEX => {
                let term_handle: Handle<tags::Term> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );

                let result = self.term_is_registered(term_handle);

                Ok(Some(RuntimeValue::I32(if result { 0 } else { 1 })))
            }
            ABI_TERM_REGISTER_VARIABLE_INDEX => {
                let name = args.nth::<semantic_types::Name>(0);
                let type_handle: Handle<tags::Type> = Handle::from(
                    args.nth::<semantic_types::Handle>(1) as usize,
                );
                let result_ptr = args.nth::<semantic_types::Pointer>(2);

                match self.term_register_variable(name, type_handle) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_handle(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_TERM_REGISTER_CONSTANT_INDEX => {
                let constant_handle: Handle<tags::Constant> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let dom_ptr = args.nth::<semantic_types::Pointer>(1);
                let dom_len = args.nth::<semantic_types::Size>(2);
                let rng_ptr = args.nth::<semantic_types::Pointer>(3);
                let rng_len = args.nth::<semantic_types::Size>(4);
                let result_ptr = args.nth::<semantic_types::Pointer>(5);

                let domains = self.read_u64s(dom_ptr, dom_len as usize)?;
                let ranges = self.read_handles(rng_ptr, rng_len as usize)?;

                let subst =
                    domains.iter().zip(ranges).map(|(d, r)| (*d, r)).collect();

                match self.term_register_constant(constant_handle, subst) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_handle(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_TERM_REGISTER_APPLICATION_INDEX => {
                let left_handle: Handle<tags::Term> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let right_handle: Handle<tags::Term> = Handle::from(
                    args.nth::<semantic_types::Handle>(1) as usize,
                );
                let result_ptr = args.nth::<semantic_types::Pointer>(2);

                match self.term_register_application(left_handle, right_handle)
                {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_handle(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_TERM_REGISTER_LAMBDA_INDEX => {
                let name = args.nth::<semantic_types::Name>(0);
                let type_handle: Handle<tags::Type> = Handle::from(
                    args.nth::<semantic_types::Handle>(1) as usize,
                );
                let body_handle: Handle<tags::Term> = Handle::from(
                    args.nth::<semantic_types::Handle>(2) as usize,
                );
                let result_ptr = args.nth::<semantic_types::Pointer>(3);

                match self.term_register_lambda(name, type_handle, body_handle)
                {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_handle(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_TERM_REGISTER_NEGATION_INDEX => {
                let term_handle: Handle<tags::Term> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let result_ptr = args.nth::<semantic_types::Pointer>(1);

                match self.term_register_negation(term_handle) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_handle(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_TERM_REGISTER_CONJUNCTION_INDEX => {
                let left_handle: Handle<tags::Term> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let right_handle: Handle<tags::Term> = Handle::from(
                    args.nth::<semantic_types::Handle>(1) as usize,
                );
                let result_ptr = args.nth::<semantic_types::Pointer>(2);

                match self.term_register_conjunction(left_handle, right_handle)
                {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_handle(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_TERM_REGISTER_DISJUNCTION_INDEX => {
                let left_handle: Handle<tags::Term> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let right_handle: Handle<tags::Term> = Handle::from(
                    args.nth::<semantic_types::Handle>(1) as usize,
                );
                let result_ptr = args.nth::<semantic_types::Pointer>(2);

                match self.term_register_disjunction(left_handle, right_handle)
                {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_handle(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_TERM_REGISTER_IMPLICATION_INDEX => {
                let left_handle: Handle<tags::Term> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let right_handle: Handle<tags::Term> = Handle::from(
                    args.nth::<semantic_types::Handle>(1) as usize,
                );
                let result_ptr = args.nth::<semantic_types::Pointer>(2);

                match self.term_register_implication(left_handle, right_handle)
                {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_handle(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_TERM_REGISTER_EQUALITY_INDEX => {
                let left_handle: Handle<tags::Term> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let right_handle: Handle<tags::Term> = Handle::from(
                    args.nth::<semantic_types::Handle>(1) as usize,
                );
                let result_ptr = args.nth::<semantic_types::Pointer>(2);

                match self.term_register_equality(left_handle, right_handle) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_handle(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_TERM_REGISTER_FORALL_INDEX => {
                let name = args.nth::<semantic_types::Name>(0);
                let type_handle: Handle<tags::Type> = Handle::from(
                    args.nth::<semantic_types::Handle>(1) as usize,
                );
                let body_handle: Handle<tags::Term> = Handle::from(
                    args.nth::<semantic_types::Handle>(2) as usize,
                );
                let result_ptr = args.nth::<semantic_types::Pointer>(3);

                match self.term_register_forall(name, type_handle, body_handle)
                {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_handle(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_TERM_REGISTER_EXISTS_INDEX => {
                let name = args.nth::<semantic_types::Name>(0);
                let type_handle: Handle<tags::Type> = Handle::from(
                    args.nth::<semantic_types::Handle>(1) as usize,
                );
                let body_handle: Handle<tags::Term> = Handle::from(
                    args.nth::<semantic_types::Handle>(2) as usize,
                );
                let result_ptr = args.nth::<semantic_types::Pointer>(3);

                match self.term_register_exists(name, type_handle, body_handle)
                {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_handle(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_TERM_SPLIT_VARIABLE_INDEX => {
                let term_handle: Handle<tags::Term> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let result_name_ptr = args.nth::<semantic_types::Pointer>(1);
                let result_type_ptr = args.nth::<semantic_types::Pointer>(2);

                match self.term_split_variable(term_handle) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok((name, tau)) => {
                        self.write_u64(result_name_ptr, name)?;
                        self.write_handle(result_type_ptr, tau)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_TERM_SPLIT_CONSTANT_INDEX => {
                let term_handle: Handle<tags::Term> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let result_const_ptr = args.nth::<semantic_types::Pointer>(1);
                let result_type_ptr = args.nth::<semantic_types::Pointer>(2);

                match self.term_split_constant(term_handle) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok((constant, tau)) => {
                        self.write_handle(result_const_ptr, constant)?;
                        self.write_handle(result_type_ptr, tau)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_TERM_SPLIT_APPLICATION_INDEX => {
                let term_handle: Handle<tags::Term> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let result_left_ptr = args.nth::<semantic_types::Pointer>(1);
                let result_right_ptr = args.nth::<semantic_types::Pointer>(2);

                match self.term_split_application(term_handle) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok((left, right)) => {
                        self.write_handle(result_left_ptr, left)?;
                        self.write_handle(result_right_ptr, right)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_TERM_SPLIT_LAMBDA_INDEX => {
                let term_handle: Handle<tags::Term> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let result_name_ptr = args.nth::<semantic_types::Pointer>(1);
                let result_type_ptr = args.nth::<semantic_types::Pointer>(2);
                let result_body_ptr = args.nth::<semantic_types::Pointer>(3);

                match self.term_split_lambda(term_handle) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok((name, tau, body)) => {
                        self.write_u64(result_name_ptr, name)?;
                        self.write_handle(result_type_ptr, tau)?;
                        self.write_handle(result_body_ptr, body)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_TERM_SPLIT_NEGATION_INDEX => {
                let term_handle: Handle<tags::Term> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let result_body_ptr = args.nth::<semantic_types::Pointer>(1);

                match self.term_split_negation(term_handle) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(body) => {
                        self.write_handle(result_body_ptr, body)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_TERM_SPLIT_CONJUNCTION_INDEX => {
                let term_handle: Handle<tags::Term> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let result_left_ptr = args.nth::<semantic_types::Pointer>(1);
                let result_right_ptr = args.nth::<semantic_types::Pointer>(2);

                match self.term_split_conjunction(term_handle) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok((left, right)) => {
                        self.write_handle(result_left_ptr, left)?;
                        self.write_handle(result_right_ptr, right)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_TERM_SPLIT_DISJUNCTION_INDEX => {
                let term_handle: Handle<tags::Term> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let result_left_ptr = args.nth::<semantic_types::Pointer>(1);
                let result_right_ptr = args.nth::<semantic_types::Pointer>(2);

                match self.term_split_disjunction(term_handle) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok((left, right)) => {
                        self.write_handle(result_left_ptr, left)?;
                        self.write_handle(result_right_ptr, right)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_TERM_SPLIT_IMPLICATION_INDEX => {
                let term_handle: Handle<tags::Term> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let result_left_ptr = args.nth::<semantic_types::Pointer>(1);
                let result_right_ptr = args.nth::<semantic_types::Pointer>(2);

                match self.term_split_implication(term_handle) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok((left, right)) => {
                        self.write_handle(result_left_ptr, left)?;
                        self.write_handle(result_right_ptr, right)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_TERM_SPLIT_EQUALITY_INDEX => {
                let term_handle: Handle<tags::Term> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let result_left_ptr = args.nth::<semantic_types::Pointer>(1);
                let result_right_ptr = args.nth::<semantic_types::Pointer>(2);

                match self.term_split_equality(term_handle) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok((left, right)) => {
                        self.write_handle(result_left_ptr, left)?;
                        self.write_handle(result_right_ptr, right)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_TERM_SPLIT_FORALL_INDEX => {
                let term_handle: Handle<tags::Term> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let result_name_ptr = args.nth::<semantic_types::Pointer>(1);
                let result_type_ptr = args.nth::<semantic_types::Pointer>(2);
                let result_body_ptr = args.nth::<semantic_types::Pointer>(3);

                match self.term_split_forall(term_handle) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok((name, tau, body)) => {
                        self.write_u64(result_name_ptr, name)?;
                        self.write_handle(result_type_ptr, tau)?;
                        self.write_handle(result_body_ptr, body)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_TERM_SPLIT_EXISTS_INDEX => {
                let term_handle: Handle<tags::Term> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let result_name_ptr = args.nth::<semantic_types::Pointer>(1);
                let result_type_ptr = args.nth::<semantic_types::Pointer>(2);
                let result_body_ptr = args.nth::<semantic_types::Pointer>(3);

                match self.term_split_exists(term_handle) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok((name, tau, body)) => {
                        self.write_u64(result_name_ptr, name)?;
                        self.write_handle(result_type_ptr, tau)?;
                        self.write_handle(result_body_ptr, body)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_TERM_TEST_VARIABLE_INDEX => {
                let term_handle: Handle<tags::Term> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let result_ptr = args.nth::<semantic_types::Pointer>(1);

                match self.term_test_variable(term_handle) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_bool(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_TERM_TEST_CONSTANT_INDEX => {
                let term_handle: Handle<tags::Term> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let result_ptr = args.nth::<semantic_types::Pointer>(1);

                match self.term_test_constant(term_handle) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_bool(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_TERM_TEST_APPLICATION_INDEX => {
                let term_handle: Handle<tags::Term> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let result_ptr = args.nth::<semantic_types::Pointer>(1);

                match self.term_test_application(term_handle) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_bool(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_TERM_TEST_LAMBDA_INDEX => {
                let term_handle: Handle<tags::Term> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let result_ptr = args.nth::<semantic_types::Pointer>(1);

                match self.term_test_lambda(term_handle) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_bool(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_TERM_TEST_NEGATION_INDEX => {
                let term_handle: Handle<tags::Term> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let result_ptr = args.nth::<semantic_types::Pointer>(1);

                match self.term_test_negation(term_handle) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_bool(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_TERM_TEST_CONJUNCTION_INDEX => {
                let term_handle: Handle<tags::Term> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let result_ptr = args.nth::<semantic_types::Pointer>(1);

                match self.term_test_conjunction(term_handle) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_bool(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_TERM_TEST_DISJUNCTION_INDEX => {
                let term_handle: Handle<tags::Term> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let result_ptr = args.nth::<semantic_types::Pointer>(1);

                match self.term_test_disjunction(term_handle) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_bool(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_TERM_TEST_IMPLICATION_INDEX => {
                let term_handle: Handle<tags::Term> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let result_ptr = args.nth::<semantic_types::Pointer>(1);

                match self.term_test_implication(term_handle) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_bool(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_TERM_TEST_EQUALITY_INDEX => {
                let term_handle: Handle<tags::Term> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let result_ptr = args.nth::<semantic_types::Pointer>(1);

                match self.term_test_equality(term_handle) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_bool(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_TERM_TEST_FORALL_INDEX => {
                let term_handle: Handle<tags::Term> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let result_ptr = args.nth::<semantic_types::Pointer>(1);

                match self.term_test_forall(term_handle) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_bool(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_TERM_TEST_EXISTS_INDEX => {
                let term_handle: Handle<tags::Term> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let result_ptr = args.nth::<semantic_types::Pointer>(1);

                match self.term_test_exists(term_handle) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_bool(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_TERM_FREE_VARIABLES_INDEX => {
                let term_handle: Handle<tags::Term> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let result_name_base_ptr =
                    args.nth::<semantic_types::Pointer>(1);
                let result_name_len_ptr =
                    args.nth::<semantic_types::Pointer>(2);
                let result_type_base_ptr =
                    args.nth::<semantic_types::Pointer>(1);
                let result_type_len_ptr =
                    args.nth::<semantic_types::Pointer>(2);

                match self.term_free_variables(term_handle) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        let (names, types): (
                            Vec<Name>,
                            Vec<Handle<tags::Type>>,
                        ) = result.iter().cloned().unzip();

                        self.write_u64(
                            result_name_len_ptr,
                            names.len() as u64,
                        )?;
                        self.write_u64s(result_name_base_ptr, names)?;
                        self.write_u64(
                            result_type_len_ptr,
                            types.len() as u64,
                        )?;
                        self.write_handles(result_type_base_ptr, types)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_TERM_SUBSTITUTE_INDEX => {
                let term_handle: Handle<tags::Term> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let dom_ptr = args.nth::<semantic_types::Pointer>(1);
                let dom_len = args.nth::<semantic_types::Size>(2);
                let type_ptr = args.nth::<semantic_types::Pointer>(3);
                let type_len = args.nth::<semantic_types::Size>(4);
                let rng_ptr = args.nth::<semantic_types::Pointer>(5);
                let rng_len = args.nth::<semantic_types::Size>(6);
                let result_ptr = args.nth::<semantic_types::Pointer>(7);

                let domains = self.read_u64s(dom_ptr, dom_len as usize)?;
                let types = self.read_u64s(type_ptr, type_len as usize)?;
                let ranges = self.read_handles(rng_ptr, rng_len as usize)?;

                let substitution = domains
                    .iter()
                    .zip(types)
                    .zip(ranges)
                    .map(|((d, t), r)| ((*d, Handle::from(t as usize)), r))
                    .collect();

                match self.term_substitute(term_handle, substitution) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_handle(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_TERM_TYPE_VARIABLES_INDEX => {
                let term_handle: Handle<tags::Term> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let result_base_ptr = args.nth::<semantic_types::Pointer>(1);
                let result_len_ptr = args.nth::<semantic_types::Pointer>(2);

                match self.term_type_variables(term_handle) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_u64(result_len_ptr, result.len() as u64)?;
                        self.write_u64s(result_base_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_TERM_TYPE_SUBSTITUTE_INDEX => {
                let term_handle: Handle<tags::Term> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );

                let dom_ptr = args.nth::<semantic_types::Pointer>(1);
                let dom_len = args.nth::<semantic_types::Size>(2);
                let rng_ptr = args.nth::<semantic_types::Pointer>(3);
                let rng_len = args.nth::<semantic_types::Size>(4);
                let result_ptr = args.nth::<semantic_types::Pointer>(5);

                let domains = self.read_u64s(dom_ptr, dom_len as usize)?;
                let ranges = self.read_handles(rng_ptr, rng_len as usize)?;

                let subst =
                    domains.iter().zip(ranges).map(|(d, r)| (*d, r)).collect();

                match self.term_type_substitute(term_handle, subst) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_handle(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_TERM_TYPE_INFER_INDEX => {
                let term_handle: Handle<tags::Term> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let result_ptr = args.nth::<semantic_types::Pointer>(1);

                match self.term_type_infer(term_handle) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_handle(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_TERM_TYPE_IS_PROPOSITION_INDEX => {
                let term_handle: Handle<tags::Term> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let result_ptr = args.nth::<semantic_types::Pointer>(1);

                match self.term_type_is_proposition(term_handle) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_bool(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_THEOREM_IS_REGISTERED_INDEX => {
                let theorem_handle: Handle<tags::Theorem> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let result = self.theorem_is_registered(theorem_handle);

                Ok(Some(RuntimeValue::I32(result.into())))
            }
            ABI_THEOREM_SPLIT_CONCLUSION_INDEX => {
                let theorem_handle: Handle<tags::Theorem> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let result_ptr = args.nth::<semantic_types::Pointer>(1);

                match self.theorem_split_conclusion(theorem_handle) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_handle(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_THEOREM_SPLIT_HYPOTHESES_INDEX => {
                let theorem_handle: Handle<tags::Theorem> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let result_base_ptr = args.nth::<semantic_types::Pointer>(1);
                let result_len_ptr = args.nth::<semantic_types::Pointer>(2);

                match self.theorem_split_premisses(theorem_handle) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_u64(result_len_ptr, result.len() as u64)?;
                        self.write_handles(result_base_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_THEOREM_REGISTER_ASSUMPTION_INDEX => {
                let term_handle: Handle<tags::Term> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let result_ptr = args.nth::<semantic_types::Pointer>(1);

                match self.theorem_register_assumption(term_handle) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_handle(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_THEOREM_REGISTER_REFLEXIVITY_INDEX => {
                let term_handle: Handle<tags::Term> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let result_ptr = args.nth::<semantic_types::Pointer>(1);

                match self.theorem_register_reflexivity(term_handle) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_handle(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_THEOREM_REGISTER_SYMMETRY_INDEX => {
                let theorem_handle: Handle<tags::Theorem> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let result_ptr = args.nth::<semantic_types::Pointer>(1);

                match self.theorem_register_symmetry(theorem_handle) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_handle(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_THEOREM_REGISTER_TRANSITIVITY_INDEX => {
                let left_handle: Handle<tags::Theorem> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let right_handle: Handle<tags::Theorem> = Handle::from(
                    args.nth::<semantic_types::Handle>(1) as usize,
                );
                let result_ptr = args.nth::<semantic_types::Pointer>(2);

                match self
                    .theorem_register_transitivity(left_handle, right_handle)
                {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_handle(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_THEOREM_REGISTER_BETA_INDEX => {
                let term_handle: Handle<tags::Term> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let result_ptr = args.nth::<semantic_types::Pointer>(1);

                match self.theorem_register_beta(term_handle) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_handle(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_THEOREM_REGISTER_ETA_INDEX => {
                let term_handle: Handle<tags::Term> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let result_ptr = args.nth::<semantic_types::Pointer>(1);

                match self.theorem_register_eta(term_handle) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_handle(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_THEOREM_REGISTER_APPLICATION_INDEX => {
                let left_handle: Handle<tags::Theorem> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let right_handle: Handle<tags::Theorem> = Handle::from(
                    args.nth::<semantic_types::Handle>(1) as usize,
                );
                let result_ptr = args.nth::<semantic_types::Pointer>(2);

                match self
                    .theorem_register_application(left_handle, right_handle)
                {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_handle(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_THEOREM_REGISTER_LAMBDA_INDEX => {
                let name: Name = args.nth::<semantic_types::Name>(0);
                let type_handle: Handle<tags::Type> = Handle::from(
                    args.nth::<semantic_types::Handle>(1) as usize,
                );
                let theorem_handle: Handle<tags::Theorem> = Handle::from(
                    args.nth::<semantic_types::Handle>(2) as usize,
                );
                let result_ptr = args.nth::<semantic_types::Pointer>(3);

                match self.theorem_register_lambda(
                    name,
                    type_handle,
                    theorem_handle,
                ) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_handle(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_THEOREM_REGISTER_SUBSTITUTE_INDEX => {
                let theorem_handle: Handle<tags::Theorem> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let dom_ptr = args.nth::<semantic_types::Pointer>(1);
                let dom_len = args.nth::<semantic_types::Size>(2);
                let tau_ptr = args.nth::<semantic_types::Pointer>(3);
                let tau_len = args.nth::<semantic_types::Pointer>(4);
                let rng_ptr = args.nth::<semantic_types::Pointer>(5);
                let rng_len = args.nth::<semantic_types::Size>(6);
                let result_ptr = args.nth::<semantic_types::Pointer>(7);

                let domains = self.read_u64s(dom_ptr, dom_len as usize)?;
                let _types = self.read_u64s(tau_ptr, tau_len as usize)?;
                let ranges = self.read_handles(rng_ptr, rng_len as usize)?;

                let subst = domains
                    .iter()
                    .zip(_types)
                    .zip(ranges)
                    .map(|((d, t), r)| ((*d, Handle::from(t as usize)), r))
                    .collect();

                match self.theorem_register_substitute(theorem_handle, subst) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_handle(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_THEOREM_REGISTER_TYPE_SUBSTITUTE_INDEX => {
                let theorem_handle: Handle<tags::Theorem> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let dom_ptr = args.nth::<semantic_types::Pointer>(1);
                let dom_len = args.nth::<semantic_types::Size>(2);
                let rng_ptr = args.nth::<semantic_types::Pointer>(3);
                let rng_len = args.nth::<semantic_types::Size>(4);
                let result_ptr = args.nth::<semantic_types::Pointer>(5);

                let domains = self.read_u64s(dom_ptr, dom_len as usize)?;
                let ranges = self.read_handles(rng_ptr, rng_len as usize)?;

                let subst =
                    domains.iter().zip(ranges).map(|(d, r)| (*d, r)).collect();

                match self
                    .theorem_register_type_substitute(theorem_handle, subst)
                {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_handle(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_THEOREM_REGISTER_TRUTH_INTRODUCTION_INDEX => {
                let result_ptr = args.nth::<semantic_types::Pointer>(0);

                match self.theorem_register_truth_introduction() {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_handle(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_THEOREM_REGISTER_FALSITY_ELIMINATION_INDEX => {
                let theorem_handle: Handle<tags::Theorem> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let term_handle: Handle<tags::Term> = Handle::from(
                    args.nth::<semantic_types::Handle>(1) as usize,
                );
                let result_ptr = args.nth::<semantic_types::Pointer>(2);

                match self.theorem_register_falsity_elimination(
                    theorem_handle,
                    term_handle,
                ) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_handle(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_THEOREM_REGISTER_CONJUNCTION_INTRODUCTION_INDEX => {
                let left_handle: Handle<tags::Theorem> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let right_handle: Handle<tags::Theorem> = Handle::from(
                    args.nth::<semantic_types::Handle>(1) as usize,
                );
                let result_ptr = args.nth::<semantic_types::Pointer>(2);

                match self.theorem_register_conjunction_introduction(
                    left_handle,
                    right_handle,
                ) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_handle(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_THEOREM_REGISTER_CONJUNCTION_LEFT_ELIMINATION_INDEX => {
                let theorem_handle: Handle<tags::Theorem> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let result_ptr = args.nth::<semantic_types::Pointer>(1);

                match self.theorem_register_conjunction_left_elimination(
                    theorem_handle,
                ) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_handle(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_THEOREM_REGISTER_CONJUNCTION_RIGHT_ELIMINATION_INDEX => {
                let theorem_handle: Handle<tags::Theorem> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let result_ptr = args.nth::<semantic_types::Pointer>(1);

                match self.theorem_register_conjunction_right_elimination(
                    theorem_handle,
                ) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_handle(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_THEOREM_REGISTER_DISJUNCTION_ELIMINATION_INDEX => {
                let left_handle: Handle<tags::Theorem> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let mid_handle: Handle<tags::Theorem> = Handle::from(
                    args.nth::<semantic_types::Handle>(1) as usize,
                );
                let right_handle: Handle<tags::Theorem> = Handle::from(
                    args.nth::<semantic_types::Handle>(2) as usize,
                );
                let result_ptr = args.nth::<semantic_types::Pointer>(3);

                match self.theorem_register_disjunction_elimination(
                    left_handle,
                    mid_handle,
                    right_handle,
                ) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_handle(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_THEOREM_REGISTER_DISJUNCTION_LEFT_INTRODUCTION_INDEX => {
                let theorem_handle: Handle<tags::Theorem> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let term_handle: Handle<tags::Term> = Handle::from(
                    args.nth::<semantic_types::Handle>(1) as usize,
                );
                let result_ptr = args.nth::<semantic_types::Pointer>(2);

                match self.theorem_register_disjunction_left_introduction(
                    theorem_handle,
                    term_handle,
                ) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_handle(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_THEOREM_REGISTER_DISJUNCTION_RIGHT_INTRODUCTION_INDEX => {
                let theorem_handle: Handle<tags::Theorem> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let term_handle: Handle<tags::Term> = Handle::from(
                    args.nth::<semantic_types::Handle>(1) as usize,
                );
                let result_ptr = args.nth::<semantic_types::Pointer>(2);

                match self.theorem_register_disjunction_right_introduction(
                    theorem_handle,
                    term_handle,
                ) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_handle(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_THEOREM_REGISTER_IMPLICATION_INTRODUCTION_INDEX => {
                let theorem_handle: Handle<tags::Theorem> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let term_handle: Handle<tags::Term> = Handle::from(
                    args.nth::<semantic_types::Handle>(1) as usize,
                );
                let result_ptr = args.nth::<semantic_types::Pointer>(2);

                match self.theorem_register_implication_introduction(
                    theorem_handle,
                    term_handle,
                ) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_handle(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_THEOREM_REGISTER_IMPLICATION_ELIMINATION_INDEX => {
                let left_handle: Handle<tags::Theorem> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let right_handle: Handle<tags::Theorem> = Handle::from(
                    args.nth::<semantic_types::Handle>(1) as usize,
                );
                let result_ptr = args.nth::<semantic_types::Pointer>(2);

                match self.theorem_register_implication_elimination(
                    left_handle,
                    right_handle,
                ) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_handle(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_THEOREM_REGISTER_IFF_INTRODUCTION_INDEX => {
                let left_handle: Handle<tags::Theorem> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let right_handle: Handle<tags::Theorem> = Handle::from(
                    args.nth::<semantic_types::Handle>(1) as usize,
                );
                let result_ptr = args.nth::<semantic_types::Pointer>(2);

                match self.theorem_register_iff_introduction(
                    left_handle,
                    right_handle,
                ) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_handle(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_THEOREM_REGISTER_IFF_LEFT_ELIMINATION_INDEX => {
                let theorem_handle: Handle<tags::Theorem> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let result_ptr = args.nth::<semantic_types::Pointer>(1);

                match self.theorem_register_iff_left_elimination(theorem_handle)
                {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_handle(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_THEOREM_REGISTER_NEGATION_INTRODUCTION_INDEX => {
                let theorem_handle: Handle<tags::Theorem> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let term_handle: Handle<tags::Term> = Handle::from(
                    args.nth::<semantic_types::Handle>(1) as usize,
                );
                let result_ptr = args.nth::<semantic_types::Pointer>(2);

                match self.theorem_register_negation_introduction(
                    theorem_handle,
                    term_handle,
                ) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_handle(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_THEOREM_REGISTER_NEGATION_ELIMINATION_INDEX => {
                let left_handle: Handle<tags::Theorem> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let right_handle: Handle<tags::Theorem> = Handle::from(
                    args.nth::<semantic_types::Handle>(1) as usize,
                );
                let result_ptr = args.nth::<semantic_types::Pointer>(2);

                match self.theorem_register_negation_elimination(
                    left_handle,
                    right_handle,
                ) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_handle(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_THEOREM_REGISTER_FORALL_INTRODUCTION_INDEX => {
                let theorem_handle: Handle<tags::Theorem> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let name: Name = args.nth::<semantic_types::Name>(1);
                let _type = Handle::from(
                    args.nth::<semantic_types::Handle>(2) as usize
                );
                let result_ptr = args.nth::<semantic_types::Pointer>(3);

                match self.theorem_register_forall_introduction(
                    theorem_handle,
                    name,
                    _type,
                ) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_handle(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_THEOREM_REGISTER_FORALL_ELIMINATION_INDEX => {
                let theorem_handle: Handle<tags::Theorem> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let term_handle: Handle<tags::Term> = Handle::from(
                    args.nth::<semantic_types::Handle>(1) as usize,
                );
                let result_ptr = args.nth::<semantic_types::Pointer>(2);

                match self.theorem_register_forall_elimination(
                    theorem_handle,
                    term_handle,
                ) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_handle(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_THEOREM_REGISTER_EXISTS_INTRODUCTION_INDEX => {
                let theorem_handle: Handle<tags::Theorem> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let term_handle: Handle<tags::Term> = Handle::from(
                    args.nth::<semantic_types::Handle>(1) as usize,
                );
                let result_ptr = args.nth::<semantic_types::Pointer>(2);

                match self.theorem_register_exists_introduction(
                    theorem_handle,
                    term_handle,
                ) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_handle(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            ABI_THEOREM_REGISTER_EXISTS_ELIMINATION_INDEX => {
                let left_handle: Handle<tags::Theorem> = Handle::from(
                    args.nth::<semantic_types::Handle>(0) as usize,
                );
                let right_handle: Handle<tags::Theorem> = Handle::from(
                    args.nth::<semantic_types::Handle>(1) as usize,
                );
                let result_ptr = args.nth::<semantic_types::Pointer>(2);

                match self.theorem_register_exists_elimination(
                    left_handle,
                    right_handle,
                ) {
                    Err(e) => Ok(Some(RuntimeValue::I32(e as i32))),
                    Ok(result) => {
                        self.write_handle(result_ptr, result)?;

                        Ok(Some(RuntimeValue::I32(
                            KernelErrorCode::Success.into(),
                        )))
                    }
                }
            }
            _otherwise => {
                Err(runtime_trap::host_trap(RuntimeTrap::NoSuchFunction))
            }
        }
    }
}

/// Maps an ABI host-call to its associated host-call number.  Also checks that
/// the function's signature is as expected, otherwise produces a runtime error
/// that is reported back to the WASM program.
impl ModuleImportResolver for WasmiRuntimeState {
    fn resolve_func(
        &self,
        field_name: &str,
        signature: &Signature,
    ) -> Result<FuncRef, WasmiError> {
        match field_name {
            ABI_TYPE_FORMER_RESOLVE_NAME => {
                if !type_checking::check_type_former_resolve_signature(
                    signature,
                ) {
                    error!("Signature check failed when checking __type_former_resolve.  Signature: {:?}.", signature);

                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_TYPE_FORMER_RESOLVE_INDEX,
                ))
            }
            ABI_TYPE_FORMER_REGISTER_NAME => {
                if !type_checking::check_type_former_register_signature(
                    signature,
                ) {
                    error!("Signature check failed when checking __type_former_register.  Signature: {:?}.", signature);

                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_TYPE_FORMER_REGISTER_INDEX,
                ))
            }
            ABI_TYPE_FORMER_IS_REGISTERED_NAME => {
                if !type_checking::check_type_former_is_registered_signature(
                    signature,
                ) {
                    error!("Signature check failed when checking __type_former_is_registered.  Signature: {:?}.", signature);

                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_TYPE_FORMER_IS_REGISTERED_INDEX,
                ))
            }
            ABI_TYPE_IS_REGISTERED_NAME => {
                if !type_checking::check_type_is_registered_signature(signature)
                {
                    error!("Signature check failed when checking __type_is_registered.  Signature: {:?}.", signature);

                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_TYPE_IS_REGISTERED_INDEX,
                ))
            }
            ABI_TYPE_REGISTER_VARIABLE_NAME => {
                if !type_checking::check_type_register_variable_signature(
                    signature,
                ) {
                    error!("Signature check failed when checking __type_register_variable.  Signature: {:?}.", signature);

                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_TYPE_REGISTER_VARIABLE_INDEX,
                ))
            }
            ABI_TYPE_REGISTER_COMBINATION_NAME => {
                if !type_checking::check_type_register_combination_signature(
                    signature,
                ) {
                    error!("Signature check failed when checking __type_register_combination.  Signature: {:?}.", signature);

                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_TYPE_REGISTER_COMBINATION_INDEX,
                ))
            }
            ABI_TYPE_REGISTER_FUNCTION_NAME => {
                if !type_checking::check_type_register_function_signature(
                    signature,
                ) {
                    error!("Signature check failed when checking __type_register_function.  Signature: {:?}.", signature);

                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_TYPE_REGISTER_FUNCTION_INDEX,
                ))
            }
            ABI_TYPE_SPLIT_VARIABLE_NAME => {
                if !type_checking::check_type_split_variable_signature(
                    signature,
                ) {
                    error!("Signature check failed when checking __type_split_variable.  Signature: {:?}.", signature);

                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_TYPE_SPLIT_VARIABLE_INDEX,
                ))
            }
            ABI_TYPE_SPLIT_COMBINATION_NAME => {
                if !type_checking::check_type_split_combination_signature(
                    signature,
                ) {
                    error!("Signature check failed when checking __type_split_combination.  Signature: {:?}.", signature);

                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_TYPE_SPLIT_COMBINATION_INDEX,
                ))
            }
            ABI_TYPE_SPLIT_FUNCTION_NAME => {
                if !type_checking::check_type_split_function_signature(
                    signature,
                ) {
                    error!("Signature check failed when checking __type_split_function.  Signature: {:?}.", signature);

                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_TYPE_SPLIT_FUNCTION_INDEX,
                ))
            }
            ABI_TYPE_TEST_VARIABLE_NAME => {
                if !type_checking::check_type_test_variable_signature(signature)
                {
                    error!("Signature check failed when checking __type_test_variable.  Signature: {:?}.", signature);

                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_TYPE_TEST_VARIABLE_INDEX,
                ))
            }
            ABI_TYPE_TEST_COMBINATION_NAME => {
                if !type_checking::check_type_test_combination_signature(
                    signature,
                ) {
                    error!("Signature check failed when checking __type_test_combination.  Signature: {:?}.", signature);

                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_TYPE_TEST_COMBINATION_INDEX,
                ))
            }
            ABI_TYPE_TEST_FUNCTION_NAME => {
                if !type_checking::check_type_test_function_signature(signature)
                {
                    error!("Signature check failed when checking __type_test_function.  Signature: {:?}.", signature);

                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_TYPE_TEST_FUNCTION_INDEX,
                ))
            }
            ABI_TYPE_SIZE_NAME => {
                if !type_checking::check_type_size_signature(signature) {
                    error!("Signature check failed when checking __type_size.  Signature: {:?}.", signature);

                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_TYPE_SIZE_INDEX,
                ))
            }
            ABI_TYPE_VARIABLES_NAME => {
                if !type_checking::check_type_variables_signature(signature) {
                    error!("Signature check failed when checking __type_variables.  Signature: {:?}.", signature);

                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_TYPE_VARIABLES_INDEX,
                ))
            }
            ABI_TYPE_SUBSTITUTE_NAME => {
                if !type_checking::check_type_substitute_signature(signature) {
                    error!("Signature check failed when checking __type_substitute.  Signature: {:?}.", signature);

                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_TYPE_SUBSTITUTE_INDEX,
                ))
            }
            ABI_CONSTANT_RESOLVE_NAME => {
                if !type_checking::check_constant_resolve_signature(signature) {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_CONSTANT_RESOLVE_INDEX,
                ))
            }
            ABI_CONSTANT_IS_REGISTERED_NAME => {
                if !type_checking::check_constant_is_registered_signature(
                    signature,
                ) {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_CONSTANT_IS_REGISTERED_INDEX,
                ))
            }
            ABI_CONSTANT_REGISTER_NAME => {
                if !type_checking::check_constant_register_signature(signature)
                {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_CONSTANT_REGISTER_INDEX,
                ))
            }
            ABI_TERM_IS_REGISTERED_NAME => {
                if !type_checking::check_term_is_registered_signature(signature)
                {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_TERM_IS_REGISTERED_INDEX,
                ))
            }
            ABI_TERM_REGISTER_VARIABLE_NAME => {
                if !type_checking::check_term_register_variable_signature(
                    signature,
                ) {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_TERM_REGISTER_VARIABLE_INDEX,
                ))
            }
            ABI_TERM_REGISTER_CONSTANT_NAME => {
                if !type_checking::check_term_register_constant_signature(
                    signature,
                ) {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_TERM_REGISTER_CONSTANT_INDEX,
                ))
            }
            ABI_TERM_REGISTER_APPLICATION_NAME => {
                if !type_checking::check_term_register_application_signature(
                    signature,
                ) {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_TERM_REGISTER_APPLICATION_INDEX,
                ))
            }
            ABI_TERM_REGISTER_LAMBDA_NAME => {
                if !type_checking::check_term_register_lambda_signature(
                    signature,
                ) {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_TERM_REGISTER_LAMBDA_INDEX,
                ))
            }
            ABI_TERM_REGISTER_NEGATION_NAME => {
                if !type_checking::check_term_register_negation_signature(
                    signature,
                ) {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_TERM_REGISTER_NEGATION_INDEX,
                ))
            }
            ABI_TERM_REGISTER_CONJUNCTION_NAME => {
                if !type_checking::check_term_register_conjunction_signature(
                    signature,
                ) {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_TERM_REGISTER_CONJUNCTION_INDEX,
                ))
            }
            ABI_TERM_REGISTER_DISJUNCTION_NAME => {
                if !type_checking::check_term_register_disjunction_signature(
                    signature,
                ) {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_TERM_REGISTER_DISJUNCTION_INDEX,
                ))
            }
            ABI_TERM_REGISTER_IMPLICATION_NAME => {
                if !type_checking::check_term_register_implication_signature(
                    signature,
                ) {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_TERM_REGISTER_IMPLICATION_INDEX,
                ))
            }
            ABI_TERM_REGISTER_EQUALITY_NAME => {
                if !type_checking::check_term_register_equality_signature(
                    signature,
                ) {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_TERM_REGISTER_EQUALITY_INDEX,
                ))
            }
            ABI_TERM_REGISTER_FORALL_NAME => {
                if !type_checking::check_term_register_forall_signature(
                    signature,
                ) {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_TERM_REGISTER_FORALL_INDEX,
                ))
            }
            ABI_TERM_REGISTER_EXISTS_NAME => {
                if !type_checking::check_term_register_exists_signature(
                    signature,
                ) {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_TERM_REGISTER_EXISTS_INDEX,
                ))
            }
            ABI_TERM_SPLIT_VARIABLE_NAME => {
                if !type_checking::check_term_split_variable_signature(
                    signature,
                ) {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_TERM_SPLIT_VARIABLE_INDEX,
                ))
            }
            ABI_TERM_SPLIT_CONSTANT_NAME => {
                if !type_checking::check_term_split_constant_signature(
                    signature,
                ) {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_TERM_SPLIT_CONSTANT_INDEX,
                ))
            }
            ABI_TERM_SPLIT_APPLICATION_NAME => {
                if !type_checking::check_term_split_application_signature(
                    signature,
                ) {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_TERM_SPLIT_APPLICATION_INDEX,
                ))
            }
            ABI_TERM_SPLIT_LAMBDA_NAME => {
                if !type_checking::check_term_split_lambda_signature(signature)
                {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_TERM_SPLIT_LAMBDA_INDEX,
                ))
            }
            ABI_TERM_SPLIT_NEGATION_NAME => {
                if !type_checking::check_term_split_negation_signature(
                    signature,
                ) {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_TERM_SPLIT_NEGATION_INDEX,
                ))
            }
            ABI_TERM_SPLIT_CONJUNCTION_NAME => {
                if !type_checking::check_term_split_conjunction_signature(
                    signature,
                ) {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_TERM_SPLIT_CONJUNCTION_INDEX,
                ))
            }
            ABI_TERM_SPLIT_DISJUNCTION_NAME => {
                if !type_checking::check_term_split_disjunction_signature(
                    signature,
                ) {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_TERM_SPLIT_DISJUNCTION_INDEX,
                ))
            }
            ABI_TERM_SPLIT_IMPLICATION_NAME => {
                if !type_checking::check_term_split_implication_signature(
                    signature,
                ) {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_TERM_SPLIT_IMPLICATION_INDEX,
                ))
            }
            ABI_TERM_SPLIT_EQUALITY_NAME => {
                if !type_checking::check_term_split_equality_signature(
                    signature,
                ) {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_TERM_SPLIT_EQUALITY_INDEX,
                ))
            }
            ABI_TERM_SPLIT_FORALL_NAME => {
                if !type_checking::check_term_split_forall_signature(signature)
                {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_TERM_SPLIT_FORALL_INDEX,
                ))
            }
            ABI_TERM_SPLIT_EXISTS_NAME => {
                if !type_checking::check_term_split_exists_signature(signature)
                {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_TERM_SPLIT_EXISTS_INDEX,
                ))
            }
            ABI_TERM_TEST_VARIABLE_NAME => {
                if !type_checking::check_term_test_variable_signature(signature)
                {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_TERM_TEST_VARIABLE_INDEX,
                ))
            }
            ABI_TERM_TEST_CONSTANT_NAME => {
                if !type_checking::check_term_test_constant_signature(signature)
                {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_TERM_TEST_CONSTANT_INDEX,
                ))
            }
            ABI_TERM_TEST_APPLICATION_NAME => {
                if !type_checking::check_term_test_application_signature(
                    signature,
                ) {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_TERM_TEST_APPLICATION_INDEX,
                ))
            }
            ABI_TERM_TEST_LAMBDA_NAME => {
                if !type_checking::check_term_test_lambda_signature(signature) {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_TERM_TEST_LAMBDA_INDEX,
                ))
            }
            ABI_TERM_TEST_NEGATION_NAME => {
                if !type_checking::check_term_test_negation_signature(signature)
                {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_TERM_TEST_NEGATION_INDEX,
                ))
            }
            ABI_TERM_TEST_CONJUNCTION_NAME => {
                if !type_checking::check_term_test_conjunction_signature(
                    signature,
                ) {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_TERM_TEST_CONJUNCTION_INDEX,
                ))
            }
            ABI_TERM_TEST_DISJUNCTION_NAME => {
                if !type_checking::check_term_test_disjunction_signature(
                    signature,
                ) {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_TERM_TEST_DISJUNCTION_INDEX,
                ))
            }
            ABI_TERM_TEST_IMPLICATION_NAME => {
                if !type_checking::check_term_test_implication_signature(
                    signature,
                ) {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_TERM_TEST_IMPLICATION_INDEX,
                ))
            }
            ABI_TERM_TEST_EQUALITY_NAME => {
                if !type_checking::check_term_test_equality_signature(signature)
                {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_TERM_TEST_EQUALITY_INDEX,
                ))
            }
            ABI_TERM_TEST_FORALL_NAME => {
                if !type_checking::check_term_test_forall_signature(signature) {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_TERM_TEST_FORALL_INDEX,
                ))
            }
            ABI_TERM_TEST_EXISTS_NAME => {
                if !type_checking::check_term_test_exists_signature(signature) {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_TERM_TEST_EXISTS_INDEX,
                ))
            }
            ABI_TERM_FREE_VARIABLES_NAME => {
                if !type_checking::check_term_fv_signature(signature) {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_TERM_FREE_VARIABLES_INDEX,
                ))
            }
            ABI_TERM_SUBSTITUTE_NAME => {
                if !type_checking::check_term_substitute_signature(signature) {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_TERM_SUBSTITUTE_INDEX,
                ))
            }
            ABI_TERM_TYPE_VARIABLES_NAME => {
                if !type_checking::check_term_type_variables_signature(
                    signature,
                ) {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_TERM_TYPE_VARIABLES_INDEX,
                ))
            }
            ABI_TERM_TYPE_SUBSTITUTE_NAME => {
                if !type_checking::check_term_type_substitute_signature(
                    signature,
                ) {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_TERM_TYPE_SUBSTITUTE_INDEX,
                ))
            }
            ABI_TERM_TYPE_INFER_NAME => {
                if !type_checking::check_term_type_infer_signature(signature) {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_TERM_TYPE_INFER_INDEX,
                ))
            }
            ABI_TERM_TYPE_IS_PROPOSITION_NAME => {
                if !type_checking::check_term_type_is_proposition_signature(
                    signature,
                ) {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_TERM_TYPE_IS_PROPOSITION_INDEX,
                ))
            }
            ABI_THEOREM_IS_REGISTERED_NAME => {
                if !type_checking::check_theorem_is_registered_signature(
                    signature,
                ) {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_THEOREM_IS_REGISTERED_INDEX,
                ))
            }
            ABI_THEOREM_REGISTER_ASSUMPTION_NAME => {
                if !type_checking::check_theorem_register_assumption_signature(
                    signature,
                ) {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_THEOREM_REGISTER_ASSUMPTION_INDEX,
                ))
            }
            ABI_THEOREM_REGISTER_REFLEXIVITY_NAME => {
                if !type_checking::check_theorem_register_reflexivity_signature(
                    signature,
                ) {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_THEOREM_REGISTER_REFLEXIVITY_INDEX,
                ))
            }
            ABI_THEOREM_REGISTER_SYMMETRY_NAME => {
                if !type_checking::check_theorem_register_symmetry_signature(
                    signature,
                ) {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_THEOREM_REGISTER_SYMMETRY_INDEX,
                ))
            }
            ABI_THEOREM_REGISTER_TRANSITIVITY_NAME => {
                if !type_checking::check_theorem_register_transitivity_signature(
                    signature,
                ) {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_THEOREM_REGISTER_TRANSITIVITY_INDEX,
                ))
            }
            ABI_THEOREM_REGISTER_APPLICATION_NAME => {
                if !type_checking::check_theorem_register_application_signature(
                    signature,
                ) {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_THEOREM_REGISTER_APPLICATION_INDEX,
                ))
            }
            ABI_THEOREM_REGISTER_LAMBDA_NAME => {
                if !type_checking::check_theorem_register_lambda_signature(
                    signature,
                ) {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_THEOREM_REGISTER_LAMBDA_INDEX,
                ))
            }
            ABI_THEOREM_REGISTER_BETA_NAME => {
                if !type_checking::check_theorem_register_beta_signature(
                    signature,
                ) {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_THEOREM_REGISTER_BETA_INDEX,
                ))
            }
            ABI_THEOREM_REGISTER_ETA_NAME => {
                if !type_checking::check_theorem_register_eta_signature(
                    signature,
                ) {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_THEOREM_REGISTER_ETA_INDEX,
                ))
            }
            ABI_THEOREM_REGISTER_SUBSTITUTE_NAME => {
                if !type_checking::check_theorem_register_substitute_signature(
                    signature,
                ) {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_THEOREM_REGISTER_SUBSTITUTE_INDEX,
                ))
            }
            ABI_THEOREM_REGISTER_TYPE_SUBSTITUTE_NAME => {
                if !type_checking::check_theorem_register_type_substitute_signature(
                    signature,
                ) {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_THEOREM_REGISTER_TYPE_SUBSTITUTE_INDEX,
                ))
            }
            ABI_THEOREM_REGISTER_TRUTH_INTRODUCTION_NAME => {
                if !type_checking::check_theorem_register_truth_introduction_signature(
                    signature,
                ) {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_THEOREM_REGISTER_TRUTH_INTRODUCTION_INDEX,
                ))
            }
            ABI_THEOREM_REGISTER_FALSITY_ELIMINATION_NAME => {
                if !type_checking::check_theorem_register_falsity_elimination_signature(
                    signature,
                ) {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_THEOREM_REGISTER_FALSITY_ELIMINATION_INDEX,
                ))
            }
            ABI_THEOREM_REGISTER_CONJUNCTION_INTRODUCTION_NAME => {
                if !type_checking::check_theorem_register_conjunction_introduction_signature(
                    signature,
                ) {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_THEOREM_REGISTER_CONJUNCTION_INTRODUCTION_INDEX,
                ))
            }
            ABI_THEOREM_REGISTER_CONJUNCTION_LEFT_ELIMINATION_NAME => {
                if !type_checking::check_theorem_register_conjunction_left_elimination_signature(signature) {
                    return Err(WasmiError::Trap(
                        runtime_trap::host_trap(RuntimeTrap::SignatureFailure)));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_THEOREM_REGISTER_CONJUNCTION_LEFT_ELIMINATION_INDEX,
                ))
            }
            ABI_THEOREM_REGISTER_CONJUNCTION_RIGHT_ELIMINATION_NAME => {
                if !type_checking::check_theorem_register_conjunction_right_elimination_signature(signature) {
                    return Err(WasmiError::Trap(
                        runtime_trap::host_trap(RuntimeTrap::SignatureFailure)));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_THEOREM_REGISTER_CONJUNCTION_RIGHT_ELIMINATION_INDEX,
                ))
            }
            ABI_THEOREM_REGISTER_DISJUNCTION_ELIMINATION_NAME => {
                if !type_checking::check_theorem_register_disjunction_elimination_signature(
                    signature,
                ) {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_THEOREM_REGISTER_DISJUNCTION_ELIMINATION_INDEX,
                ))
            }
            ABI_THEOREM_REGISTER_DISJUNCTION_LEFT_INTRODUCTION_NAME => {
                if !type_checking::check_theorem_register_disjunction_left_introduction_signature(signature) {
                    return Err(WasmiError::Trap(
                        runtime_trap::host_trap(RuntimeTrap::SignatureFailure)));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_THEOREM_REGISTER_DISJUNCTION_LEFT_INTRODUCTION_INDEX,
                ))
            }
            ABI_THEOREM_REGISTER_DISJUNCTION_RIGHT_INTRODUCTION_NAME => {
                if !type_checking::check_theorem_register_disjunction_right_introduction_signature(signature) {
                    return Err(WasmiError::Trap(
                        runtime_trap::host_trap(RuntimeTrap::SignatureFailure)));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_THEOREM_REGISTER_DISJUNCTION_RIGHT_INTRODUCTION_INDEX,
                ))
            }
            ABI_THEOREM_REGISTER_IMPLICATION_INTRODUCTION_NAME => {
                if !type_checking::check_theorem_register_implication_introduction_signature(
                    signature,
                ) {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_THEOREM_REGISTER_IMPLICATION_INTRODUCTION_INDEX,
                ))
            }
            ABI_THEOREM_REGISTER_IMPLICATION_ELIMINATION_NAME => {
                if !type_checking::check_theorem_register_implication_elimination_signature(
                    signature,
                ) {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_THEOREM_REGISTER_IMPLICATION_ELIMINATION_INDEX,
                ))
            }
            ABI_THEOREM_REGISTER_IFF_INTRODUCTION_NAME => {
                if !type_checking::check_theorem_register_iff_introduction_signature(signature)
                {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_THEOREM_REGISTER_IFF_INTRODUCTION_INDEX,
                ))
            }
            ABI_THEOREM_REGISTER_IFF_LEFT_ELIMINATION_NAME => {
                if !type_checking::check_theorem_register_iff_left_elimination_signature(
                    signature,
                ) {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_THEOREM_REGISTER_IFF_LEFT_ELIMINATION_INDEX,
                ))
            }
            ABI_THEOREM_REGISTER_NEGATION_INTRODUCTION_NAME => {
                if !type_checking::check_theorem_register_negation_introduction_signature(
                    signature,
                ) {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_THEOREM_REGISTER_NEGATION_INTRODUCTION_INDEX,
                ))
            }
            ABI_THEOREM_REGISTER_NEGATION_ELIMINATION_NAME => {
                if !type_checking::check_theorem_register_negation_elimination_signature(
                    signature,
                ) {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_THEOREM_REGISTER_NEGATION_ELIMINATION_INDEX,
                ))
            }
            ABI_THEOREM_REGISTER_FORALL_INTRODUCTION_NAME => {
                if !type_checking::check_theorem_register_forall_introduction_signature(
                    signature,
                ) {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_THEOREM_REGISTER_FORALL_INTRODUCTION_INDEX,
                ))
            }
            ABI_THEOREM_REGISTER_FORALL_ELIMINATION_NAME => {
                if !type_checking::check_theorem_register_forall_elimination_signature(
                    signature,
                ) {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_THEOREM_REGISTER_FORALL_ELIMINATION_INDEX,
                ))
            }
            ABI_THEOREM_REGISTER_EXISTS_ELIMINATION_NAME => {
                if !type_checking::check_theorem_register_exists_elimination_signature(
                    signature,
                ) {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_THEOREM_REGISTER_EXISTS_ELIMINATION_INDEX,
                ))
            }
            ABI_THEOREM_REGISTER_EXISTS_INTRODUCTION_NAME => {
                if !type_checking::check_theorem_register_exists_introduction_signature(
                    signature,
                ) {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_THEOREM_REGISTER_EXISTS_INTRODUCTION_INDEX,
                ))
            }
            ABI_THEOREM_SPLIT_CONCLUSION_NAME => {
                if !type_checking::check_theorem_split_conclusion_signature(
                    signature,
                ) {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_THEOREM_SPLIT_CONCLUSION_INDEX,
                ))
            }
            ABI_THEOREM_SPLIT_HYPOTHESES_NAME => {
                if !type_checking::check_theorem_split_hypotheses_signature(
                    signature,
                ) {
                    return Err(WasmiError::Trap(runtime_trap::host_trap(
                        RuntimeTrap::SignatureFailure,
                    )));
                }

                Ok(FuncInstance::alloc_host(
                    signature.clone(),
                    ABI_THEOREM_SPLIT_HYPOTHESES_INDEX,
                ))
            }
            _otherwise => {
                Err(runtime_trap::host_error(KernelErrorCode::NoSuchFunction))
            }
        }
    }
}
