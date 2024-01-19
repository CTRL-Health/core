#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod users {
    use ink::storage::Mapping;

    #[derive(scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std",derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub struct User {
        id: u128,
        system_id: u128,
        role: Role,
        pwd_hash: String,
    }

    #[derive(scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std",derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub enum Role {
        Patient,
        Doctor
    }

    #[derive(scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std",derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub struct User_group {
        params: Mapping<u128, u128>,
        members: Mapping<AccountId, AccessLevel>, //Implement this
        members_count: u128,
    }

    #[ink(storage)]
    pub struct Users {
        version: u128,
        users: Mapping<AccountId, User>,
    }

    impl Users {
        #[ink(constructor)]
        pub fn new(version: u128) -> Self {
            let users = Mapping::default();

            Self { 
                version,
                users
            }
        }

        #[ink(message)]
        pub fn flip(&mut self) {

        }

    }
    
}
