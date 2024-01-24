#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod access {
    use ink::storage::Mapping;

    #[derive(scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std",derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub enum AccessLevel { 
        NoAccess, 
        Owner, 
        Admin, 
        Read 
    }

    #[derive(scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std",derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub enum AccessKind { 
        Doc, 
        DocGroup, 
        UserGroup 
    }

    #[derive(scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std",derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub struct Object {
        id_hash: [u8; 32],
        id_encr: [u8; 32],
        key_encr: [u8; 32],
        level: AccessLevel,
    }

    #[ink(storage)]
    pub struct Access {
        access_store: Mapping<[u8; 32], Vec<Object>>,
    }


    impl Access {
        #[ink(constructor)]
        pub fn new() -> Self {
            let access_store = Mapping::default();
            Self { 
                access_store 
            }
        }

        #[ink(message)]
        pub fn get_access_by_hash(&self, user_id_hash: [u8; 32], object_id_hash: [u8; 32]) -> Object {

            let objects = self.access_store.get(&user_id_hash).expect("NFD");

            // objects

            for object in objects {
                if object.id_hash == object_id_hash {
                    return object;
                }
            }
            // ink_env::panic_str("NFD");
        }

    }


    // function getAccessByIdHash(
    //     bytes32 userIdHash, 
    //     bytes32 objectIdHash
    // ) 
    //     external view returns(Object memory) 
    // {
    //     for (uint i; i < accessStore[userIdHash].length; i++){
    //         if (accessStore[userIdHash][i].idHash == objectIdHash) {
    //             return accessStore[userIdHash][i];
    //         }
    //     }

    //     revert("NFD");
    // }

    // function getUserAccessList(bytes32 userIdHash) external view returns (Object[] memory) {
    //     require(accessStore[userIdHash].length > 0, "NFD");
    //     return accessStore[userIdHash];
    // }

    // function getUserAccessLevel(
    //     bytes32 userID,
    //     AccessKind kind,
    //     bytes32 idHash
    // )
    //     internal view returns (AccessLevel) 
    // {
    //     bytes32 accessID = keccak256(abi.encode(userID, kind));
    //     for(uint i; i < accessStore[accessID].length; i++){
    //         if (accessStore[accessID][i].idHash == idHash) {
    //             return accessStore[accessID][i].level;
    //         }
    //     }

    //     // Checking groups
    //     accessID = keccak256(abi.encode(userID, AccessKind.UserGroup));
    //     for (uint i = 0; i < accessStore[accessID].length; i++) {
    //         for (uint j = 0; j < accessStore[accessID].length; j++) {
    //             if (accessStore[accessID][j].idHash == idHash) {
    //                 return accessStore[accessID][j].level;
    //             }
    //         }
    //     }

    //     return AccessLevel.NoAccess;
    // }



}
