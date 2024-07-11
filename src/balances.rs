use num::traits::{CheckedAdd, CheckedSub, Zero};
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Pallet<AccountID, Balance> {
    balances: BTreeMap<AccountID, Balance>,
}

impl<AccountID: Clone + Ord, Balance: Ord + Zero + CheckedSub + CheckedAdd + Copy>
    Pallet<AccountID, Balance>
{
    pub fn new() -> Self {
        Self { balances: BTreeMap::new() }
    }

    /// Set the balance of an account `who` to some `amount`.
    pub fn set_balance(&mut self, who: &AccountID, amount: Balance) {
        self.balances.insert(who.clone(), amount);
    }

    /// Get the balance of an account `who`.
    /// If the account has no stored balance, we return zero.
    pub fn balance(&self, who: &AccountID) -> Balance {
        *self.balances.get(who).unwrap_or(&Balance::zero())
    }

    /// Transfer `amount` from one account to another.
    /// This function verifies that `from` has at least `amount` balance to transfer,
    /// and that no mathematical overflows occur.
    pub fn transfer(
        &mut self,
        from: &AccountID,
        to: &AccountID,
        amount: Balance,
    ) -> Result<(), &'static str> {
        let balance_sender = self.balance(from);
        let new_balance_sender = balance_sender.checked_sub(&amount).ok_or("Unsufficient balance");
        self.set_balance(from, new_balance_sender?);

        let balance_receiver = self.balance(to);
        let new_balance_receiver = balance_receiver.checked_add(&amount).ok_or("Overflow");
        self.set_balance(to, new_balance_receiver?);

        Ok(())
    }
}

#[test]
fn init_balances() {
    let mut p = Pallet::new();

    assert_eq!(p.balance(&"wasi".to_string()), 0);

    p.set_balance(&"wasi".to_string(), 1);

    assert_eq!(p.balance(&"wasi".to_string()), 1);
}

#[test]
fn transfer_balance() {
    let mut p = Pallet::new();
    assert_eq!(
        p.transfer(&"wasi".to_string(), &"alice".to_string(), 1),
        Err("Unsufficient balance")
    );
    assert_eq!(p.balance(&"wasi".to_string()), 0);
    assert_eq!(p.balance(&"alice".to_string()), 0);

    p.set_balance(&"wasi".to_string(), 1);
    assert_eq!(p.transfer(&"wasi".to_string(), &"alice".to_string(), 1), Ok(()));
    assert_eq!(p.balance(&"wasi".to_string()), 0);
    assert_eq!(p.balance(&"alice".to_string()), 1);

    p.set_balance(&"wasi".to_string(), 1);
    p.set_balance(&"alice".to_string(), u128::max_value());
    assert_eq!(p.transfer(&"wasi".to_string(), &"alice".to_string(), 1), Err("Overflow"));
}
