#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod ownable {

    #[ink(event)]
    pub struct OwnershipTransferred {
        previous_owner: Option<AccountId>,
        new_owner: Option<AccountId>,
    }

    #[ink(storage)]
    pub struct Ownable {
        owner: AccountId,
    }

    impl Ownable {
        #[ink(constructor)]
        pub fn new(owner: AccountId) -> Self {

            Self { 
                owner
            }
        }

        #[ink(message)]
        pub fn owner(&self) -> AccountId {
            self.owner
        }
    
        fn _check_owner(&self) {
            assert!(self.env().caller() == self.owner, "Ownable: caller is not the owner")
        }

        #[ink(message)]
        pub fn renounce_ownership(&mut self) {
            self._check_owner();
            let zero_addr = AccountId::from([0u8; 32]);
            self.transfer_ownership(zero_addr);
        }

        #[ink(message)]
        pub fn transfer_ownership(&mut self, new_owner: AccountId) {
            self._check_owner();
            let old_owner: AccountId = self.owner;
            self.owner = new_owner;

            Self::env().emit_event(OwnershipTransferred {
                    previous_owner: Some(old_owner),
                    new_owner: Some(new_owner),
                }
            )
        }
    }
}
