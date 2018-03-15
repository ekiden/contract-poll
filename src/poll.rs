// Inspired by https://www.ethereum.org/token

use std::collections::HashMap;

use libcontract_common::{Address, Contract, ContractError};

use token_api::TokenState; // TODO: what does this do?

pub struct PollContract {
    total: u64
}

impl PollContract {
    pub fn new(
    ) -> PollContract {
        let value = 0;

        PollContract {
            total: value,
        }
    }

    fn get_total(&self) -> Result<u32> {
        return Ok(self.total);
    }
}


// Public Methods 


pub fn get_total(&self) -> Result<u32> {
    Ok(self.total);
}
