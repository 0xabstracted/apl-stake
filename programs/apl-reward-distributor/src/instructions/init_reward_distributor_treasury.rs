use {
    crate::{errors::ErrorCode, state::*},
    anchor_lang::prelude::*,
    anchor_spl::{associated_token::AssociatedToken, token::{self,Mint, Token, TokenAccount}},
    apl_stake_pool::state::StakePool,
};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitRewardDistributorTreasuryIx {
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
pub struct InitRewardDistributorTreasuryCtx<'info> {
    #[account(
        init,
        payer = payer,
        space = REWARD_DISTRIBUTOR_SIZE,
        seeds = [REWARD_DISTRIBUTOR_SEED.as_bytes(), stake_pool.key().as_ref()],
        bump,
    )]
    reward_distributor_treasury: Box<Account<'info, RewardDistributor>>,
    stake_pool: Box<Account<'info, StakePool>>,
    #[account(mut)]
    reward_mint: Box<Account<'info, Mint>>,
    #[account(init_if_needed,
        associated_token::mint = reward_mint,
        associated_token::authority = reward_distributor_treasury,
        payer = payer)]
    pub reward_distributor_treasury_token_account: Box<Account<'info, TokenAccount>>,
    // #[account(init_if_needed,
    //     associated_token::mint = reward_mint,
    //     associated_token::authority = authority,
    //     payer = payer)]
    #[account(mut)]
    pub authority_token_account: Box<Account<'info, TokenAccount>>,
    
    #[account(mut)]
    authority: Signer<'info>,
    #[account(mut)]
    payer: Signer<'info>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>,
    rent: Sysvar<'info, Rent>,
}

pub fn handler<'key, 'accounts, 'remaining, 'info>(ctx: Context<'key, 'accounts, 'remaining, 'info, InitRewardDistributorTreasuryCtx<'info>>, ix: InitRewardDistributorTreasuryIx) -> Result<()> {
    let reward_distributor_treasury = &mut ctx.accounts.reward_distributor_treasury;
    reward_distributor_treasury.bump = *ctx.bumps.get("reward_distributor_treasury").unwrap();
    reward_distributor_treasury.kind = 2;
    reward_distributor_treasury.authority = ctx.accounts.authority.key();
    reward_distributor_treasury.stake_pool = ctx.accounts.stake_pool.key();
    reward_distributor_treasury.reward_mint = ctx.accounts.reward_mint.key();
    reward_distributor_treasury.reward_amount = ix.reward_amount;
    reward_distributor_treasury.reward_duration_seconds = ix.reward_duration_seconds as u128;
    reward_distributor_treasury.max_supply = ix.max_supply;
    reward_distributor_treasury.default_multiplier = ix.default_multiplier.unwrap_or(1);
    reward_distributor_treasury.multiplier_decimals = ix.multiplier_decimals.unwrap_or(0);
    reward_distributor_treasury.max_reward_seconds_received = ix.max_reward_seconds_received;

    // let remaining_accs = &mut ctx.remaining_accounts.iter();
    if ix.supply.is_none() && ix.max_supply.is_none() {
        return Err(error!(ErrorCode::SupplyRequired));
    }
    let reward_distributor_treasury_token_account = &mut ctx.accounts.reward_distributor_treasury_token_account;
    let authority_token_account = &mut ctx.accounts.authority_token_account;
    // let reward_distributor_treasury_token_account = Account::<TokenAccount>::try_from(reward_distributor_treasury_token_account_info)?;
    // let authority_token_account_info = next_account_info(remaining_accs)?;
    // let authority_token_account = Account::<TokenAccount>::try_from(authority_token_account_info)?;

    let cpi_accounts = token::Transfer {
        from: authority_token_account.to_account_info(),
        to: reward_distributor_treasury_token_account.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_context, ix.supply.unwrap_or_else(|| ix.max_supply.unwrap()))?;
    Ok(())
}
