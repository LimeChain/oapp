use crate::{
    consts::{BRIDGE_SEED, REMOTE_SEED},
    state::{Bridge, Remote},
    *,
};

#[derive(Accounts)]
#[instruction(params: SetRemoteParams)]
pub struct SetRemote<'info> {
    #[account(mut, address = bridge.admin)]
    pub admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        space = Remote::SIZE,
        seeds = [REMOTE_SEED, &params.dst_eid.to_be_bytes()],
        bump
    )]
    pub remote: Account<'info, Remote>,
    #[account(seeds = [BRIDGE_SEED], bump = bridge.bump)]
    pub bridge: Account<'info, Bridge>,
    pub system_program: Program<'info, System>,
}

pub fn set_remote(ctx: &mut Context<SetRemote>, params: &SetRemoteParams) -> Result<()> {
    ctx.accounts.remote.address = params.remote;
    ctx.accounts.remote.bump = ctx.bumps.remote;
    Ok(())
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct SetRemoteParams {
    pub dst_eid: u32,
    pub remote: [u8; 32],
}
