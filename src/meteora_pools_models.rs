use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::pubkey::Pubkey;

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Pool {
    pub discriminator: [u8; 8], // 0-7    (8 bytes)
    pub lp_mint: Pubkey,        // 8-39   (32 bytes)
    pub token_a_mint: Pubkey,   // 40-71  (32 bytes)
    pub token_b_mint: Pubkey,   // 72-103 (32 bytes)
    pub a_vault: Pubkey,
    pub b_vault: Pubkey,
    pub a_vault_lp: Pubkey,
    pub b_vault_lp: Pubkey,
    pub a_vault_lp_bump: u8,
    pub enabled: bool,
    pub protocol_token_a_fee: Pubkey,
    pub protocol_token_b_fee: Pubkey,
    pub fee_last_updated_at: u64,
    pub padding0: [u8; 24],
    pub fees: PoolFees,
    pub pool_type: PoolType,
    pub stake: Pubkey,
    pub total_locked_lp: u64,
    pub bootstrapping: Bootstrapping,
    pub partner_info: PartnerInfo,
    pub padding: Padding,
    pub curve_type: CurveType,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Padding {
    pub padding0: [u8; 6],
    pub padding1: [u64; 21],
    pub padding2: [u64; 21],
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct PoolFees {
    pub trade_fee_numerator: u64,
    pub trade_fee_denominator: u64,
    pub protocol_trade_fee_numerator: u64,
    pub protocol_trade_fee_denominator: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub enum PoolType {
    Permissioned,
    Permissionless,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Bootstrapping {
    pub activation_point: u64,
    pub whitelisted_vault: Pubkey,
    pub pool_creator: Pubkey,
    pub activation_type: u8,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct PartnerInfo {
    pub fee_numerator: u64,
    pub partner_authority: Pubkey,
    pub pending_fee_a: u64,
    pub pending_fee_b: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub enum CurveType {
    ConstantProduct,
    Stable {
        amp: u64,
        token_multiplier: TokenMultiplier,
        depeg: Depeg,
        last_amp_updated_timestamp: u64,
    },
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct TokenMultiplier {
    pub token_a_multiplier: u64,
    pub token_b_multiplier: u64,
    pub precision_factor: u8,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Depeg {
    pub base_virtual_price: u64,
    pub base_cache_updated: u64,
    pub depeg_type: DepegType,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub enum DepegType {
    None,
    Marinade,
    Lido,
    SplStake,
}
