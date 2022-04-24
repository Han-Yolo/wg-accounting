use crate::account::Account;
use crate::transaction::Transaction;

use std::ops;

pub struct BalanceEntry {
    sender_index: usize,
    recipient_index: usize,
    balance: f64,
}
impl BalanceEntry {
    pub fn new(
        sender_index_param: usize,
        recipient_index_param: usize,
        balance_param: f64,
    ) -> Self {
        assert_ne!(sender_index_param, recipient_index_param);
        BalanceEntry {
            sender_index: sender_index_param,
            recipient_index: recipient_index_param,
            balance: balance_param,
        }
    }
    pub fn sender_index(&self) -> usize {
        self.sender_index
    }
    pub fn recipient_index(&self) -> usize {
        self.recipient_index
    }
    pub fn balance(&self) -> f64 {
        self.balance
    }
}
impl ops::AddAssign<f64> for BalanceEntry {
    fn add_assign(&mut self, amount: f64) {
        self.balance += amount;
    }
}
impl ops::SubAssign<f64> for BalanceEntry {
    fn sub_assign(&mut self, amount: f64) {
        self.balance -= amount;
    }
}

pub struct Balance {
    entries: Vec<BalanceEntry>,
}
impl Balance {
    pub fn new() -> Self {
        Balance {
            entries: Vec::new(),
        }
    }
    pub fn add_invoice(&mut self, invoice: &Transaction) {
        self.add_transaction(
            invoice.sender_index(),
            invoice.recipient_index(),
            -invoice.amount(),
        );
    }
    pub fn add_payment(&mut self, payment: &Transaction) {
        self.add_transaction(
            payment.sender_index(),
            payment.recipient_index(),
            payment.amount(),
        );
    }
    pub fn to_string(&self, accounts: &Vec<Account>) -> String {
        let mut string = "Balance:\n".to_owned();
        for entry in &self.entries {
            string += format!(
                "{} -> {} {} CHF\n",
                accounts[entry.sender_index].acronym(),
                accounts[entry.recipient_index].acronym(),
                entry.balance
            )
            .as_str();
        }
        string
    }
    pub fn entries(&self) -> &Vec<BalanceEntry> {
        &self.entries
    }
    fn add_transaction(&mut self, sender_index: usize, recipient_index: usize, amount: f64) {
        if let Some(entry) = self.entries.iter_mut().find(|entry| {
            (entry.sender_index == sender_index) && (entry.recipient_index == recipient_index)
        }) {
            *entry += amount;
        } else if let Some(entry) = self.entries.iter_mut().find(|entry| {
            (entry.sender_index == recipient_index) && (entry.recipient_index == sender_index)
        }) {
            *entry -= amount;
        } else {
            self.entries
                .push(BalanceEntry::new(sender_index, recipient_index, amount));
        }
    }
}
