#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod restrictable {
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct Restrictable {
        allowed_change: Mapping<AccountId, bool>,
        current_nonces: Mapping<AccountId, u128>,
    }

    impl Restrictable {
        #[ink(constructor)]
        pub fn new() -> Self {
            let allowed_change = Mapping::default();
            let current_nonces = Mapping::default();

            Self { 
                allowed_change,
                current_nonces
            }
        }

        #[ink(message)]
        pub fn only_allowed(&self, addr: AccountId) {
            assert!(self.allowed_change.get(addr) == Some(true), "Not allowed")
        }

        #[ink(message)]
        pub fn check_nonce(&mut self, addr: AccountId, nonce: u128) {
            let current = self.current_nonces.get(&addr);
            let increment = self.add_to_option(current, 1);
            let res = self.get_value_or_default(increment);
            assert!(self.current_nonces.get(addr) == Some(nonce - 1), "NON");
            self.current_nonces.insert(addr, &res);
        }

        fn add_to_option(&self, value: Option<u128>, addend: u128) -> Option<u128> {
            match value {
                Some(number) => Some(number + addend),
                None => None,
            }
        }

        fn get_value_or_default(&self, option: Option<u128>) -> u128 {
            match option {
                Some(value) => value,
                None => 0, // default value if None
            }
        }
        
        #[ink(message)]
        pub fn set_allowed(&mut self, addr: AccountId, allowed: bool) {
            self.allowed_change.insert(addr, &allowed);
        }
    }
}