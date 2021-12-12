mod account;
mod balance;
mod date;
mod ledger;
mod report;
mod transaction;

use balance::Balance;
use ledger::Ledger;

use std::env;
use std::path::Path;

fn main() {
    // Get arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Wrong number of arguments {}", args.len());
    }
    let ledger_path = Path::new(&args[1]);
    let output_folder_path = ledger_path.parent().unwrap();

    let ledger = Ledger::new(ledger_path);
    print!("{}", ledger);

    let mut balance = Balance::new();
    for invoice in ledger.invoices() {
        balance.add_invoice(invoice);
    }
    for payment in ledger.payments() {
        balance.add_payment(payment);
    }
    print!("\n{}", balance.to_string(ledger.accounts()));

    report::generate(&output_folder_path, "TestReport");
}
