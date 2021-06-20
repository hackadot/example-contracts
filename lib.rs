#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

use ink_prelude::*;


#[ink::contract]
mod swap {
    use ink_storage::Lazy;
    use erc20::stub::Erc20;
    use brush::traits::InkStorage;

    #[ink(storage)]
    #[derive(Default)]
    pub struct Swap {
        token_addr: AccountId,
        exchange_rate: Lazy<u8>,
    }

    impl InkStorage for Swap {}

    impl Swap {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(token_addr: AccountId, exchange_rate: u8) -> Self {
            Self { token_addr, exchange_rate: Lazy::new(exchange_rate) }
        }

        #[ink(message, payable)]
        pub fn buy(&mut self) {
            let caller = self.env().caller();
            let value = self.env().transferred_balance();
            assert!(value > 0, "msg.value cannot be zero");
            let mut my_erc20: Erc20 = ink_env::call::FromAccountId::from_account_id(self.token_addr);
            let hackt_balance = my_erc20.balance_of(self.env().account_id());
            let transaction_value = value * (*self.exchange_rate as u128);
            assert!(hackt_balance >= transaction_value, "Insufficient balance on contract, transaction_value is {}, current balance is {}",
            transaction_value, hackt_balance);
            my_erc20.transfer(caller, transaction_value);

        }

        pub fn sell(&mut self, amount: Balance) {
            let caller = self.env().caller();
            assert!(amount > 0, "Amount cannot be zero");
            let mut this_balance = self.env().balance();
            let dot_amount = amount / (*self.exchange_rate as u128);
            assert!(this_balance >= dot_amount);
            let mut my_erc20: Erc20 = ink_env::call::FromAccountId::from_account_id(self.token_addr);
            assert!(amount <= my_erc20.allowance(caller, self.env().account_id()));
            my_erc20.transfer_from(caller, self.env().account_id(), amount);
            self.env().transfer(caller, dot_amount);
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn exchange_rate(&self) -> u8 {
            *self.exchange_rate
        }
    }
}