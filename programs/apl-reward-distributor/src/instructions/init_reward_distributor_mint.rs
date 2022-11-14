use {
    crate::state::*,
    anchor_lang::prelude::*,
    anchor_spl::token::{self, Mint, SetAuthority, Token},
    apl_stake_pool::state::StakePool,
    spl_token::instruction::AuthorityType,
};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitRewardDistributorMintIx {
    pub reward_amount: u64,
    pub reward_duration_seconds: u128,
    // pub kind: u8,
    pub supply: Option<u64>,
    pub max_supply: Option<u64>,
    pub default_multiplier: Option<u64>,
    pub multiplier_decimals: Option<u8>,
    pub max_reward_seconds_received: Option<u128>,
}

#[derive(Accounts)]
pub struct InitRewardDistributorMintCtx<'info> {
    #[account(
        init,
        payer = payer,
        space = REWARD_DISTRIBUTOR_SIZE,
        seeds = [REWARD_DISTRIBUTOR_SEED.as_bytes(), stake_pool.key().as_ref()],
        bump,
    )]
    reward_distributor_mint: Box<Account<'info, RewardDistributor>>,
    stake_pool: Box<Account<'info, StakePool>>,
    #[account(mut)]
    reward_mint: Box<Account<'info, Mint>>,

    #[account(mut)]
    authority: Signer<'info>,
    #[account(mut)]
    payer: Signer<'info>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
}

pub fn handler<'key, 'accounts, 'remaining, 'info>(ctx: Context<'key, 'accounts, 'remaining, 'info, InitRewardDistributorMintCtx<'info>>, ix: InitRewardDistributorMintIx) -> Result<()> {
    let reward_distributor_mint = &mut ctx.accounts.reward_distributor_mint;
    reward_distributor_mint.bump = *ctx.bumps.get("reward_distributor_mint").unwrap();
    reward_distributor_mint.kind = 1;
    reward_distributor_mint.authority = ctx.accounts.authority.key();
    reward_distributor_mint.stake_pool = ctx.accounts.stake_pool.key();
    reward_distributor_mint.reward_mint = ctx.accounts.reward_mint.key();
    reward_distributor_mint.reward_amount = ix.reward_amount;
    reward_distributor_mint.reward_duration_seconds = ix.reward_duration_seconds as u128;
    reward_distributor_mint.max_supply = ix.max_supply;
    reward_distributor_mint.default_multiplier = ix.default_multiplier.unwrap_or(1);
    reward_distributor_mint.multiplier_decimals = ix.multiplier_decimals.unwrap_or(0);
    reward_distributor_mint.max_reward_seconds_received = ix.max_reward_seconds_received;

    let cpi_accounts = SetAuthority {
        account_or_mint: ctx.accounts.reward_mint.to_account_info(),
        current_authority: ctx.accounts.authority.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
    token::set_authority(cpi_context, AuthorityType::MintTokens, Some(reward_distributor_mint.key()))?;
    Ok(())
}
