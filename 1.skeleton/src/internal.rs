use near_sdk::require;

use crate::*;

impl Contract {
    pub(crate) fn internal_deposit(&mut self, account_id: &AccountId, amount: Balance) {
        let balance = self.internal_unwrap_balance_of(account_id);

        if let Some(new_balance) = balance.checked_add(amount) {
            self.accounts.insert(account_id, &new_balance);
        } else {
            env::panic_str("Balance overflow");
        }
    }

    pub(crate) fn internal_register_account(&mut self, account_id: &AccountId) {
        if self.accounts.insert(account_id, &0).is_some() {
            env::panic_str("Account is already registered");
        }
    }

    pub(crate) fn internal_unwrap_balance_of(&mut self, account_id: &AccountId) -> Balance {
        if let Some(balance) = self.accounts.get(account_id) {
            balance
        } else {
            env::panic_str("Account is not registered");
        }
    }

    pub(crate) fn measure_bytes_for_longest_account_id(&mut self) {
        let initial_storage_usage = env::storage_usage();
        let long_account_id = AccountId::new_unchecked("a".repeat(64));
        self.accounts.insert(&long_account_id, &0u128);
        self.bytes_for_longest_account_id = env::storage_usage() - initial_storage_usage;
        self.accounts.remove(&long_account_id);
    }
}
