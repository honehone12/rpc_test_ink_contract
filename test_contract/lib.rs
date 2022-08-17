#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod test_contract {
    pub type Result<T> = core::result::Result<T, Error>;
    pub type Vec<T> = ink_prelude::vec::Vec<T>;
    pub type String = ink_prelude::string::String;
    
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        TestError
    }

    #[derive(
        Debug, Eq, PartialEq, scale::Encode, scale::Decode, Clone,
        ink_storage::traits::PackedLayout, ink_storage::traits::SpreadLayout
    )]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum CustomOption {
        None,
        Some(Vec<u64>)
    }

    #[ink(storage)]
    #[derive(Default, ink_storage::traits::SpreadAllocate)]
    pub struct TestContract {
        int_storage: u64,
        map_storage: ink_storage::Mapping<u64, Vec<u64>>
    }

    impl TestContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::utils::initialize_contract(|cnt: &mut Self| {
                cnt.int_storage = 1;
            })
        }

        #[ink(message)]
        pub fn ping(&self) -> String {
            String::from("pong")
        }

        #[ink(message)]
        pub fn echo(&self, what: String) -> String {
            what
        }

        #[ink(message)]
        pub fn echo_int(&self, what: u64) -> u64 {
            what
        }

        #[ink(message)]
        pub fn get_int_storage(&self) -> u64 {
            self.int_storage
        }

        #[ink(message)]
        pub fn add_storage(&mut self, add: u64) -> u64 {
            self.int_storage.checked_add(add).unwrap_or(0)
        }

        #[ink(message)]
        pub fn add_storage_result(&mut self, add: u64) -> Result<u64> {
            self.int_storage.checked_add(add).ok_or(Error::TestError)
        }

        #[ink(message)]
        pub fn add_storage_option(&mut self, add: Option<u64>) -> u64 {
            match add {
                None => self.int_storage,
                Some(n) => {
                    self.add_storage(n)
                }
            }
        }

        #[ink(message)]
        pub fn get_map_storage(&self, key: u64) -> Vec<u64> {
            self.map_storage.get(key).unwrap_or(ink_prelude::vec![])
        }

        #[ink(message)]
        pub fn get_map_storage_result(&self, key: u64) -> Result<Vec<u64>> {
            self.map_storage.get(key).ok_or(Error::TestError)
        }

        #[ink(message)]
        pub fn get_map_storage_option(&self, key: u64) -> Option<Vec<u64>> {
            self.map_storage.get(key)
        }

        #[ink(message)]
        pub fn swap_map_storage(&mut self, key: u64, vec: CustomOption) -> CustomOption {
            match vec {
                CustomOption::None => {
                    let v: Vec<u64> = ink_prelude::vec![];
                    self.map_storage.insert(key, &v);
                },
                CustomOption::Some(ref v) => {
                    self.map_storage.insert(key, v);
                }
            }
            vec
        }
    }
}