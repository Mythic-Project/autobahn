use std::collections::HashSet;

use crate::account_write::AccountWrite;
use solana_client::rpc_config::RpcProgramAccountsConfig;
use solana_sdk::account::Account;
use solana_sdk::pubkey::Pubkey;

#[async_trait::async_trait]
pub trait RouterRpcClientTrait: Sync + Send {
    async fn get_account(&mut self, pubkey: &Pubkey) -> anyhow::Result<Option<Account>>;

    async fn get_multiple_accounts(
        &mut self,
        pubkeys: &HashSet<Pubkey>,
    ) -> anyhow::Result<Vec<(Pubkey, Account)>>;

    async fn get_program_accounts_with_config(
        &mut self,
        pubkey: &Pubkey,
        config: RpcProgramAccountsConfig,
    ) -> anyhow::Result<Vec<AccountWrite>>;

    fn is_gpa_compression_enabled(&self) -> bool;
}

pub struct RouterRpcClient {
    pub rpc: Box<dyn RouterRpcClientTrait + Send + Sync + 'static>,
    pub gpa_compression_enabled: bool,
}

#[async_trait::async_trait]
impl RouterRpcClientTrait for RouterRpcClient {
    async fn get_account(&mut self, pubkey: &Pubkey) -> anyhow::Result<Option<Account>> {
        self.rpc.get_account(pubkey).await
    }

    async fn get_multiple_accounts(
        &mut self,
        pubkeys: &HashSet<Pubkey>,
    ) -> anyhow::Result<Vec<(Pubkey, Account)>> {
        self.rpc.get_multiple_accounts(pubkeys).await
    }

    async fn get_program_accounts_with_config(
        &mut self,
        pubkey: &Pubkey,
        config: RpcProgramAccountsConfig,
    ) -> anyhow::Result<Vec<AccountWrite>> {
        self.rpc
            .get_program_accounts_with_config(pubkey, config)
            .await
    }

    fn is_gpa_compression_enabled(&self) -> bool {
        self.gpa_compression_enabled
    }
}
