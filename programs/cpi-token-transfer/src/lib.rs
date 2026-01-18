use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

// Protocol fee: 1% (100 basis points)
const PROTOCOL_FEE_BPS: u64 = 100;
const BPS_DENOMINATOR: u64 = 10000;

#[program]
pub mod cpi_token_transfer {
    use super::*;

    /// Initialize the protocol config
    /// TODO: Set up protocol fee recipient
    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        // TODO: Initialize protocol config with fee recipient
        Ok(())
    }

    /// Simple token transfer via CPI
    /// TODO: Implement basic CPI transfer
    /// - User signs the transfer
    /// - Call Token Program's transfer instruction
    pub fn transfer_tokens(
        _ctx: Context<TransferTokens>,
        _amount: u64,
    ) -> Result<()> {
        // TODO: Implement CPI to Token Program
        // Use token::transfer() with CpiContext
        Ok(())
    }

    /// Transfer with protocol fee
    /// TODO: Implement payment splitter
    /// - Calculate fee amount
    /// - Transfer fee to protocol
    /// - Transfer remainder to recipient
    pub fn transfer_with_fee(
        _ctx: Context<TransferWithFee>,
        amount: u64,
    ) -> Result<()> {
        // TODO: Calculate fee
        let _fee = amount
            .checked_mul(PROTOCOL_FEE_BPS)
            .unwrap()
            .checked_div(BPS_DENOMINATOR)
            .unwrap();
        
        // TODO: Transfer fee to protocol via CPI
        
        // TODO: Transfer remainder to recipient via CPI
        
        Ok(())
    }

    /// PDA-signed transfer from vault
    /// TODO: Implement CPI with PDA signer
    /// - Vault is a PDA owned by this program
    /// - Sign CPI with PDA seeds
    pub fn vault_transfer(
        _ctx: Context<VaultTransfer>,
        _amount: u64,
    ) -> Result<()> {
        // TODO: Implement CPI with PDA signer
        // Use CpiContext::new_with_signer() with seeds
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    // TODO: Add accounts for initialization
    // - Protocol config PDA
    // - Fee recipient
    
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TransferTokens<'info> {
    // TODO: Add accounts for simple transfer
    // Hints:
    // - User signs the transfer
    // - Source token account (user's)
    // - Destination token account
    // - Token Program for CPI
    
    pub user: Signer<'info>,
    
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct TransferWithFee<'info> {
    // TODO: Add accounts for transfer with fee
    // Hints:
    // - Same as TransferTokens PLUS
    // - Protocol fee token account
    // - Validate token mints match
    
    pub user: Signer<'info>,
    
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub protocol_fee_account: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct VaultTransfer<'info> {
    // TODO: Add accounts for PDA-signed transfer
    // Hints:
    // - Authority must match vault's authority
    // - Vault is a PDA token account
    // - Need vault PDA seeds for signing
    
    pub authority: Signer<'info>,
    
    /// CHECK: Vault PDA - used for signing
    pub vault_authority: AccountInfo<'info>,
    
    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
}

// TODO: Define ProtocolConfig account
// #[account]
// pub struct ProtocolConfig {
//     pub authority: Pubkey,
//     pub fee_recipient: Pubkey,
//     pub fee_bps: u64,
//     pub bump: u8,
// }

#[error_code]
pub enum CpiError {
    #[msg("Unauthorized: Invalid authority")]
    Unauthorized,
    #[msg("Invalid token mint")]
    InvalidMint,
    #[msg("Insufficient balance for transfer")]
    InsufficientBalance,
    #[msg("Invalid program for CPI")]
    InvalidProgram,
    #[msg("Arithmetic overflow")]
    Overflow,
}
