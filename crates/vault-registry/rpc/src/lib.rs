//! RPC interface for the Vault Registry.

pub use self::gen_client::Client as VaultRegistryClient;
use codec::Codec;
use jsonrpc_core::{Error as RpcError, ErrorCode, Result as JsonRpcResult};
use jsonrpc_derive::rpc;
use module_exchange_rate_oracle_rpc_runtime_api::BalanceWrapper;
pub use module_vault_registry_rpc_runtime_api::VaultRegistryApi as VaultRegistryRuntimeApi;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{
    generic::BlockId,
    traits::{Block as BlockT, MaybeDisplay, MaybeFromStr},
    DispatchError,
};
use std::sync::Arc;

#[rpc]
pub trait VaultRegistryApi<BlockHash, AccountId, Wrapped, Collateral, UnsignedFixedPoint>
where
    Wrapped: Codec + MaybeDisplay + MaybeFromStr,
    Collateral: Codec + MaybeDisplay + MaybeFromStr,
    UnsignedFixedPoint: Codec + MaybeDisplay + MaybeFromStr,
{
    #[rpc(name = "vaultRegistry_getTotalCollateralization")]
    fn get_total_collateralization(&self, at: Option<BlockHash>) -> JsonRpcResult<UnsignedFixedPoint>;

    #[rpc(name = "vaultRegistry_getFirstVaultWithSufficientCollateral")]
    fn get_first_vault_with_sufficient_collateral(
        &self,
        amount: BalanceWrapper<Wrapped>,
        at: Option<BlockHash>,
    ) -> JsonRpcResult<AccountId>;

    #[rpc(name = "vaultRegistry_getFirstVaultWithSufficientTokens")]
    fn get_first_vault_with_sufficient_tokens(
        &self,
        amount: BalanceWrapper<Wrapped>,
        at: Option<BlockHash>,
    ) -> JsonRpcResult<AccountId>;

    #[rpc(name = "vaultRegistry_getPremiumRedeemVaults")]
    fn get_premium_redeem_vaults(
        &self,
        at: Option<BlockHash>,
    ) -> JsonRpcResult<Vec<(AccountId, BalanceWrapper<Wrapped>)>>;

    #[rpc(name = "vaultRegistry_getVaultsWithIssuableTokens")]
    fn get_vaults_with_issuable_tokens(
        &self,
        at: Option<BlockHash>,
    ) -> JsonRpcResult<Vec<(AccountId, BalanceWrapper<Wrapped>)>>;

    #[rpc(name = "vaultRegistry_getVaultsWithRedeemableTokens")]
    fn get_vaults_with_redeemable_tokens(
        &self,
        at: Option<BlockHash>,
    ) -> JsonRpcResult<Vec<(AccountId, BalanceWrapper<Wrapped>)>>;

    #[rpc(name = "vaultRegistry_getIssueableTokensFromVault")]
    fn get_issuable_tokens_from_vault(
        &self,
        vault: AccountId,
        at: Option<BlockHash>,
    ) -> JsonRpcResult<BalanceWrapper<Wrapped>>;

    #[rpc(name = "vaultRegistry_getCollateralizationFromVault")]
    fn get_collateralization_from_vault(
        &self,
        vault: AccountId,
        only_issued: bool,
        at: Option<BlockHash>,
    ) -> JsonRpcResult<UnsignedFixedPoint>;

    #[rpc(name = "vaultRegistry_getCollateralizationFromVaultAndCollateral")]
    fn get_collateralization_from_vault_and_collateral(
        &self,
        vault: AccountId,
        collateral: BalanceWrapper<Collateral>,
        only_issued: bool,
        at: Option<BlockHash>,
    ) -> JsonRpcResult<UnsignedFixedPoint>;

    #[rpc(name = "vaultRegistry_getRequiredCollateralForWrapped")]
    fn get_required_collateral_for_wrapped(
        &self,
        amount_btc: BalanceWrapper<Wrapped>,
        at: Option<BlockHash>,
    ) -> JsonRpcResult<BalanceWrapper<Collateral>>;

    #[rpc(name = "vaultRegistry_getRequiredCollateralForVault")]
    fn get_required_collateral_for_vault(
        &self,
        vault_id: AccountId,
        at: Option<BlockHash>,
    ) -> JsonRpcResult<BalanceWrapper<Collateral>>;
}

/// A struct that implements the [`VaultRegistryApi`].
pub struct VaultRegistry<C, B> {
    client: Arc<C>,
    _marker: std::marker::PhantomData<B>,
}

impl<C, B> VaultRegistry<C, B> {
    /// Create new `VaultRegistry` with the given reference to the client.
    pub fn new(client: Arc<C>) -> Self {
        VaultRegistry {
            client,
            _marker: Default::default(),
        }
    }
}

pub enum Error {
    RuntimeError,
}

impl From<Error> for i64 {
    fn from(e: Error) -> i64 {
        match e {
            Error::RuntimeError => 1,
        }
    }
}

fn handle_response<T, E: std::fmt::Debug>(
    result: Result<Result<T, DispatchError>, E>,
    msg: String,
) -> JsonRpcResult<T> {
    result.map_or_else(
        |e| {
            Err(RpcError {
                code: ErrorCode::ServerError(Error::RuntimeError.into()),
                message: msg.clone(),
                data: Some(format!("{:?}", e).into()),
            })
        },
        |result| {
            result.map_err(|e| RpcError {
                code: ErrorCode::ServerError(Error::RuntimeError.into()),
                message: msg.clone(),
                data: Some(format!("{:?}", e).into()),
            })
        },
    )
}

