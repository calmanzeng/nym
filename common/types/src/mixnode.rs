// Copyright 2022 - Nym Technologies SA <contact@nymtech.net>
// SPDX-License-Identifier: Apache-2.0

use crate::currency::{DecCoin, RegisteredCoins};
use crate::error::TypesError;
use cosmwasm_std::Decimal;
use mixnet_contract_common::{
    EpochId, MixId, MixNode, MixNodeBond as MixnetContractMixNodeBond,
    MixNodeCostParams as MixnetContractMixNodeCostParams,
    MixNodeDetails as MixnetContractMixNodeDetails,
    MixNodeRewarding as MixnetContractMixNodeRewarding, Percent,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt;

#[cfg_attr(feature = "generate-ts", derive(ts_rs::TS))]
#[cfg_attr(
    feature = "generate-ts",
    ts(export_to = "ts-packages/types/src/types/rust/MixNodeDetails.ts")
)]
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, JsonSchema)]
pub struct MixNodeDetails {
    pub bond_information: MixNodeBond,
    pub rewarding_details: MixNodeRewarding,
}

impl MixNodeDetails {
    pub fn from_mixnet_contract_mixnode_details(
        details: MixnetContractMixNodeDetails,
        reg: &RegisteredCoins,
    ) -> Result<MixNodeDetails, TypesError> {
        Ok(MixNodeDetails {
            bond_information: MixNodeBond::from_mixnet_contract_mixnode_bond(
                details.bond_information,
                reg,
            )?,
            rewarding_details: MixNodeRewarding::from_mixnet_contract_mixnode_rewarding(
                details.rewarding_details,
                reg,
            )?,
        })
    }
}

#[cfg_attr(feature = "generate-ts", derive(ts_rs::TS))]
#[cfg_attr(
    feature = "generate-ts",
    ts(export_to = "ts-packages/types/src/types/rust/MixNodeBond.ts")
)]
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, JsonSchema)]
pub struct MixNodeBond {
    pub mix_id: MixId,
    pub owner: String,
    pub original_pledge: DecCoin,
    pub layer: String,
    pub mix_node: MixNode,
    pub proxy: Option<String>,
    pub bonding_height: u64,
    pub is_unbonding: bool,
}

impl MixNodeBond {
    pub fn from_mixnet_contract_mixnode_bond(
        bond: MixnetContractMixNodeBond,
        reg: &RegisteredCoins,
    ) -> Result<MixNodeBond, TypesError> {
        Ok(MixNodeBond {
            mix_id: bond.mix_id,
            owner: bond.owner.into_string(),
            original_pledge: reg
                .attempt_convert_to_display_dec_coin(bond.original_pledge.into())?,
            layer: bond.layer.into(),
            mix_node: bond.mix_node,
            proxy: bond.proxy.map(|p| p.into_string()),
            bonding_height: bond.bonding_height,
            is_unbonding: bond.is_unbonding,
        })
    }
}

#[cfg_attr(feature = "generate-ts", derive(ts_rs::TS))]
#[cfg_attr(
    feature = "generate-ts",
    ts(export_to = "ts-packages/types/src/types/rust/MixNodeRewarding.ts")
)]
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, JsonSchema)]
pub struct MixNodeRewarding {
    pub cost_params: MixNodeCostParams,

    #[cfg_attr(feature = "generate-ts", ts(type = "string"))]
    pub operator: Decimal,

    #[cfg_attr(feature = "generate-ts", ts(type = "string"))]
    pub delegates: Decimal,

    #[cfg_attr(feature = "generate-ts", ts(type = "string"))]
    pub total_unit_reward: Decimal,

    #[cfg_attr(feature = "generate-ts", ts(type = "string"))]
    pub unit_delegation: Decimal,

    pub last_rewarded_epoch: EpochId,

    pub unique_delegations: u32,
}

impl MixNodeRewarding {
    pub fn from_mixnet_contract_mixnode_rewarding(
        mix_rewarding: MixnetContractMixNodeRewarding,
        reg: &RegisteredCoins,
    ) -> Result<MixNodeRewarding, TypesError> {
        Ok(MixNodeRewarding {
            cost_params: MixNodeCostParams::from_mixnet_contract_mixnode_cost_params(
                mix_rewarding.cost_params,
                reg,
            )?,
            operator: mix_rewarding.operator,
            delegates: mix_rewarding.delegates,
            total_unit_reward: mix_rewarding.total_unit_reward,
            unit_delegation: mix_rewarding.unit_delegation,
            last_rewarded_epoch: mix_rewarding.last_rewarded_epoch,
            unique_delegations: mix_rewarding.unique_delegations,
        })
    }
}

#[cfg_attr(feature = "generate-ts", derive(ts_rs::TS))]
#[cfg_attr(
    feature = "generate-ts",
    ts(export_to = "ts-packages/types/src/types/rust/MixNodeCostParams.ts")
)]
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, JsonSchema)]
pub struct MixNodeCostParams {
    #[cfg_attr(feature = "generate-ts", ts(type = "string"))]
    pub profit_margin_percent: Percent,

    pub interval_operating_cost: DecCoin,
}

impl MixNodeCostParams {
    pub fn from_mixnet_contract_mixnode_cost_params(
        cost_params: MixnetContractMixNodeCostParams,
        reg: &RegisteredCoins,
    ) -> Result<MixNodeCostParams, TypesError> {
        Ok(MixNodeCostParams {
            profit_margin_percent: cost_params.profit_margin_percent,
            interval_operating_cost: reg
                .attempt_convert_to_display_dec_coin(cost_params.interval_operating_cost.into())?,
        })
    }

    pub fn try_convert_to_mixnet_contract_cost_params(
        self,
        reg: &RegisteredCoins,
    ) -> Result<MixnetContractMixNodeCostParams, TypesError> {
        Ok(MixnetContractMixNodeCostParams {
            profit_margin_percent: self.profit_margin_percent,
            interval_operating_cost: reg
                .attempt_convert_to_base_coin(self.interval_operating_cost)?
                .into(),
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct MixnodeNodeDetailsResponse {
    pub identity_key: String,
    pub sphinx_key: String,
    pub owner_signature: String,
    pub announce_address: String,
    pub bind_address: String,
    pub version: String,
    pub mix_port: u16,
    pub http_api_port: u16,
    pub verloc_port: u16,
    pub wallet_address: Option<String>,
}

impl fmt::Display for MixnodeNodeDetailsResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let wallet_address = self.wallet_address.clone().unwrap_or_default();
        writeln!(f, "Identity Key: {}", self.identity_key)?;
        writeln!(f, "Sphinx Key: {}", self.sphinx_key)?;
        writeln!(f, "Owner Signature: {}", self.owner_signature)?;
        writeln!(
            f,
            "Host: {} (bind address: {})",
            self.announce_address, self.bind_address
        )?;
        writeln!(f, "Version: {}", self.version)?;
        writeln!(
            f,
            "Mix Port: {}, Verloc port: {}, Http Port: {}\n",
            self.mix_port, self.verloc_port, self.http_api_port
        )?;
        writeln!(f, "You are bonding to wallet address: {wallet_address}\n\n")
    }
}
