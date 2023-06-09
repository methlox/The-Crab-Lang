// use anchor_lang::prelude::*;
// use puppet::cpi::accounts::SetData;
// use puppet::program::Puppet;
// use puppet::{self, Data};

// declare_id!("HmbTLCmaGvZhKnn1Zfa1JVnp7vkMV4DYVxPLWBVoN65L");

// #[program]
// mod puppet_master {
//     use super::*;
//     pub fn pull_strings(ctx: Context<PullStrings>, data: u64) -> Result<()> {
//         let cpi_program = ctx.accounts.puppet_program.to_account_info();
//         let cpi_accounts = SetData {
//             puppet: ctx.accounts.puppet.to_account_info(),
//         };
//         let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
//         puppet::cpi::set_data(cpi_ctx, data)
//     }
// }

// #[derive(Accounts)]
// pub struct PullStrings<'info> {
//     #[account(mut)]
//     pub puppet: Account<'info, Data>,
//     pub puppet_program: Program<'info, Puppet>,
// }

use anchor_lang::prelude::*;
use puppet::cpi::accounts::SetData;
use puppet::program::Puppet;
use puppet::{self, Data};

declare_id!("6hXzEwQBihTqKfJ6piC9Ekkvmb42rZZdhuhtSXswzYHa");

#[program]
mod puppet_master {
    use super::*;
    pub fn pull_strings(ctx: Context<PullStrings>, data: u64) -> Result<()> {
        puppet::cpi::set_data(ctx.accounts.set_data_ctx(), data)
    }
}

#[derive(Accounts)]
pub struct PullStrings<'info> {
    #[account(mut)]
    pub puppet: Account<'info, Data>,
    pub puppet_program: Program<'info, Puppet>,
}

impl<'info> PullStrings<'info> {
    pub fn set_data_ctx(&self) -> CpiContext<'_, '_, '_, 'info, SetData<'info>> {
        let cpi_program = self.puppet_program.to_account_info();
        let cpi_accounts = SetData {
            puppet: self.puppet.to_account_info()
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}
