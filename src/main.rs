mod account;
mod balance;
mod date;
mod ledger;
mod report_txt;
mod transaction;

use balance::Balance;
use ledger::Ledger;

use std::env;
use std::path::Path;

fn main() {
    // Get arguments
    let args: Vec<String> = env::args().collect();
    let ledger_path = Path::new(&args[1]);
    let report_acronyms = &args[2..];

    let ledger = Ledger::new(ledger_path);
    // print!("{}", ledger);

    let mut balance = Balance::new();
    for invoice in ledger.invoices() {
        balance.add_invoice(invoice);
    }
    for payment in ledger.payments() {
        balance.add_payment(payment);
    }
    // print!("\n{}", balance.to_string(ledger.accounts()));

    let output_folder_path = ledger_path.parent().unwrap();
    for report_acronym in report_acronyms {
        report_txt::generate(&ledger, &balance, &output_folder_path, report_acronym);
    }
}
