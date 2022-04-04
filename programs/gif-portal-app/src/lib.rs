use anchor_lang::prelude::*;

declare_id!("4yj5VKgX1ieoeQRvn97nd9zbv9RA3VxM2rJzwUNcS7xq");

#[program]
pub mod git_portal_app {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        // Get a reference to the account.
        let base_account = &mut ctx.accounts.base_account;
        // Initialize total_gifs.
        base_account.total_gifs_count = 0;
        Ok(())
    }

    pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> ProgramResult {
        // Get a reference to the account and increment total_gifs.
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        // Build the struct.
        let item = ItemStruct {
            gif_link: gif_link.to_string(),
            user_address: *user.to_account_info().key,
        };

        // Add it to the gif_list vector.
        base_account.gifs_list.push(item);
        base_account.total_gifs_count += 1;

        Ok(())
    }

    // pub fn send_sol(ctx: Context<SendSol>, amount: u64) -> ProgramResult {
    //     let ix = anchor_lang::solana_program::system_instruction::transfer(
    //         &ctx.accounts.from.key(),
    //         &ctx.accounts.to.key(),
    //         amount,
    //     );
    //
    //     anchor_lang::solana_program::program::invoke(
    //         &ix,
    //         &[
    //             ctx.accounts.from.to_account_info(),
    //             ctx.accounts.to.to_account_info(),
    //         ],
    //     )
    // }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

#[account]
pub struct BaseAccount {
    pub total_gifs_count: u64,
    // Attach a Vector of type ItemStruct to the account.
    pub gifs_list: Vec<ItemStruct>,
}

// Specify what data you want in the AddGif Context.
// Getting a handle on the flow of things :)?
#[derive(Accounts)]
pub struct AddGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

// Create a custom struct for us to work with.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
}

// #[derive(Accounts)]
// pub struct SendSol<'info> {
//     #[account(mut)]
//     from: Signer<'info>,
//     #[account(mut)]
//     to: AccountInfo<'info>,
//     system_program: Program<'info, System>,
// }
