pub mod errors;
pub mod instructions;
pub mod state;

use {anchor_lang::prelude::*, instructions::*};

declare_id!("apmYJ4wgVkCm5AWYnuJCqr8eBbu7hm89DcLgQEkgbG2");

#[program]
pub mod apl_payment_manager {
    use super::*;

    pub fn init(ctx: Context<InitCtx>, ix: InitIx) -> Result<()> {
        init::handler(ctx, ix)
    }

    pub fn manage_payment(ctx: Context<HandlePaymentCtx>, payment_amount: u64) -> Result<()> {
        handle_payment::handler(ctx, payment_amount)
    }

    pub fn handle_payment_with_royalties<'key, 'accounts, 'remaining, 'info>(
        ctx: Context<'key, 'accounts, 'remaining, 'info, HandlePaymentWithRoyaltiesCtx<'info>>,
        payment_amount: u64,
    ) -> Result<()> {
        handle_payment_with_royalties::handler(ctx, payment_amount)
    }

    pub fn handle_native_payment_with_royalties<'key, 'accounts, 'remaining, 'info>(
        ctx: Context<'key, 'accounts, 'remaining, 'info, HandleNativePaymentWithRoyaltiesCtx<'info>>,
        payment_amount: u64,
    ) -> Result<()> {
        handle_native_payment_with_royalties::handler(ctx, payment_amount)
    }

    pub fn close(ctx: Context<CloseCtx>) -> Result<()> {
        close::handler(ctx)
    }

    pub fn update(ctx: Context<UpdateCtx>, ix: UpdateIx) -> Result<()> {
        update::handler(ctx, ix)
    }
}
