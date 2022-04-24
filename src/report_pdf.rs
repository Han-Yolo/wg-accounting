use crate::account::{self, Account};
use crate::balance::{Balance, BalanceEntry};
use crate::ledger::Ledger;
use crate::transaction::Transaction;

use printpdf::*;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

const PAGE_WIDTH: f64 = 210.0;
const PAGE_HEIGHT: f64 = 297.0;

const TOP_BORER: f64 = PAGE_HEIGHT - 30.0;
const LEFT_BORER: f64 = 20.0;
const BOTTOM_BORDER: f64 = 30.0;

const SECTION_TITLE_SIZE: f64 = 14.0;
const TEXT_SIZE: f64 = 12.0;
const LINE_DISTANCE: f64 = 6.0;

pub struct Report<'a> {
    accounts: &'a Vec<Account>,
    ledger: &'a Ledger,
    balance: &'a Balance,
    output: &'a Path,
    active_document: Option<PdfDocumentReference>,
    active_layer: Option<PdfLayerReference>,
    cursor_height: f64,
}
impl<'a> Report<'a> {
    pub fn new(
        ledger_param: &'a Ledger,
        balance_param: &'a Balance,
        output_param: &'a Path,
    ) -> Self {
        Report {
            accounts: ledger_param.accounts(),
            ledger: ledger_param,
            balance: balance_param,
            output: output_param,
            active_document: None,
            active_layer: None,
            cursor_height: TOP_BORER - 30.0,
        }
    }
    pub fn generate(&mut self, acronym: &String) {
        let accounts = self.ledger.accounts();
        let account_index = account::find_index(acronym, accounts);
        let account_name = accounts[account_index].name();
        let file_name = "WG Abrechnung ".to_owned() + &account_name;

        let mut incoming_invoices: Vec<&Transaction> = Vec::new();
        let mut outgoing_invoices: Vec<&Transaction> = Vec::new();
        for invoice in self.ledger.invoices() {
            if invoice.sender_index() == account_index {
                incoming_invoices.push(invoice);
            } else if invoice.recipient_index() == account_index {
                outgoing_invoices.push(invoice);
            }
        }

        let mut outgoing_payments: Vec<&Transaction> = Vec::new();
        let mut incoming_payments: Vec<&Transaction> = Vec::new();
        for payment in self.ledger.payments() {
            if payment.sender_index() == account_index {
                outgoing_payments.push(payment);
            } else if payment.recipient_index() == account_index {
                incoming_payments.push(payment);
            }
        }

        let mut balance_entries: Vec<&BalanceEntry> = Vec::new();
        for entrie in self.balance.entries() {
            if (entrie.sender_index() == account_index)
                || (entrie.recipient_index() == account_index)
            {
                balance_entries.push(entrie);
            }
        }

        // Make new pdf object
        let (mut document, page_index, layer_index) =
            PdfDocument::new(&file_name, Mm(PAGE_WIDTH), Mm(PAGE_HEIGHT), "Layer 1");
        document = document.with_conformance(PdfConformance::Custom(CustomPdfConformance {
            requires_icc_profile: false,
            requires_xmp_metadata: false,
            ..Default::default()
        }));
        self.active_document = Some(document);
        self.active_layer = Some(document.get_page(page_index).get_layer(layer_index));
        let layer = self.active_layer.as_ref().unwrap();

        // Get fonts
        let normal_font = document
            .add_external_font(
                File::open("/usr/share/fonts/truetype/open-sans/OpenSans-Regular.ttf").unwrap(),
            )
            .unwrap();
        let name_font = document
            .add_external_font(
                File::open("/usr/share/fonts/opentype/noto/NotoSansCJK-Bold.ttc").unwrap(),
            )
            .unwrap();

        // Write title
        layer.begin_text_section();
        layer.set_text_cursor(Mm(LEFT_BORER), Mm(TOP_BORER));
        layer.set_line_height(33.0);
        layer.set_font(&normal_font, 32.0);
        layer.write_text("WG Abrechnung", &normal_font);
        layer.add_line_break();
        layer.set_font(&name_font, 26.0);
        layer.write_text("      ".to_owned() + account_name, &name_font);
        layer.end_text_section();

        // Write incoming invoices
        self.create_table(
            String::from("Zu zahlen"),
            &mut incoming_invoices,
            &normal_font,
        );

        doc.save(&mut BufWriter::new(
            File::create(self.output.join(&file_name).with_extension("pdf")).unwrap(),
        ))
        .unwrap();
    }

    fn create_table(
        &mut self,
        name: String,
        transactions: &mut Vec<&Transaction>,
        font: &IndirectFontRef,
    ) {
        if transactions.len() != 0 {
            let table_end = self.cursor_height - (LINE_DISTANCE * transactions.len() as f64);
            if table_end < BOTTOM_BORDER {
                let (new_page_index, new_layer_index) =
                    doc.add_page(Mm(PAGE_WIDTH), Mm(PAGE_HEIGHT), "Layer 1");
                layer = doc.get_page(new_page_index).get_layer(new_layer_index);
                self.cursor_height = TOP_BORER;
            }
            transactions.sort();
            let layer = self.active_layer.as_ref().unwrap();
            layer.begin_text_section();
            layer.set_font(&font, SECTION_TITLE_SIZE);
            layer.set_text_cursor(Mm(LEFT_BORER), Mm(self.cursor_height));
            layer.write_text(name + ":", &font);
            layer.end_text_section();
            self.cursor_height -= LINE_DISTANCE;
            for outgoing_invoice in transactions {
                layer.begin_text_section();
                layer.set_font(&font, TEXT_SIZE);
                layer.set_text_cursor(Mm(LEFT_BORER), Mm(self.cursor_height));
                layer.write_text(
                    format!(
                        "{}     {}",
                        outgoing_invoice.date(),
                        self.accounts[outgoing_invoice.recipient_index()].name(),
                    ),
                    &font,
                );
                layer.end_text_section();
                layer.begin_text_section();
                layer.set_font(&font, TEXT_SIZE);
                layer.set_text_cursor(Mm(LEFT_BORER + 70.0), Mm(self.cursor_height));
                layer.write_text(format!("{} CHF", outgoing_invoice.amount(),), &font);
                layer.end_text_section();
                layer.begin_text_section();
                layer.set_font(&font, TEXT_SIZE);
                layer.set_text_cursor(Mm(LEFT_BORER + 100.0), Mm(self.cursor_height));
                layer.write_text(format!("{}", outgoing_invoice.note()), &font);
                layer.end_text_section();
                self.cursor_height -= LINE_DISTANCE;
            }
            layer.end_text_section();
        }
    }
}
