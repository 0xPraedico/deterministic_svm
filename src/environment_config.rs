
pub type Pubkey = [u8; 32];
/// Size of a hash in bytes.
pub const HASH_BYTES: usize = 32;
pub struct Hash(pub(crate) [u8; HASH_BYTES]);


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FeatureSet {
    pub active: AHashMap<Pubkey, u64>,
    pub inactive: AHashSet<Pubkey>,
}
pub struct EnvironmentConfig<'a> {
    pub blockhash: Hash,
    pub blockhash_lamports_per_signature: u64,
    epoch_total_stake: u64,
    get_epoch_vote_account_stake_callback: &'a dyn Fn(&'a Pubkey) -> u64,
    pub feature_set: Arc<FeatureSet>,
    sysvar_cache: &'a SysvarCache,
}