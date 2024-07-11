use num::traits::{One, Zero};
use std::{collections::BTreeMap, ops::AddAssign};

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
#[derive(Debug)]
pub struct Pallet<AccountID, BlockNumber, NonceNumber> {
    block_number: BlockNumber,
    nonce: BTreeMap<AccountID, NonceNumber>,
}

impl<
        AccountID: Ord + Clone,
        BlockNumber: Zero + One + AddAssign + Copy,
        NonceNumber: Zero + One + AddAssign + Copy,
    > Pallet<AccountID, BlockNumber, NonceNumber>
{
    /// Create a new instance of the System Pallet.
    pub fn new() -> Self {
        Self { block_number: BlockNumber::zero(), nonce: BTreeMap::new() }
    }

    /// Get the current block number.
    pub fn block_number(&self) -> BlockNumber {
        self.block_number
    }

    // This function can be used to increment the block number.
    // Increases the block number by one.
    pub fn inc_block_number(&mut self) {
        self.block_number += BlockNumber::one();
    }

    // Increment the nonce of an account. This helps us keep track of how many transactions each
    // account has made.
    pub fn inc_nonce(&mut self, who: &AccountID) {
        let mut user_nonce = *self.nonce.get(who).unwrap_or(&NonceNumber::zero());
        user_nonce += NonceNumber::one();
        self.nonce.insert(who.clone(), user_nonce);
    }
}

#[test]
fn init_system() {
    type AccountID = String;
    type BlockNumber = u32;
    type NonceNumber = u32;

    let mut s: self::Pallet<AccountID, BlockNumber, NonceNumber> = Pallet::new();

    s.inc_block_number();
    assert_eq!(s.block_number(), 1);

    s.inc_nonce(&"wasi".to_string());
    assert_eq!(s.nonce.get(&"wasi".to_string()), Some(1).as_ref());
}
