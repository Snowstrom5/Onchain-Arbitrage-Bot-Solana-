use crate::{
    constants::SOL_MINT,
    dex::raydium::{clmm_info::POOL_TICK_ARRAY_BITMAP_SEED, raydium_clmm_program_id},
};
use solana_program::pubkey::Pubkey;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct RaydiumPool {
    pub pool: Pubkey,
    pub token_vault: Pubkey,
    pub sol_vault: Pubkey,
}

#[derive(Debug, Clone)]
pub struct RaydiumCpPool {
    pub pool: Pubkey,
    pub token_vault: Pubkey,
    pub sol_vault: Pubkey,
    pub amm_config: Pubkey,
    pub observation: Pubkey,
}

#[derive(Debug, Clone)]
pub struct PumpPool {
    pub pool: Pubkey,
    pub token_vault: Pubkey,
    pub sol_vault: Pubkey,
    pub fee_token_wallet: Pubkey,
    pub coin_creator_vault_ata: Pubkey,
    pub coin_creator_vault_authority: Pubkey,
}

#[derive(Debug, Clone)]
pub struct DlmmPool {
    pub pair: Pubkey,
    pub token_vault: Pubkey,
    pub sol_vault: Pubkey,
    pub oracle: Pubkey,
    pub bin_arrays: Vec<Pubkey>,
}

#[derive(Debug, Clone)]
pub struct WhirlpoolPool {
    pub pool: Pubkey,
    pub oracle: Pubkey,
    pub x_vault: Pubkey,
    pub y_vault: Pubkey,
    pub tick_arrays: Vec<Pubkey>,
}

#[derive(Debug, Clone)]
pub struct RaydiumClmmPool {
    pub pool: Pubkey,
    pub amm_config: Pubkey,
    pub observation_state: Pubkey,
    pub bitmap_extension: Pubkey,
    pub x_vault: Pubkey,
    pub y_vault: Pubkey,
    pub tick_arrays: Vec<Pubkey>,
}

#[derive(Debug, Clone)]
pub struct MeteoraDAmmPool {
    pub pool: Pubkey,
    pub token_x_vault: Pubkey,
    pub token_sol_vault: Pubkey,
    pub token_x_token_vault: Pubkey,
    pub token_sol_token_vault: Pubkey,
    pub token_x_lp_mint: Pubkey,
    pub token_sol_lp_mint: Pubkey,
    pub token_x_pool_lp: Pubkey,
    pub token_sol_pool_lp: Pubkey,
    pub admin_token_fee_x: Pubkey,
    pub admin_token_fee_sol: Pubkey,
}

#[derive(Debug, Clone)]
pub struct MintPoolData {
    pub mint: Pubkey,
    pub wallet_account: Pubkey,
    pub wallet_wsol_account: Pubkey,
    pub raydium_pools: Vec<RaydiumPool>,
    pub raydium_cp_pools: Vec<RaydiumCpPool>,
    pub pump_pools: Vec<PumpPool>,
    pub dlmm_pairs: Vec<DlmmPool>,
    pub whirlpool_pools: Vec<WhirlpoolPool>,
    pub raydium_clmm_pools: Vec<RaydiumClmmPool>,
    pub meteora_damm_pools: Vec<MeteoraDAmmPool>,
}

impl MintPoolData {
    pub fn new(mint: &str, wallet_account: &str) -> anyhow::Result<Self> {
        let sol_mint = Pubkey::from_str(SOL_MINT)?;
        let wallet_pk = Pubkey::from_str(wallet_account)?;
        let wallet_wsol_pk =
            spl_associated_token_account::get_associated_token_address(&wallet_pk, &sol_mint);
        Ok(Self {
            mint: Pubkey::from_str(mint)?,
            wallet_account: wallet_pk,
            wallet_wsol_account: wallet_wsol_pk,
            raydium_pools: Vec::new(),
            raydium_cp_pools: Vec::new(),
            pump_pools: Vec::new(),
            dlmm_pairs: Vec::new(),
            whirlpool_pools: Vec::new(),
            raydium_clmm_pools: Vec::new(),
            meteora_damm_pools: Vec::new(),
        })
    }

    pub fn add_raydium_pool(
        &mut self,
        pool: &str,
        token_vault: &str,
        sol_vault: &str,
    ) -> anyhow::Result<()> {
        self.raydium_pools.push(RaydiumPool {
            pool: Pubkey::from_str(pool)?,
            token_vault: Pubkey::from_str(token_vault)?,
            sol_vault: Pubkey::from_str(sol_vault)?,
        });
        Ok(())
    }

    pub fn add_raydium_cp_pool(
        &mut self,
        pool: &str,
        token_vault: &str,
        sol_vault: &str,
        amm_config: &str,
        observation: &str,
    ) -> anyhow::Result<()> {
        self.raydium_cp_pools.push(RaydiumCpPool {
            pool: Pubkey::from_str(pool)?,
            token_vault: Pubkey::from_str(token_vault)?,
            sol_vault: Pubkey::from_str(sol_vault)?,
            amm_config: Pubkey::from_str(amm_config)?,
            observation: Pubkey::from_str(observation)?,
        });
        Ok(())
    }

