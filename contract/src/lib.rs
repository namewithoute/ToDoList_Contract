/*
 * Example smart contract written in RUST
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://near-docs.io/develop/Contract
 *
 */

use std::vec;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, near_bindgen};

// Define the default message
// const DEFAULT_MESSAGE: &str = "Hello";

// Define the contract structure
// #[near_bindgen]
// #[derive(BorshDeserialize, BorshSerialize)]
// pub struct Contract {
//     message: String,
// }

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct ToDoList {
    list: Vec<OwnerTask>,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Clone)]
pub struct OwnerTask {
    name: String,
    task: Vec<String>,
}


impl OwnerTask {
    pub fn get_name(&self) -> String {
        return self.name.clone();
    }
    pub fn get_task(&self) -> Vec<String> {
        return self.task.clone();
    }
}

// Define the default, which automatically initializes the contract
impl Default for ToDoList {
    fn default() -> Self {
        Self { list: Vec::new() }
    }
}

// Implement the contract structure
#[near_bindgen]
impl ToDoList {
    #[result_serializer(borsh)]

    // Public method - returns the greeting saved, defaulting to DEFAULT_MESSAGE
    pub fn get_list(&self, name: String) -> Option<OwnerTask> {
        for i in self.list.clone() {
            if i.get_name() == name {
                return Some(i)
            }
        };
        None
        // for i in self.get_all(){
        //     if i.GetName()==name{
        //         a = i;
        //         return a
        //     }
        // }
        // return None;
    }

    // Public method - accepts a greeting, such as "howdy", and records it
    pub fn add_task(&mut self, name: String, task: String) {
        // Use env::log to record logs permanently to the blockchain!
        log!("Saving task {} of {}", task, name);
        // let newOwner = OwnerTask { name:name.to_string(), task: task.to_string()};
    
        
        let mut count =0;
        for i in self.list.clone(){
            if i.get_name() == name{
                i.get_task().push(task.to_string());
                count +=1;
                break;
            }
        }
        if count==0{
            self.list.push( OwnerTask{name:name,task : vec![task.to_string()] })
        }

        // self.list.push(newOwner);
    }
}
// #[near_bindgen]

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
//  */
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn get_default_greeting() {
//         let contract = Contract::default();
//         // this test did not call set_greeting so should return the default "Hello" greeting
//         assert_eq!(
//             contract.get_greeting(),
//             "Hello".to_string()
//         );
//     }

//     #[test]
//     fn set_then_get_greeting() {
//         let mut contract = Contract::default();
//         contract.set_greeting("howdy".to_string());
//         assert_eq!(
//             contract.get_greeting(),
//             "howdy".to_string()
//         );
//     }
// }
