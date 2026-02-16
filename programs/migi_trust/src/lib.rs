use anchor_lang::prelude::*;

declare_id!("3aNa219G1VwZq2gC8NRL9NRGXCbrhnhqivHDpmZS8TeA");

#[program]
pub mod migi_trust {
    use super::*;
    pub fn criar_perfil(ctx: Context<CriarPerfil>, nome: String) -> Result<()> {
        let perfil = &mut ctx.accounts.perfil;
        perfil.autor = *ctx.accounts.usuario.key;
        perfil.nome = nome;
        msg!("Perfil criado!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CriarPerfil<'info> {
    #[account(init, payer = usuario, space = 8 + 32 + 40)]
    pub perfil: Account<'info, Perfil>,
    #[account(mut)]
    pub usuario: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Perfil {
    pub autor: Pubkey,
    pub nome: String,
}
