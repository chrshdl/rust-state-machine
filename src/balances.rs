use std::collections::BTreeMap;

pub struct Pallet {
    balances: BTreeMap<String, u128>,
}

impl Pallet {
    pub fn new() -> Self {
        Self { balances: BTreeMap::new() }
    }

	/// Set the balance of an account `who` to some `amount`.
    pub fn set_balance(&mut self, who: &String, amount: u128) {
        self.balances.insert(who.to_string(), amount);
    }

    /// Get the balance of an account `who`.
    /// If the account has no stored balance, we return zero.
    pub fn balance(&self, who: &String) -> u128 {
        *self.balances.get(who).unwrap_or(&0)
    }
}

#[test]
fn init_balances() {
    let mut p = Pallet::new();

    assert_eq!(p.balance(&String::from("wasi")), 0);

    p.set_balance(&String::from("wasi"), 1);

    assert_eq!(p.balance(&String::from("wasi")), 1);
}
