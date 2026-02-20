use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("3aNa219G1VwZq2gC8NRL9NRGXCbrhnhqivHDpmZS8TeA");

#[program]
pub mod migi_trust {
    use super::*;

    pub fn inicializar_handshake_vault(ctx: Context<InicializarVault>, amount_usdc: u64) -> Result<()> {
        let transfer_instruction = Transfer {
            from: ctx.accounts.cliente_usdc.to_account_info(),
            to: ctx.accounts.protocol_vault.to_account_info(),
            authority: ctx.accounts.cliente.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), transfer_instruction);
        token::transfer(cpi_ctx, amount_usdc)?;

        let acordo = &mut ctx.accounts.acordo_dados;
        acordo.cliente = *ctx.accounts.cliente.key;
        acordo.prestador = *ctx.accounts.prestador.key;
        acordo.valor_travado = amount_usdc;
        acordo.status_sbt = 1;
        acordo.timestamp_inicio = Clock::get()?.unix_timestamp;
        acordo.em_disputa = false;
        acordo.liberado = false;

        msg!("Handshake iniciado.");
        Ok(())
    }

    pub fn abrir_disputa(ctx: Context<GerenciarAcordo>) -> Result<()> {
        ctx.accounts.acordo_dados.em_disputa = true;
        msg!("Disputa aberta.");
        Ok(())
    }

    pub fn executar_liquidacao(ctx: Context<ExecutarLiquidacao>) -> Result<()> {
        let acordo = &mut ctx.accounts.acordo_dados;
        require!(!acordo.em_disputa, ErrorCode::EmDisputa);

        let seeds = &[b"vault".as_ref(), &[ctx.bumps.protocol_vault]];
        let signer = &[&seeds[..]];

        let transfer_instruction = Transfer {
            from: ctx.accounts.protocol_vault.to_account_info(),
            to: ctx.accounts.prestador_usdc.to_account_info(),
            authority: ctx.accounts.protocol_vault.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(), transfer_instruction, signer);
        token::transfer(cpi_ctx, acordo.valor_travado)?;

        acordo.liberado = true;
        acordo.status_sbt = 2;

        msg!("Liquidacao concluida.");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InicializarVault<'info> {
    #[account(init, payer = cliente, space = 8 + 32 + 32 + 8 + 1 + 8 + 1 + 1)]
    pub acordo_dados: Account<'info, AcordoMetadata>,
    #[account(mut)]
    pub cliente: Signer<'info>,
    /// CHECK: Apenas armazena a pubkey do prestador para registro
    pub prestador: AccountInfo<'info>,
    #[account(mut)]
    pub cliente_usdc: Account<'info, TokenAccount>,
    #[account(init, payer = cliente, seeds = [b"vault"], bump, token::mint = mint_usdc, token::authority = protocol_vault)]
    pub protocol_vault: Account<'info, TokenAccount>,
    pub mint_usdc: Account<'info, token::Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct ExecutarLiquidacao<'info> {
    #[account(mut, has_one = cliente)]
    pub acordo_dados: Account<'info, AcordoMetadata>,
    #[account(mut, seeds = [b"vault"], bump)]
    pub protocol_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub prestador_usdc: Account<'info, TokenAccount>,
    pub cliente: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct GerenciarAcordo<'info> {
    #[account(mut, has_one = cliente)]
    pub acordo_dados: Account<'info, AcordoMetadata>,
    pub cliente: Signer<'info>,
}

#[account]
pub struct AcordoMetadata {
    pub cliente: Pubkey,
    pub prestador: Pubkey,
    pub valor_travado: u64,
    pub status_sbt: u8,
    pub timestamp_inicio: i64,
    pub em_disputa: bool,
    pub liberado: bool,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Operacao travada: acordo em disputa.")]
    EmDisputa,
}
