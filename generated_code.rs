//! MultiversX Smart Contract for sending 1 EGLD from Alice to Bob

#![no_std]
#![allow(unused_attributes)]

// Imports
use multiversx_sc::*;

// Type definitions
#[derive(TopEncode, TopDecode, PartialEq, TypeAbi)]
pub struct TokenIdentifier(pub Box<[u8]>);

#[derive(TopEncode, TopDecode, PartialEq, TypeAbi)]
pub struct NonFungibleToken {
    pub token_identifier: TokenIdentifier,
    pub token_nonce: Nonce,
}

pub trait MultiversXContract {
    fn get_egld_balance(&self, address: &Address) -> Self::BigUint;
    fn transfer_egld(&self, from: &Address, to: &Address, amount: &BigUint);
    fn mint_token(&self, token: &NonFungibleToken, owner: Address);
}

#[multiversx_sc_module]
pub trait TokenModule: MultiversXContract {
    #[storage_set("balance")]
    fn set_balance(&self, address: &Address, balance: BigNumber<Self::Api>);
    #[storage_get("balance")]
    fn get_balance(&self, address: &Address) -> BigNumber<Self::Api>;
}

impl<MultiversXContract: proxy_obj::ProxyObjCode> TokenModule for MultiversXContract {
    fn mint_token(&self, token: &NonFungibleToken, owner: Address) {
        let amount = self.get_egld_balance(&owner);
        if amount > Self::BigUint::from(0u32) {
            let one_egld: Self::BigUint = Self::BigUint::from(1_000_000_000_000_000_000u64); // 1 EGLD in smaller units

            let alice = owner;
            let bob = Address::from("erd1cux02zersde0l7hhklzhywcxk4u9n4py5tdxyx7vrvhnza2r4gmq4vw35l"); // Bob's Address
            let remaining_balance = &amount - &one_egld;

            self.transfer_egld(&alice, &bob, &one_egld);
            self.set_balance(&alice, remaining_balance);
        }
        self._mint_token(token, owner);
    }
}