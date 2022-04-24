use crate::account::Account;
use crate::date::Date;

use std::cmp::Ordering;

pub struct Transaction {
    sender_index: usize,
    recipient_index: usize,
    amount: f64,
    date: Date,
    note: String,
}
impl Transaction {
    pub fn new(
        sender_index_param: usize,
        recipient_index_param: usize,
        amount_param: f64,
        date_param: Date,
        note_param: String,
    ) -> Self {
        assert_ne!(sender_index_param, recipient_index_param);
        Transaction {
            sender_index: sender_index_param,
            recipient_index: recipient_index_param,
            amount: amount_param,
            date: date_param,
            note: note_param,
        }
    }
    pub fn sender_index(&self) -> usize {
        self.sender_index
    }
    pub fn recipient_index(&self) -> usize {
        self.recipient_index
    }
    pub fn amount(&self) -> f64 {
        self.amount
    }
    pub fn date(&self) -> Date {
        self.date
    }
    pub fn note(&self) -> String {
        self.note.clone()
    }
    pub fn to_string(&self, accounts: &Vec<Account>) -> String {
        format!(
            "{} -> {} {} CHF\t{}\t{}",
            accounts[self.sender_index].acronym(),
            accounts[self.recipient_index].acronym(),
            self.amount,
            self.date,
            self.note
        )
    }
}
impl Ord for Transaction {
    fn cmp(&self, other: &Self) -> Ordering {
        self.date.cmp(&other.date)
    }
}
impl PartialOrd for Transaction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Eq for Transaction {}
impl PartialEq for Transaction {
    fn eq(&self, other: &Self) -> bool {
        (self.sender_index == other.sender_index)
            && (self.recipient_index == other.recipient_index)
            && (self.date == other.date)
            && (self.note == other.note)
    }
}
