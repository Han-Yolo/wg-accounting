use crate::account::{self, Account};
use crate::balance::{Balance, BalanceEntry};
use crate::ledger::Ledger;
use crate::transaction::Transaction;

use std::fmt::Write as FmtWrite;
use std::fs::File;
use std::io::Write as IoWrite;
use std::path::Path;

pub fn generate(ledger: &Ledger, balance: &Balance, output: &Path, acronym: &String) {
    let accounts = ledger.accounts();
    let account_index = account::find_index(acronym, accounts);
    let account_name = accounts[account_index].name();
    let file_name = "WG Abrechnung ".to_owned() + &account_name;

    let mut incoming_invoices: Vec<&Transaction> = Vec::new();
    let mut outgoing_invoices: Vec<&Transaction> = Vec::new();
    for invoice in ledger.invoices() {
        if invoice.sender_index() == account_index {
            incoming_invoices.push(invoice);
        } else if invoice.recipient_index() == account_index {
            outgoing_invoices.push(invoice);
        }
    }

    let mut outgoing_payments: Vec<&Transaction> = Vec::new();
    let mut incoming_payments: Vec<&Transaction> = Vec::new();
    for payment in ledger.payments() {
        if payment.sender_index() == account_index {
            outgoing_payments.push(payment);
        } else if payment.recipient_index() == account_index {
            incoming_payments.push(payment);
        }
    }

    let mut relevant_balance_entries: Vec<&BalanceEntry> = Vec::new();
    for balance_entry in balance.entries() {
        if balance_entry.sender_index() == account_index {
            relevant_balance_entries.push(balance_entry);
        } else if balance_entry.sender_index() == account_index {
            relevant_balance_entries.push(balance_entry);
        }
    }

    let mut file = File::create(output.join(&file_name).with_extension("txt")).unwrap();
    writeln!(&mut file, "{}", file_name).unwrap();
    add_transaction_table(
        &mut file,
        "Zu zahlen",
        "An",
        accounts,
        &mut incoming_invoices,
    );
    add_transaction_table(
        &mut file,
        "Zu gute",
        "Von",
        accounts,
        &mut outgoing_invoices,
    );
    add_transaction_table(&mut file, "Gezahlt", "An", accounts, &mut outgoing_payments);
    add_transaction_table(
        &mut file,
        "Erhalten",
        "Von",
        accounts,
        &mut incoming_payments,
    );
    writeln!(file, "\nNoch offen:").unwrap();
    if relevant_balance_entries.len() != 0 {
        relevant_balance_entries.sort_by(|a, b| a.balance().partial_cmp(&b.balance()).unwrap());
        for balance_entry in relevant_balance_entries {
            if balance_entry.balance() > 0.01 {
                let debtor_creditor = if balance_entry.balance() > 0.0 {
                    (
                        balance_entry.sender_index(),
                        balance_entry.recipient_index(),
                    )
                } else {
                    (
                        balance_entry.recipient_index(),
                        balance_entry.sender_index(),
                    )
                };
                let mut line = String::new();
                write!(
                    line,
                    "{} -> {}  ",
                    accounts[debtor_creditor.0].name(),
                    accounts[debtor_creditor.1].name()
                )
                .unwrap();
                for _ in line.len()..35 {
                    line.push(' ');
                }
                add_amount_to_string(balance_entry.balance().abs(), &mut line);
                // Write line to file
                writeln!(file, "{}", line).unwrap();
            }
        }
    }
}

fn add_transaction_table(
    file: &mut File,
    name: &str,
    preposition: &str,
    accounts: &Vec<Account>,
    transactions: &mut Vec<&Transaction>,
) {
    if transactions.len() != 0 {
        transactions.sort();
        writeln!(file, "\n{}:", name).unwrap();
        for transaction in transactions {
            let mut line = String::new();
            write!(line, "{}", transaction.date()).unwrap();
            assert!(preposition.len() <= 4);
            for _ in line.len()..(15 - preposition.len()) {
                line.push(' ');
            }
            write!(
                line,
                "{} {}",
                preposition,
                accounts[transaction.recipient_index()].name()
            )
            .unwrap();
            for _ in line.len()..35 {
                line.push(' ');
            }
            add_amount_to_string(transaction.amount(), &mut line);
            for _ in line.len()..50 {
                line.push(' ');
            }
            write!(line, "{}", transaction.note()).unwrap();
            // Write line to file
            writeln!(file, "{}", line).unwrap();
        }
    }
}

fn add_amount_to_string(amount: f64, string: &mut String) {
    assert!(amount >= 0.0);
    write!(string, "{}.", amount.trunc()).unwrap();
    let cents = (amount.fract() * 100.0).round();
    if cents > 0.0 {
        write!(string, "{} CHF", cents).unwrap();
    } else {
        write!(string, "- CHF").unwrap();
    }
}
