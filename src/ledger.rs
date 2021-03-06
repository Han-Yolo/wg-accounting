use crate::account::{self, Account};
use crate::date::Date;
use crate::transaction::Transaction;

use regex::Regex;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct Ledger {
    accounting_date: Date,
    accounts: Vec<Account>,
    invoices: Vec<Transaction>,
    payments: Vec<Transaction>,
}
impl Ledger {
    pub fn new(input: &Path) -> Self {
        // Regex pattern strings
        let comment_pattern = r"^//";
        let date_pattern = r"(?P<day>\d{1,2})\.(?P<month>\d{1,2})\.(?P<year>\d{4})";
        let time_range_pattern = r"(?P<end_date>\s-\s\d{1,2}\.\d{1,2}\.\d{4})(?P<frequency>\s:\s\d{1,2}\.\d{1,2}\.\d{4})";
        let amount_pattern = r"(?P<amount>\d+\.\d+)";
        let note_pattern = r"(?P<note>.+)";
        let header_pattern = r"^accounting_date\s".to_owned() + &date_pattern.to_owned() + r"$";
        let account_pattern = r"^account\s(?P<acronym>[A-Z]{2})\s(?P<name>.+)";
        let invoice_pattern =
            r"^invoice\s(?P<first_sender>[A-Z]{2})(\s:\s[A-Z]{2})*(\s->\s[A-Z]{2})+\s".to_owned()
                + &amount_pattern.to_owned()
                + r"\s(?P<start_date>\d{1,2}\.\d{1,2}\.\d{4})(?P<time_range>\s-\s\d{1,2}\.\d{1,2}\.\d{4}\s:\s\d{1,2}\.\d{1,2}\.\d{4})?\s"
                + &note_pattern.to_owned();
        let additional_sender_pattern = r"\s:\s(?P<additional_sender>[A-Z]{2})";
        let recipient_pattern = r"\s\->\s(?P<recipient>[A-Z]{2})";
        let payment_pattern = r"^payment\s(?P<sender>[A-Z]{2})\s->\s(?P<recipient>[A-Z]{2})\s"
            .to_owned()
            + &amount_pattern.to_owned()
            + r"\s"
            + &date_pattern.to_owned()
            + r"\s"
            + &note_pattern.to_owned();

        // Regex objects
        let comment_regex = Regex::new(comment_pattern).unwrap();
        let date_regex = Regex::new(date_pattern).unwrap();
        let time_range_regex = Regex::new(time_range_pattern).unwrap();
        let header_regex = Regex::new(header_pattern.as_str()).unwrap();
        let account_regex = Regex::new(account_pattern).unwrap();
        let invoice_regex = Regex::new(invoice_pattern.as_str()).unwrap();
        let additional_sender_regex = Regex::new(additional_sender_pattern).unwrap();
        let recipient_regex = Regex::new(recipient_pattern).unwrap();
        let payment_regex = Regex::new(payment_pattern.as_str()).unwrap();

        // Open input file
        let transactions_file = File::open(input).unwrap();
        let mut transactions_lines = BufReader::new(transactions_file).lines();

        // Parse header
        let first_line = transactions_lines.next().unwrap().unwrap();
        let header_captures = header_regex.captures(&first_line).unwrap();
        let accounting_date = Date::new(&header_captures);

        // Parse accounts, invoices and transactions
        let mut accounts: Vec<Account> = Vec::new();
        let mut invoices: Vec<Transaction> = Vec::new();
        let mut payments: Vec<Transaction> = Vec::new();

        let mut add_invoice =
            |sender_index: usize, recipient_index: usize, amount: f64, date: Date, note: String| {
                if sender_index != recipient_index {
                    let mut actual_amount = amount;
                    let existing_invoice = invoices.iter().find(|invoice| {
                        (((invoice.sender_index() == sender_index)
                            && (invoice.recipient_index() == recipient_index))
                            || ((invoice.sender_index() == recipient_index)
                                && (invoice.recipient_index() == sender_index)))
                            && (invoice.date() == date)
                            && (invoice.note() == note)
                    });
                    if existing_invoice.is_some() {
                        // This invoice exists already
                        let existing_invoice_clone = (*existing_invoice.unwrap()).clone();
                        // Add the amount of the existing invoice to the new invoice
                        actual_amount += if existing_invoice_clone.sender_index() == sender_index {
                            existing_invoice_clone.amount()
                        } else {
                            -existing_invoice_clone.amount()
                        };
                        // Remove existing invoice
                        invoices.retain(|invoice| invoice != &existing_invoice_clone);
                    }
                    invoices.push(Transaction::new(
                        sender_index,
                        recipient_index,
                        actual_amount,
                        date,
                        note,
                    ));
                }
            };

        for wraped_line in transactions_lines {
            let line = wraped_line.unwrap();
            if line == "" {
                // Ignore empty line
            } else if let Some(_captures) = comment_regex.captures(&line) {
                // Ignore comment
            } else if let Some(captures) = account_regex.captures(&line) {
                // Add new account
                accounts.push(Account::new(&captures));
            } else if let Some(captures) = invoice_regex.captures(&line) {
                // Determine invoice dates
                let start_date = Date::new(
                    &date_regex
                        .captures(captures.name("start_date").unwrap().as_str())
                        .unwrap(),
                );
                if start_date > accounting_date {
                    // Ignore invoice
                    continue;
                }
                let mut invoice_dates: Vec<Date> = Vec::new();
                invoice_dates.push(start_date);
                if let Some(time_range_match) = captures.name("time_range") {
                    let time_range_captures = time_range_regex
                        .captures(time_range_match.as_str())
                        .unwrap();
                    let mut end_date = Date::new(
                        &date_regex
                            .captures(time_range_captures.name("end_date").unwrap().as_str())
                            .unwrap(),
                    );
                    if end_date > accounting_date {
                        end_date = accounting_date;
                    }
                    let frequency_captures = date_regex
                        .captures(time_range_captures.name("frequency").unwrap().as_str())
                        .unwrap();
                    let frequency_days = frequency_captures
                        .name("day")
                        .unwrap()
                        .as_str()
                        .parse::<u32>()
                        .unwrap();
                    let frequency_months = frequency_captures
                        .name("month")
                        .unwrap()
                        .as_str()
                        .parse::<u32>()
                        .unwrap();
                    let frequency_years = frequency_captures
                        .name("day")
                        .unwrap()
                        .as_str()
                        .parse::<u32>()
                        .unwrap();
                    let mut new_date = start_date;
                    loop {
                        new_date.add(frequency_days, frequency_months, frequency_years);
                        if new_date > end_date {
                            break;
                        };
                        invoice_dates.push(new_date);
                    }
                }
                // Determine sender indices
                let mut sender_acronyms: Vec<String> = Vec::new();
                sender_acronyms.push(captures.name("first_sender").unwrap().as_str().to_owned());
                for additional_sender in additional_sender_regex.captures_iter(&line) {
                    sender_acronyms.push(
                        additional_sender
                            .name("additional_sender")
                            .unwrap()
                            .as_str()
                            .to_owned(),
                    );
                }
                let sender_indices: Vec<usize> = sender_acronyms
                    .iter()
                    .map(|acronym| account::find_index(&acronym, &accounts))
                    .collect();
                // Determine recipient indices
                let mut recipient_acronyms: Vec<String> = Vec::new();
                for recipient in recipient_regex.captures_iter(&line) {
                    recipient_acronyms
                        .push(recipient.name("recipient").unwrap().as_str().to_owned());
                }
                let recipient_indices: Vec<usize> = recipient_acronyms
                    .iter()
                    .map(|acronym| account::find_index(&acronym, &accounts))
                    .collect();
                // Add invoices
                let total_amount = captures
                    .name("amount")
                    .unwrap()
                    .as_str()
                    .parse::<f64>()
                    .unwrap();
                let amount_per_sender = total_amount / sender_indices.len() as f64;
                let note = captures.name("note").unwrap().as_str().to_owned();
                for invoice_date in invoice_dates {
                    // Sender -> first recipient
                    for sender_index in &sender_indices {
                        add_invoice(
                            *sender_index,
                            recipient_indices[0],
                            amount_per_sender,
                            invoice_date,
                            note.clone(),
                        );
                    }
                    // Additional recipients
                    for i in 1..recipient_indices.len() {
                        add_invoice(
                            recipient_indices[i - 1],
                            recipient_indices[i],
                            total_amount,
                            invoice_date,
                            note.clone(),
                        );
                    }
                }
            } else if let Some(captures) = payment_regex.captures(&line) {
                // Add payment
                let sender_acronym = captures.name("sender").unwrap().as_str().to_owned();
                let recipient_acronym = captures.name("recipient").unwrap().as_str().to_owned();
                let date = Date::new(&captures);
                if date > accounting_date {
                    // Ignore payment
                    continue;
                }
                payments.push(Transaction::new(
                    account::find_index(&sender_acronym, &accounts),
                    account::find_index(&recipient_acronym, &accounts),
                    captures
                        .name("amount")
                        .unwrap()
                        .as_str()
                        .parse::<f64>()
                        .unwrap(),
                    date,
                    captures.name("note").unwrap().as_str().to_owned(),
                ));
            } else {
                panic!("Parsing error on line \"{}\"", line);
            }
        }
        Self {
            accounting_date: accounting_date,
            accounts: accounts,
            invoices: invoices,
            payments: payments,
        }
    }
    pub fn accounting_date(&self) -> &Date {
        &self.accounting_date
    }
    pub fn accounts(&self) -> &Vec<Account> {
        &self.accounts
    }
    pub fn invoices(&self) -> &Vec<Transaction> {
        &self.invoices
    }
    pub fn payments(&self) -> &Vec<Transaction> {
        &self.payments
    }
}
impl fmt::Display for Ledger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string = format!("Accounting date {}\n", &self.accounting_date)
            .as_str()
            .to_owned();
        string += "\nAccounts:\n";
        for account in &self.accounts {
            string += format!("{}\n", account).as_str();
        }
        string += "\nInvoices:\n";
        for invoice in &self.invoices {
            string += format!("{}\n", invoice.to_string(&self.accounts)).as_str();
        }
        string += "\nPayments:\n";
        for payment in &self.payments {
            string += format!("{}\n", payment.to_string(&self.accounts)).as_str();
        }
        write!(f, "{}", string)
    }
}
