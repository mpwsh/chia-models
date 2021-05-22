use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub(crate) struct BlockchainStateResponse {
	pub blockchain_state: BlockchainState,
	pub success: bool
}

#[derive(Debug, Deserialize)]
pub struct BlockchainState {
	pub difficulty: u16,
	pub genesis_challenge_initialized: bool,
	pub mempool_size: u8,
	pub peak: BlockchainStatePeak,
	pub space: f64,
	pub sub_slot_iters: u32,
	pub sync: BlockchainStateSync
}

#[derive(Debug, Deserialize)]
pub struct BlockchainStatePeak {
	pub challenge_block_info_hash: String,
	pub challenge_vdf_output: ChallengeVdfOutput,
	pub deficit: u8,
	pub farmer_puzzle_hash: String,
	pub fees: Value,
	pub finished_challenge_slot_hashes: Value,
	pub finished_infused_challenge_slot_hashes: Value,
	pub finished_reward_slot_hashes: Value,
	pub header_hash: String,
	pub height: u32,
	pub infused_challenge_vdf_output: Value,
	pub overflow: bool,
	pub pool_puzzle_hash: String,
	pub prev_hash: String,
	pub prev_transaction_block_hash: Value,
	pub prev_transaction_block_height: u32,
	pub required_iters: u32,
	pub reward_claims_incorporated: Value,
	pub reward_infusion_new_challenge: String,
	pub signage_point_index: u8,
	pub sub_epoch_summary_included: Value,
	pub sub_slot_iters: u32,
	#[serde(deserialize_with = "crate::util::deserialize_optional_timestamp")]
	pub timestamp: Option<DateTime<Utc>>,
	pub total_iters: u64,
	pub weight: u32
}

#[derive(Debug, Deserialize)]
pub struct ChallengeVdfOutput {
	data: String
}

#[derive(Debug, Deserialize)]
pub struct BlockchainStateSync {
	sync_mode: bool,
	sync_progress_height: u32,
	sync_tip_height: u32,
	synced: bool
}