    pub fn add_pump_pool(
        &mut self,
        pool: &str,
        token_vault: &str,
        sol_vault: &str,
        fee_token_wallet: &str,
        coin_creator_vault_ata: &str,
        coin_creator_authority: &str,
    ) -> anyhow::Result<()> {
        self.pump_pools.push(PumpPool {
            pool: Pubkey::from_str(pool)?,
            token_vault: Pubkey::from_str(token_vault)?,
            sol_vault: Pubkey::from_str(sol_vault)?,
            fee_token_wallet: Pubkey::from_str(fee_token_wallet)?,
            coin_creator_vault_ata: Pubkey::from_str(coin_creator_vault_ata)?,
            coin_creator_vault_authority: Pubkey::from_str(coin_creator_authority)?,
        });
        Ok(())
    }

    pub fn add_dlmm_pool(
        &mut self,
        pair: &str,
        token_vault: &str,
        sol_vault: &str,
        oracle: &str,
        bin_arrays: Vec<&str>,
    ) -> anyhow::Result<()> {
        let bin_array_pubkeys = bin_arrays
            .iter()
            .map(|&s| Pubkey::from_str(s))
            .collect::<Result<Vec<_>, _>>()?;

        self.dlmm_pairs.push(DlmmPool {
            pair: Pubkey::from_str(pair)?,
            token_vault: Pubkey::from_str(token_vault)?,
            sol_vault: Pubkey::from_str(sol_vault)?,
            oracle: Pubkey::from_str(oracle)?,
            bin_arrays: bin_array_pubkeys,
        });
        Ok(())
    }

    pub fn add_whirlpool_pool(
        &mut self,
        pool: &str,
        oracle: &str,
        x_vault: &str,
        y_vault: &str,
        tick_arrays: Vec<&str>,
    ) -> anyhow::Result<()> {
        let tick_array_pubkeys = tick_arrays
            .iter()
            .map(|&s| Pubkey::from_str(s))
            .collect::<Result<Vec<_>, _>>()?;

        self.whirlpool_pools.push(WhirlpoolPool {
            pool: Pubkey::from_str(pool)?,
            oracle: Pubkey::from_str(oracle)?,
            x_vault: Pubkey::from_str(x_vault)?,
            y_vault: Pubkey::from_str(y_vault)?,
            tick_arrays: tick_array_pubkeys,
        });
        Ok(())
    }

    pub fn add_raydium_clmm_pool(
        &mut self,
        pool: &str,
        amm_config: &str,
        observation_state: &str,
        x_vault: &str,
        y_vault: &str,
        tick_arrays: Vec<&str>,
    ) -> anyhow::Result<()> {
        let pool_pubkey = Pubkey::from_str(pool)?;
        let bitmap_extension = Pubkey::find_program_address(
            &[
                POOL_TICK_ARRAY_BITMAP_SEED.as_bytes(),
                &pool_pubkey.as_ref(),
            ],
            &raydium_clmm_program_id(),
        )
        .0;
        let tick_array_pubkeys = tick_arrays
            .iter()
            .map(|&s| Pubkey::from_str(s))
            .collect::<Result<Vec<_>, _>>()?;

        self.raydium_clmm_pools.push(RaydiumClmmPool {
            pool: pool_pubkey,
            amm_config: Pubkey::from_str(amm_config)?,
            observation_state: Pubkey::from_str(observation_state)?,
            x_vault: Pubkey::from_str(x_vault)?,
            y_vault: Pubkey::from_str(y_vault)?,
            bitmap_extension,
            tick_arrays: tick_array_pubkeys,
        });
        Ok(())
    }

    pub fn add_meteora_damm_pool(
        &mut self,
        pool: &str,
        token_x_vault: &str,
        token_sol_vault: &str,
        token_x_token_vault: &str,
        token_sol_token_vault: &str,
        token_x_lp_mint: &str,
        token_sol_lp_mint: &str,
        token_x_pool_lp: &str,
        token_sol_pool_lp: &str,
        admin_token_fee_x: &str,
        admin_token_fee_sol: &str,
    ) -> anyhow::Result<()> {
        self.meteora_damm_pools.push(MeteoraDAmmPool {
            pool: Pubkey::from_str(pool)?,
            token_x_vault: Pubkey::from_str(token_x_vault)?,
            token_sol_vault: Pubkey::from_str(token_sol_vault)?,
            token_x_token_vault: Pubkey::from_str(token_x_token_vault)?,
            token_sol_token_vault: Pubkey::from_str(token_sol_token_vault)?,
            token_x_lp_mint: Pubkey::from_str(token_x_lp_mint)?,
            token_sol_lp_mint: Pubkey::from_str(token_sol_lp_mint)?,
            token_x_pool_lp: Pubkey::from_str(token_x_pool_lp)?,
            token_sol_pool_lp: Pubkey::from_str(token_sol_pool_lp)?,
            admin_token_fee_x: Pubkey::from_str(admin_token_fee_x)?,
            admin_token_fee_sol: Pubkey::from_str(admin_token_fee_sol)?,
        });
        Ok(())
    }
}
