use std::collections::BTreeMap;

#[derive(Debug)]
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

    /// Transfer `amount` from one account to another.
    /// This function verifies that `from` has at least `amount` balance to transfer,
    /// and that no mathematical overflows occur.
    pub fn transfer(
        &mut self,
        from: &String,
        to: &String,
        amount: u128,
    ) -> Result<(), &'static str> {
        let balance_sender = self.balance(from);
        let new_balance_sender = balance_sender.checked_sub(amount).ok_or("Unsufficient balance");
        self.set_balance(from, new_balance_sender?);

        let balance_receiver = self.balance(to);
        let new_balance_receiver = balance_receiver.checked_add(amount).ok_or("Overflow");
        self.set_balance(to, new_balance_receiver?);

        Ok(())
    }
}

#[test]
fn init_balances() {
    let mut p = Pallet::new();

    assert_eq!(p.balance(&String::from("wasi")), 0);

    p.set_balance(&String::from("wasi"), 1);

    assert_eq!(p.balance(&String::from("wasi")), 1);
}

#[test]
fn transfer_balance() {
    let mut p = Pallet::new();
    assert_eq!(
        p.transfer(&String::from("wasi"), &String::from("alice"), 1),
        Err("Unsufficient balance")
    );
    assert_eq!(p.balance(&String::from("wasi")), 0);
    assert_eq!(p.balance(&String::from("alice")), 0);

    p.set_balance(&String::from("wasi"), 1);
    assert_eq!(p.transfer(&String::from("wasi"), &String::from("alice"), 1), Ok(()));
    assert_eq!(p.balance(&String::from("wasi")), 0);
    assert_eq!(p.balance(&String::from("alice")), 1);

    p.set_balance(&String::from("wasi"), 1);
    p.set_balance(&String::from("alice"), u128::max_value());
    assert_eq!(p.transfer(&String::from("wasi"), &String::from("alice"), 1), Err("Overflow"));
}
