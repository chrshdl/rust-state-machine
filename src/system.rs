use std::collections::BTreeMap;

type AccountID = String;
type BlockNumber = u32;
type NonceNumber = u32;

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
#[derive(Debug)]
pub struct Pallet {
    block_number: BlockNumber,
    nonce: BTreeMap<AccountID, NonceNumber>,
}

impl Pallet {
    /// Create a new instance of the System Pallet.
    pub fn new() -> Self {
        Self { block_number: 0, nonce: BTreeMap::new() }
    }

    /// Get the current block number.
    pub fn block_number(&self) -> BlockNumber {
        self.block_number
    }

    // This function can be used to increment the block number.
    // Increases the block number by one.
    pub fn inc_block_number(&mut self) {
        self.block_number += 1;
    }

    // Increment the nonce of an account. This helps us keep track of how many transactions each
    // account has made.
    pub fn inc_nonce(&mut self, who: &AccountID) {
        let user_nonce = *self.nonce.get(who).unwrap_or(&0);
        self.nonce.insert(who.to_string(), user_nonce + 1);
    }
}

#[test]
fn init_system() {
    let mut s = Pallet::new();

    s.inc_block_number();
    assert_eq!(s.block_number(), 1);

    s.inc_nonce(&"wasi".to_string());
    assert_eq!(s.nonce.get(&"wasi".to_string()), Some(1).as_ref());
}
