use printpdf::*;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

const PAGE_WIDTH: f64 = 210.0;
const PAGE_HEIGHT: f64 = 297.0;

pub fn generate(output: &Path, name: &str) {
    let (doc, page1, layer1) = PdfDocument::new(name, Mm(PAGE_WIDTH), Mm(PAGE_HEIGHT), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    let title_font = doc
        .add_external_font(File::open("/usr/share/fonts/truetype/baekmuk/dotum.ttf").unwrap())
        .unwrap();
    current_layer.use_text(name, 48.0, Mm(30.0), Mm(PAGE_HEIGHT - 30.0), &title_font);

    doc.save(&mut BufWriter::new(
        File::create(output.join(name).with_extension("pdf")).unwrap(),
    ))
    .unwrap();
}
