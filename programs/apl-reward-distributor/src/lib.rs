pub mod errors;
pub mod instructions;
pub mod state;

use {anchor_lang::prelude::*, instructions::*};

declare_id!("ardJ1zpGCk4RrBz3cNAbN2Kg2VubpR1HPu2RFXm3Y3E");

#[program]
pub mod apl_reward_distributor {
    use super::*;

    pub fn init_reward_distributor_mint<'key, 'accounts, 'remaining, 'info>(ctx: Context<'key, 'accounts, 'remaining, 'info, InitRewardDistributorMintCtx<'info>>, ix: InitRewardDistributorMintIx) -> Result<()> {
        init_reward_distributor_mint::handler(ctx, ix)
    }

    pub fn init_reward_distributor_treasury<'key, 'accounts, 'remaining, 'info>(ctx: Context<'key, 'accounts, 'remaining, 'info, InitRewardDistributorTreasuryCtx<'info>>, ix: InitRewardDistributorTreasuryIx) -> Result<()> {
        init_reward_distributor_treasury::handler(ctx, ix)
    }

    pub fn init_reward_entry(ctx: Context<InitRewardEntryCtx>) -> Result<()> {
        init_reward_entry::handler(ctx)
    }

    pub fn claim_rewards<'key, 'accounts, 'remaining, 'info>(ctx: Context<'key, 'accounts, 'remaining, 'info, ClaimRewardsCtx<'info>>) -> Result<()> {
        claim_rewards::handler(ctx)
    }

    pub fn update_reward_entry(ctx: Context<UpdateRewardEntryCtx>, ix: UpdateRewardEntryIx) -> Result<()> {
        update_reward_entry::handler(ctx, ix)
    }

    pub fn close_reward_distributor<'key, 'accounts, 'remaining, 'info>(ctx: Context<'key, 'accounts, 'remaining, 'info, CloseCtx<'info>>) -> Result<()> {
        close_reward_distributor::handler(ctx)
    }

    pub fn close_reward_entry(ctx: Context<CloseRewardEntryCtx>) -> Result<()> {
        close_reward_entry::handler(ctx)
    }

    pub fn update_reward_distributor(ctx: Context<UpdateRewardDistributorCtx>, ix: UpdateRewardDistributorIx) -> Result<()> {
        update_reward_distributor::handler(ctx, ix)
    }

    pub fn reclaim_funds(ctx: Context<ReclaimFundsCtx>, amount: u64) -> Result<()> {
        reclaim_funds::handler(ctx, amount)
    }
}
