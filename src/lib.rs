mod transaction_context;
mod program_cache_for_tx_batch;
mod log_collector;
mod compute_budget;
mod measure;
mod timings;
mod syscall_context;
mod environment_config;

pub use transaction_context::*;
pub use program_cache_for_tx_batch::*;
pub use log_collector::*;
pub use compute_budget::*;
pub use measure::*;
pub use timings::*;
pub use syscall_context::*;
pub use environment_config::*;

use std::{cell::RefCell, rc::Rc};
pub struct InvokeContext<'a> {
    /// Information about the currently executing transaction.
    pub transaction_context: &'a mut TransactionContext,
    /// The local program cache for the transaction batch.
    pub program_cache_for_tx_batch: &'a mut ProgramCacheForTxBatch,
    /// Runtime configurations used to provision the invocation environment.
    pub environment_config: EnvironmentConfig<'a>,
    /// The compute budget for the current invocation.
    compute_budget: ComputeBudget,
    /// Instruction compute meter, for tracking compute units consumed against
    /// the designated compute budget during program execution.
    compute_meter: RefCell<u64>,
    log_collector: Option<Rc<RefCell<LogCollector>>>,
    /// Latest measurement not yet accumulated in [ExecuteDetailsTimings::execute_us]
    pub execute_time: Option<Measure>,
    pub timings: ExecuteDetailsTimings,
    pub syscall_context: Vec<Option<SyscallContext>>,
    traces: Vec<Vec<[u64; 12]>>,
}
