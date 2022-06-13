use crate::account::{self, Account};
use crate::balance::{Balance, BalanceEntry};
use crate::ledger::Ledger;
use crate::transaction::Transaction;

use std::fmt::Write as FmtWrite;
use std::fs::File;
use std::io::Write as IoWrite;
use std::path::Path;

enum MoneyFlow {
    In,
    Out,
}

pub fn generate(ledger: &Ledger, balance: &Balance, output: &Path, acronym: &String) {
    let accounts = ledger.accounts();
    let account_index = account::find_index(acronym, accounts);
    let account_name = accounts[account_index].name();
    let title = format!(
        "WG Abrechnung {} {}",
        ledger.accounting_date(),
        &account_name
    );

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
        if (balance_entry.sender_index() == account_index)
            || (balance_entry.recipient_index() == account_index)
        {
            relevant_balance_entries.push(balance_entry);
        }
    }

    let mut file =
        File::create(output.join(&title.replace(".", "_")).with_extension("txt")).unwrap();
    writeln!(&mut file, "{}", title).unwrap();
    add_transaction_table(
        &mut file,
        "Zu zahlen",
        MoneyFlow::Out,
        accounts,
        &mut incoming_invoices,
    );
    add_transaction_table(
        &mut file,
        "Zu gute",
        MoneyFlow::In,
        accounts,
        &mut outgoing_invoices,
    );
    add_transaction_table(
        &mut file,
        "Gezahlt",
        MoneyFlow::Out,
        accounts,
        &mut outgoing_payments,
    );
    add_transaction_table(
        &mut file,
        "Erhalten",
        MoneyFlow::In,
        accounts,
        &mut incoming_payments,
    );
    writeln!(file, "\nNoch offen:").unwrap();
    let mut uncleared_balance_found = false;
    relevant_balance_entries.sort_by(|a, b| a.balance().partial_cmp(&b.balance()).unwrap());
    for balance_entry in relevant_balance_entries {
        let balance = (balance_entry.balance() * 100.0).round() / 100.0;
        if balance.abs() >= 0.01 {
            uncleared_balance_found = true;
            let debtor_creditor = if balance > 0.0 {
                (
                    balance_entry.recipient_index(),
                    balance_entry.sender_index(),
                )
            } else {
                (
                    balance_entry.sender_index(),
                    balance_entry.recipient_index(),
                )
            };
            let mut line = String::new();
            write!(
                line,
                "{} -> {}",
                accounts[debtor_creditor.0].name(),
                accounts[debtor_creditor.1].name()
            )
            .unwrap();
            for _ in line.chars().count()..35 {
                line.push(' ');
            }
            add_amount_to_string(balance.abs(), &mut line);
            // Write line to file
            writeln!(file, "{}", line).unwrap();
        }
    }
    if !uncleared_balance_found {
        writeln!(file, "-").unwrap();
    }
}

fn add_transaction_table(
    file: &mut File,
    name: &str,
    money_flow: MoneyFlow,
    accounts: &Vec<Account>,
    transactions: &mut Vec<&Transaction>,
) {
    if transactions.len() != 0 {
        transactions.sort();
        let preposition = match money_flow {
            MoneyFlow::In => "Von",
            MoneyFlow::Out => "An",
        };
        writeln!(file, "\n{}:", name).unwrap();
        for transaction in transactions {
            let mut line = String::new();
            write!(line, "{}", transaction.date()).unwrap();

            assert!(preposition.len() <= 4);
            for _ in line.chars().count()..(15 - preposition.len()) {
                line.push(' ');
            }
            let other_index = match money_flow {
                MoneyFlow::In => transaction.sender_index(),
                MoneyFlow::Out => transaction.recipient_index(),
            };
            write!(line, "{} {}", preposition, accounts[other_index].name()).unwrap();
            for _ in line.chars().count()..35 {
                line.push(' ');
            }
            add_amount_to_string(transaction.amount(), &mut line);
            for _ in line.chars().count()..50 {
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
