use anchor_lang::prelude::*;

declare_id!("CHi4sCSpmcDVVzhefrWe2MJrERACUNUWC43eEXpPsnPC");

#[program]
pub mod marinade_idl {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
