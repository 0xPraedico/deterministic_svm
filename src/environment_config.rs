use std::{collections::{HashMap, HashSet}, hash::RandomState, sync::Arc};


pub type Pubkey = [u8; 32];
/// Size of a hash in bytes.
pub const HASH_BYTES: usize = 32;
#[derive(Clone, Copy, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Hash(pub(crate) [u8; HASH_BYTES]);

pub type SlotHash = (u64, Hash);

pub type StakeHistoryInner = StakeHistory;
#[derive(PartialEq, Eq, Debug, Default)]
pub struct SlotHashes(Vec<SlotHash>);



#[derive(Default, Clone, PartialEq, Eq, Debug)]
pub struct StakeHistory(Arc<StakeHistoryInner>);

#[derive(Debug, Clone)]
pub struct FeatureSet {
    pub active: AHashMap<Pubkey, u64>, // randomness
    pub inactive: AHashSet<Pubkey>,  // randomness 
}

/// A [`HashMap`](std::collections::HashMap) using [`RandomState`](crate::RandomState) to hash the items.
/// (Requires the `std` feature to be enabled.)
#[derive(Clone, Debug)]
pub struct AHashMap<K, V, S = RandomState>(HashMap<K, V, S>);

/// A [`HashSet`](std::collections::HashSet) using [`RandomState`](crate::RandomState) to hash the items.
/// (Requires the `std` feature to be enabled.)
#[derive(Clone, Debug)]
pub struct AHashSet<T, S = RandomState>(HashSet<T, S>);

#[derive(Debug,Clone, Default, PartialEq, Eq)]
pub struct Fees {
    pub fee_calculator: FeeCalculator,
}

#[derive(Default, PartialEq, Eq, Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct FeeCalculator {
    /// The current cost of a signature.
    ///
    /// This amount may increase/decrease over time based on cluster processing
    /// load.
    pub lamports_per_signature: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecentBlockhashes(Vec<Entry>);

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Entry {
    pub blockhash: Hash,
    pub fee_calculator: FeeCalculator,
}

#[derive(Default, Clone, Debug)]
pub struct SysvarCache {
    // full account data as provided by bank, including any trailing zero bytes
    clock: Option<Vec<u8>>,
    epoch_schedule: Option<Vec<u8>>,
    epoch_rewards: Option<Vec<u8>>,
    rent: Option<Vec<u8>>,
    slot_hashes: Option<Vec<u8>>,
    stake_history: Option<Vec<u8>>,
    last_restart_slot: Option<Vec<u8>>,

    // object representations of large sysvars for convenience
    // these are used by the stake and vote builtin programs
    // these should be removed once those programs are ported to bpf
    slot_hashes_obj: Option<Arc<SlotHashes>>,
    stake_history_obj: Option<Arc<StakeHistory>>,

    // deprecated sysvars, these should be removed once practical
    #[allow(deprecated)]
    fees: Option<Fees>,
    #[allow(deprecated)]
    recent_blockhashes: Option<RecentBlockhashes>,
}

pub struct EnvironmentConfig<'a> {
    pub blockhash: Hash,
    pub blockhash_lamports_per_signature: u64,
    epoch_total_stake: u64,
    get_epoch_vote_account_stake_callback: &'a dyn Fn(&'a Pubkey) -> u64,
    pub feature_set: Arc<FeatureSet>,
    sysvar_cache: &'a SysvarCache,
}