impl<C, Block, AccountId, Wrapped, Collateral, UnsignedFixedPoint>
    VaultRegistryApi<<Block as BlockT>::Hash, AccountId, Wrapped, Collateral, UnsignedFixedPoint>
    for VaultRegistry<C, Block>
where
    Block: BlockT,
    C: Send + Sync + 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
    C::Api: VaultRegistryRuntimeApi<Block, AccountId, Wrapped, Collateral, UnsignedFixedPoint>,
    AccountId: Codec,
    Wrapped: Codec + MaybeDisplay + MaybeFromStr,
    Collateral: Codec + MaybeDisplay + MaybeFromStr,
    UnsignedFixedPoint: Codec + MaybeDisplay + MaybeFromStr,
{
    fn get_total_collateralization(&self, at: Option<<Block as BlockT>::Hash>) -> JsonRpcResult<UnsignedFixedPoint> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

        handle_response(
            api.get_total_collateralization(&at),
            "Unable to get total collateralization.".into(),
        )
    }

    fn get_first_vault_with_sufficient_collateral(
        &self,
        amount: BalanceWrapper<Wrapped>,
        at: Option<<Block as BlockT>::Hash>,
    ) -> JsonRpcResult<AccountId> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

        handle_response(
            api.get_first_vault_with_sufficient_collateral(&at, amount),
            "Unable to find a vault with sufficient collateral.".into(),
        )
    }

    fn get_first_vault_with_sufficient_tokens(
        &self,
        amount: BalanceWrapper<Wrapped>,
        at: Option<<Block as BlockT>::Hash>,
    ) -> JsonRpcResult<AccountId> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

        handle_response(
            api.get_first_vault_with_sufficient_tokens(&at, amount),
            "Unable to find a vault with sufficient tokens.".into(),
        )
    }

    fn get_premium_redeem_vaults(
        &self,
        at: Option<<Block as BlockT>::Hash>,
    ) -> JsonRpcResult<Vec<(AccountId, BalanceWrapper<Wrapped>)>> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

        handle_response(
            api.get_premium_redeem_vaults(&at),
            "Unable to find a vault below the premium redeem threshold.".into(),
        )
    }

    fn get_vaults_with_issuable_tokens(
        &self,
        at: Option<<Block as BlockT>::Hash>,
    ) -> JsonRpcResult<Vec<(AccountId, BalanceWrapper<Wrapped>)>> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

        handle_response(
            api.get_vaults_with_issuable_tokens(&at),
            "Unable to find a vault with issuable tokens.".into(),
        )
    }

    fn get_vaults_with_redeemable_tokens(
        &self,
        at: Option<<Block as BlockT>::Hash>,
    ) -> JsonRpcResult<Vec<(AccountId, BalanceWrapper<Wrapped>)>> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

        handle_response(
            api.get_vaults_with_redeemable_tokens(&at),
            "Unable to find a vault with redeemable tokens.".into(),
        )
    }

    fn get_issuable_tokens_from_vault(
        &self,
        vault: AccountId,
        at: Option<<Block as BlockT>::Hash>,
    ) -> JsonRpcResult<BalanceWrapper<Wrapped>> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

        handle_response(
            api.get_issuable_tokens_from_vault(&at, vault),
            "Unable to get issuable tokens from vault.".into(),
        )
    }

    fn get_collateralization_from_vault(
        &self,
        vault: AccountId,
        only_issued: bool,
        at: Option<<Block as BlockT>::Hash>,
    ) -> JsonRpcResult<UnsignedFixedPoint> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

        handle_response(
            api.get_collateralization_from_vault(&at, vault, only_issued),
            "Unable to get collateralization from vault.".into(),
        )
    }

    fn get_collateralization_from_vault_and_collateral(
        &self,
        vault: AccountId,
        collateral: BalanceWrapper<Collateral>,
        only_issued: bool,
        at: Option<<Block as BlockT>::Hash>,
    ) -> JsonRpcResult<UnsignedFixedPoint> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

        handle_response(
            api.get_collateralization_from_vault_and_collateral(&at, vault, collateral, only_issued),
            "Unable to get collateralization from vault.".into(),
        )
    }

    fn get_required_collateral_for_wrapped(
        &self,
        amount_btc: BalanceWrapper<Wrapped>,
        at: Option<<Block as BlockT>::Hash>,
    ) -> JsonRpcResult<BalanceWrapper<Collateral>> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

        handle_response(
            api.get_required_collateral_for_wrapped(&at, amount_btc),
            "Unable to get required collateral for amount.".into(),
        )
    }

    fn get_required_collateral_for_vault(
        &self,
        vault_id: AccountId,
        at: Option<<Block as BlockT>::Hash>,
    ) -> JsonRpcResult<BalanceWrapper<Collateral>> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));
        api.get_required_collateral_for_vault(&at, vault_id).map_or_else(
            |e| {
                Err(RpcError {
                    code: ErrorCode::ServerError(Error::RuntimeError.into()),
                    message: "Unable to get required collateral for vault.".into(),
                    data: Some(format!("{:?}", e).into()),
                })
            },
            |result| {
                result.map_err(|e| RpcError {
                    code: ErrorCode::ServerError(Error::RuntimeError.into()),
                    message: "Unable to get required collateral for vault.".into(),
                    data: Some(format!("{:?}", e).into()),
                })
            },
        )
    }
}
