// Render a consultation statement (Vernehmlassungs-Stellungnahme) from its
// Markdown source into a plain A4 letter-style PDF.
//
// Same toolchain as the opinion piece (`genpdfi`, DejaVu fonts), but here the
// Markdown file is the single source of truth: the text is read at run time,
// not embedded. Only the small Markdown subset the statements use is handled:
// `# ` headings, `> ` quoted law text, `- ` bullets, `**bold**` runs, and one
// paragraph per line (a blank line adds vertical space).
//
// Usage: stellungnahme [input.md] [output.pdf]
// Font dir override: $FONT_DIR (default /usr/share/fonts/dejavu).

use anyhow::{anyhow, Result};
use genpdfi::elements::{Break, Paragraph};
use genpdfi::style::{Color, Style};
use genpdfi::{Alignment, Element, Margins};
use std::path::PathBuf;

const DEFAULT_FONT_DIR: &str = "/usr/share/fonts/dejavu";
const DEFAULT_IN: &str = "../Stellungnahme_ywesee_HMG_3b.md";
const DEFAULT_OUT: &str = "../Stellungnahme_ywesee_HMG_3b.pdf";

const ACCENT: Color = Color::Rgb(90, 96, 168);
const INK: Color = Color::Rgb(25, 25, 25);
const DARKGREY: Color = Color::Rgb(90, 90, 90);

const TITLE: [&str; 2] = [
    "Stellungnahme der ywesee GmbH",
    "zur Änderung des Heilmittelgesetzes 3b (HMG 3b)",
];
const ADDRESSEE: [&str; 3] = [
    "Bundesamt für Gesundheit BAG",
    "Sektion Heilmittelrecht",
    "Schwarzenburgstrasse 157, 3003 Bern",
];
const DATE: &str = "13. Oktober 2026";
const RUNNING_HEAD: &str = "ywesee GmbH \u{00B7} Stellungnahme HMG 3b";

fn load_font_family(font_dir: &str) -> Result<genpdfi::fonts::FontFamily<genpdfi::fonts::FontData>> {
    let load = |file: &str| -> Result<genpdfi::fonts::FontData> {
        let path = PathBuf::from(font_dir).join(file);
        let data = std::fs::read(&path).map_err(|e| anyhow!("read {}: {}", path.display(), e))?;
        genpdfi::fonts::FontData::new(data, None).map_err(|e| anyhow!("parse font {}: {}", file, e))
    };
    Ok(genpdfi::fonts::FontFamily {
        regular: load("DejaVuSerif.ttf")?,
        bold: load("DejaVuSerif-Bold.ttf")?,
        italic: load("DejaVuSerif-Italic.ttf")?,
        bold_italic: load("DejaVuSerif-BoldItalic.ttf")?,
    })
}

// Push `text` into `p`, rendering `**…**` runs in bold on top of `base`.
fn push_inline(p: &mut Paragraph, text: &str, base: Style) {
    for (i, run) in text.split("**").enumerate() {
        if run.is_empty() {
            continue;
        }
        let style = if i % 2 == 1 { base.bold() } else { base };
        p.push_styled(run.to_string(), style);
    }
}

fn main() -> Result<()> {
    let font_dir = std::env::var("FONT_DIR").unwrap_or_else(|_| DEFAULT_FONT_DIR.to_string());
    let input = std::env::args().nth(1).unwrap_or_else(|| DEFAULT_IN.to_string());
    let out = std::env::args().nth(2).unwrap_or_else(|| DEFAULT_OUT.to_string());
    let source = std::fs::read_to_string(&input).map_err(|e| anyhow!("read {}: {}", input, e))?;

    let mut doc = genpdfi::Document::new(load_font_family(&font_dir)?);
    // ASCII-only metadata title, see the note in main.rs.
    doc.set_title("Stellungnahme der ywesee GmbH zur Aenderung des Heilmittelgesetzes 3b (HMG 3b)");
    doc.set_minimal_conformance();
    doc.set_line_spacing(1.25);
    let mut deco = genpdfi::SimplePageDecorator::new();
    deco.set_margins(22);
    deco.set_header(|page| {
        let mut p = Paragraph::default();
        p.push_styled(
            format!("{} \u{00B7} Seite {}", RUNNING_HEAD, page),
            Style::new().with_color(DARKGREY).with_font_size(7),
        );
        genpdfi::elements::LinearLayout::vertical()
            .element(p.aligned(Alignment::Right))
            .element(Break::new(1.5))
    });
    doc.set_page_decorator(deco);

    // --- letterhead: addressee, date, title ---
    let small = Style::new().with_color(INK).with_font_size(9);
    for line in ADDRESSEE.iter() {
        let mut p = Paragraph::default();
        p.push_styled((*line).to_string(), small);
        doc.push(p);
    }
    doc.push(Break::new(0.8));
    {
        let mut p = Paragraph::default();
        p.push_styled(DATE.to_string(), small);
        doc.push(p);
    }
    doc.push(Break::new(1.5));
    for line in TITLE.iter() {
        let mut p = Paragraph::default();
        p.push_styled(
            (*line).to_string(),
            Style::new().with_color(Color::Rgb(20, 20, 20)).with_font_size(15).bold(),
        );
        doc.push(p);
        doc.push(Break::new(0.2));
    }
    doc.push(Break::new(1.2));

    // --- body ---
    let body = Style::new().with_color(INK).with_font_size(9);
    let quote = Style::new().with_color(INK).with_font_size(9).italic();
    let heading = Style::new().with_color(ACCENT).with_font_size(13).bold();
    for raw in source.lines() {
        // The Federal Chancellery writes "Art. 9cbis"; the caret is only our
        // plain-text stand-in for the superscript.
        let line = raw.trim_end().replace('^', "");
        if line.is_empty() {
            doc.push(Break::new(0.6));
        } else if let Some(text) = line.strip_prefix("# ") {
            doc.push(Break::new(0.6));
            let mut p = Paragraph::default();
            p.push_styled(text.to_string(), heading);
            doc.push(p);
        } else if let Some(text) = line.strip_prefix("> ") {
            let mut p = Paragraph::default();
            push_inline(&mut p, text, quote);
            doc.push(p.padded(Margins::trbl(0, 6, 0, 6)));
        } else if let Some(text) = line.strip_prefix("- ") {
            let mut p = Paragraph::default();
            p.push_styled("\u{2022} ".to_string(), body);
            push_inline(&mut p, text, body);
            doc.push(p.padded(Margins::trbl(0, 0, 1, 4)));
        } else {
            let mut p = Paragraph::default();
            push_inline(&mut p, &line, body);
            doc.push(p);
        }
    }

    doc.render_to_file(&out).map_err(|e| anyhow!("render {}: {}", out, e))?;
    println!("written {}", out);
    Ok(())
}
