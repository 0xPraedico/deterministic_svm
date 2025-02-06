use std::collections::HashMap;
use std::num::Saturating;

pub type Pubkey = [u8; 32];

#[derive(Default, Debug, PartialEq, Eq)]
pub struct ProgramTiming {
    pub accumulated_us: Saturating<u64>,
    pub accumulated_units: Saturating<u64>,
    pub count: Saturating<u32>,
    pub errored_txs_compute_consumed: Vec<u64>,
    // Sum of all units in `errored_txs_compute_consumed`
    pub total_errored_units: Saturating<u64>,
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct ExecuteDetailsTimings {
    pub serialize_us: Saturating<u64>,
    pub create_vm_us: Saturating<u64>,
    pub execute_us: Saturating<u64>,
    pub deserialize_us: Saturating<u64>,
    pub get_or_create_executor_us: Saturating<u64>,
    pub changed_account_count: Saturating<u64>,
    pub total_account_count: Saturating<u64>,
    pub create_executor_register_syscalls_us: Saturating<u64>,
    pub create_executor_load_elf_us: Saturating<u64>,
    pub create_executor_verify_code_us: Saturating<u64>,
    pub create_executor_jit_compile_us: Saturating<u64>,
    pub per_program_timings: HashMap<Pubkey, ProgramTiming>,
}
