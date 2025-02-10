use crate::*;


const REMOTE_SEED: &[u8] = b"Remote";
#[derive(Accounts)]
#[instruction(params: SetRemoteParams)]
pub struct SetRemote<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    //init_if_needed was part of bottom account attribute instead of init
    #[account(
        init,
        payer = admin,
        space = Remote::SIZE,
        seeds = [REMOTE_SEED, &params.dst_eid.to_be_bytes()],
        bump
    )]
    pub remote: Account<'info, Remote>,
    pub system_program: Program<'info, System>,
}

pub fn set_remote(ctx: Context<SetRemote>, params: SetRemoteParams) -> Result<()> {
    ctx.accounts.remote.address = params.remote;
    ctx.accounts.remote.bump = ctx.bumps.remote;
    Ok(())
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct SetRemoteParams {
    pub id: u8,
    pub dst_eid: u32,
    pub remote: [u8; 32],
}

#[account]
pub struct Remote {
    pub address: [u8; 32],
    pub bump: u8,
}

impl Remote {
    pub const SIZE: usize = 8 + std::mem::size_of::<Self>();
}
