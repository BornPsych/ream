use alloy_primitives::{aliases::B32, fixed_bytes};

pub const BASE_REWARD_FACTOR: u64 = 64;
pub const BLS_WITHDRAWAL_PREFIX: &[u8] = &[0];
pub const CAPELLA_FORK_VERSION: B32 = fixed_bytes!("0x03000000");
pub const CHURN_LIMIT_QUOTIENT: u64 = 65536;
pub const DEPOSIT_CONTRACT_TREE_DEPTH: u64 = 32;
pub const DOMAIN_BEACON_ATTESTER: B32 = fixed_bytes!("0x01000000");
pub const DOMAIN_BEACON_PROPOSER: B32 = fixed_bytes!("0x00000000");
pub const DOMAIN_BLS_TO_EXECUTION_CHANGE: B32 = fixed_bytes!("0x0A000000");
pub const DOMAIN_DEPOSIT: B32 = fixed_bytes!("0x03000000");
pub const DOMAIN_RANDAO: B32 = fixed_bytes!("0x02000000");
pub const DOMAIN_SYNC_COMMITTEE: B32 = fixed_bytes!("0x07000000");
pub const DOMAIN_VOLUNTARY_EXIT: B32 = fixed_bytes!("0x04000000");
pub const EFFECTIVE_BALANCE_INCREMENT: u64 = 1_000_000_000;
pub const EJECTION_BALANCE: u64 = 16000000000;
pub const EPOCHS_PER_ETH1_VOTING_PERIOD: u64 = 64;
pub const EPOCHS_PER_HISTORICAL_VECTOR: u64 = 65536;
pub const EPOCHS_PER_SLASHINGS_VECTOR: u64 = 8192;
pub const EPOCHS_PER_SYNC_COMMITTEE_PERIOD: u64 = 256;
pub const ETH1_ADDRESS_WITHDRAWAL_PREFIX: [u8; 1] = [1];
pub const FAR_FUTURE_EPOCH: u64 = 18446744073709551615;
pub const GENESIS_SLOT: u64 = 0;
pub const GENESIS_EPOCH: u64 = 0;
pub const GENESIS_FORK_VERSION: B32 = fixed_bytes!("0x00000000");
pub const HYSTERESIS_DOWNWARD_MULTIPLIER: u64 = 1;
pub const HYSTERESIS_UPWARD_MULTIPLIER: u64 = 5;
pub const HYSTERESIS_QUOTIENT: u64 = 4;
pub const INACTIVITY_PENALTY_QUOTIENT_ALTAIR: u64 = 50331648;
pub const INTERVALS_PER_SLOT: u64 = 3;
pub const INACTIVITY_SCORE_BIAS: u64 = 4;
pub const INACTIVITY_SCORE_RECOVERY_RATE: u64 = 16;
pub const JUSTIFICATION_BITS_LENGTH: u64 = 4;
pub const MAX_BLOBS_PER_BLOCK: u64 = 6;
pub const MAX_COMMITTEES_PER_SLOT: u64 = 64;
pub const MAX_DEPOSITS: u64 = 16;
pub const MAX_SEED_LOOKAHEAD: u64 = 4;
pub const MAX_EFFECTIVE_BALANCE: u64 = 32_000_000_000;
pub const MAX_PER_EPOCH_ACTIVATION_CHURN_LIMIT: u64 = 8;
pub const MAX_RANDOM_BYTE: u64 = 255;
pub const MAX_VALIDATORS_PER_WITHDRAWALS_SWEEP: usize = 16384;
pub const MAX_WITHDRAWALS_PER_PAYLOAD: u64 = 16;
pub const MIN_ATTESTATION_INCLUSION_DELAY: u64 = 1;
pub const MIN_EPOCHS_TO_INACTIVITY_PENALTY: u64 = 4;
pub const MIN_GENESIS_ACTIVE_VALIDATOR_COUNT: u64 = 16384;
pub const MIN_GENESIS_TIME: u64 = 1606824000;
pub const MIN_PER_EPOCH_CHURN_LIMIT: u64 = 4;
pub const MIN_SEED_LOOKAHEAD: u64 = 1;
pub const MIN_SLASHING_PENALTY_QUOTIENT: u64 = 32; // updated value in Bellatrix
pub const MIN_VALIDATOR_WITHDRAWABILITY_DELAY: u64 = 256;
pub const NUM_FLAG_INDICES: usize = 3;
pub const PROPORTIONAL_SLASHING_MULTIPLIER_BELLATRIX: u64 = 3;
pub const PROPOSER_REWARD_QUOTIENT: u64 = 8;
pub const PROPOSER_SCORE_BOOST: u64 = 40;
pub const PROPOSER_WEIGHT: u64 = 8;
pub const REORG_MAX_EPOCHS_SINCE_FINALIZATION: u64 = 2;
pub const REORG_HEAD_WEIGHT_THRESHOLD: u64 = 20;
pub const REORG_PARENT_WEIGHT_THRESHOLD: u64 = 160;
pub const SECONDS_PER_SLOT: u64 = 12;
pub const SHARD_COMMITTEE_PERIOD: u64 = 256;
pub const SHUFFLE_ROUND_COUNT: u8 = 90;
pub const SLOTS_PER_EPOCH: u64 = 32;
pub const SLOTS_PER_HISTORICAL_ROOT: u64 = 8192;
pub const SYNC_COMMITTEE_SIZE: u64 = 512;
pub const SYNC_REWARD_WEIGHT: u64 = 2;
pub const TARGET_COMMITTEE_SIZE: u64 = 128;
pub const TIMELY_HEAD_FLAG_INDEX: u8 = 2;
pub const TIMELY_SOURCE_FLAG_INDEX: u8 = 0;
pub const TIMELY_TARGET_FLAG_INDEX: u8 = 1;
pub const TIMELY_SOURCE_WEIGHT: u64 = 14;
pub const TIMELY_TARGET_WEIGHT: u64 = 26;
pub const TIMELY_HEAD_WEIGHT: u64 = 14;
pub const WEIGHT_DENOMINATOR: u64 = 64;
pub const WHISTLEBLOWER_REWARD_QUOTIENT: u64 = 512;

pub const PARTICIPATION_FLAG_WEIGHTS: [u64; NUM_FLAG_INDICES] = [
    TIMELY_SOURCE_WEIGHT,
    TIMELY_TARGET_WEIGHT,
    TIMELY_HEAD_WEIGHT,
];
