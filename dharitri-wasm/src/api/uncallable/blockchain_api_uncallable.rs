use alloc::boxed::Box;

use crate::{
    api::{BlockchainApi, BlockchainApiImpl, Handle, ManagedTypeApi},
    types::{
        heap::{Address, H256},
        BigUint, DctTokenData, ManagedAddress, TokenIdentifier,
    },
};

use super::UncallableApi;

impl BlockchainApi for UncallableApi {
    type BlockchainApiImpl = UncallableApi;

    fn blockchain_api_impl() -> Self::BlockchainApiImpl {
        unreachable!()
    }
}

impl BlockchainApiImpl for UncallableApi {
    fn get_sc_address_legacy(&self) -> Address {
        unreachable!()
    }

    fn get_owner_address_legacy(&self) -> Address {
        unreachable!()
    }

    fn get_shard_of_address_legacy(&self, _address: &Address) -> u32 {
        unreachable!()
    }

    fn is_smart_contract_legacy(&self, _address: &Address) -> bool {
        unreachable!()
    }

    fn get_caller_legacy(&self) -> Address {
        unreachable!()
    }

    fn get_balance_legacy(&self, _address: &Address) -> Handle {
        unreachable!()
    }

    fn get_state_root_hash_legacy(&self) -> H256 {
        unreachable!()
    }

    fn get_tx_hash_legacy(&self) -> H256 {
        unreachable!()
    }

    fn get_gas_left(&self) -> u64 {
        unreachable!()
    }

    fn get_block_timestamp(&self) -> u64 {
        unreachable!()
    }

    fn get_block_nonce(&self) -> u64 {
        unreachable!()
    }

    fn get_block_round(&self) -> u64 {
        unreachable!()
    }

    fn get_block_epoch(&self) -> u64 {
        unreachable!()
    }

    fn get_block_random_seed_legacy(&self) -> Box<[u8; 48]> {
        unreachable!()
    }

    fn get_prev_block_timestamp(&self) -> u64 {
        unreachable!()
    }

    fn get_prev_block_nonce(&self) -> u64 {
        unreachable!()
    }

    fn get_prev_block_round(&self) -> u64 {
        unreachable!()
    }

    fn get_prev_block_epoch(&self) -> u64 {
        unreachable!()
    }

    fn get_prev_block_random_seed_legacy(&self) -> Box<[u8; 48]> {
        unreachable!()
    }

    fn get_current_dct_nft_nonce<M: ManagedTypeApi>(
        &self,
        _address: &ManagedAddress<M>,
        _token: &TokenIdentifier<M>,
    ) -> u64 {
        unreachable!()
    }

    fn get_dct_balance<M: ManagedTypeApi>(
        &self,
        _address: &ManagedAddress<M>,
        _token: &TokenIdentifier<M>,
        _nonce: u64,
    ) -> BigUint<M> {
        unreachable!()
    }

    fn get_dct_token_data<M: ManagedTypeApi>(
        &self,
        _address: &ManagedAddress<M>,
        _token: &TokenIdentifier<M>,
        _nonce: u64,
    ) -> DctTokenData<M> {
        unreachable!()
    }

    fn get_dct_local_roles<M: ManagedTypeApi>(
        &self,
        _token_id: &TokenIdentifier<M>,
    ) -> crate::types::DctLocalRoleFlags {
        unreachable!()
    }
}
