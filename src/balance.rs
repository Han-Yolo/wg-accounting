use crate::account::Account;
use crate::transaction::Transaction;

use std::ops;

struct Entry {
    sender_index: usize,
    recipient_index: usize,
    balance: f64,
}
impl Entry {
    pub fn new(
        sender_index_param: usize,
        recipient_index_param: usize,
        balance_param: f64,
    ) -> Self {
        assert_ne!(sender_index_param, recipient_index_param);
        Self {
            sender_index: sender_index_param,
            recipient_index: recipient_index_param,
            balance: balance_param,
        }
    }
}
impl ops::AddAssign<f64> for Entry {
    fn add_assign(&mut self, amount: f64) {
        self.balance += amount;
    }
}
impl ops::SubAssign<f64> for Entry {
    fn sub_assign(&mut self, amount: f64) {
        self.balance -= amount;
    }
}

pub struct Balance {
    balances: Vec<Entry>,
}
impl Balance {
    pub fn new() -> Self {
        Balance {
            balances: Vec::new(),
        }
    }
    pub fn add_invoice(&mut self, invoice: &Transaction) {
        if let Some(entry) = self.find_entry(invoice) {
            *entry -= invoice.amount();
        } else {
            self.balances.push(Entry::new(
                invoice.sender_index(),
                invoice.recipient_index(),
                -invoice.amount(),
            ));
        }
    }
    pub fn add_payment(&mut self, payment: &Transaction) {
        if let Some(entry) = self.find_entry(payment) {
            *entry += payment.amount();
        } else {
            self.balances.push(Entry::new(
                payment.sender_index(),
                payment.recipient_index(),
                payment.amount(),
            ));
        }
    }
    pub fn to_string(&self, accounts: &Vec<Account>) -> String {
        let mut string = "Balance:\n".to_owned();
        for entry in &self.balances {
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
    fn find_entry(&mut self, transaction: &Transaction) -> Option<&mut Entry> {
        self.balances.iter_mut().find(|entry| {
            (entry.sender_index == transaction.sender_index())
                && (entry.recipient_index == transaction.recipient_index())
        })
    }
}
