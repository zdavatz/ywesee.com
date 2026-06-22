// Render the Carte-Blanche article into a magazine-style PDF.
//
// Pure Rust, no Chrome: built directly with `genpdf` (DejaVu fonts), the same
// toolchain we use in ../../listingtracker. genpdf 0.2 has no multi-column or
// hyperlink support, so the two-column print look of the reference is rendered
// as a single justified-left flow that still fits one A4 page.
//
// Font dir override: $FONT_DIR (default /usr/share/fonts/dejavu).
// Output: ../CarteBlanche_Documed-Galenica_vs_ywesee.pdf (override with arg 1).

use anyhow::{anyhow, Result};
use genpdf::elements::{Break, Paragraph};
use genpdf::style::{Color, Style};
use genpdf::Alignment;
use std::path::PathBuf;

const DEFAULT_FONT_DIR: &str = "/usr/share/fonts/dejavu";

const ACCENT: Color = Color::Rgb(90, 96, 168); // muted violet/blue like the reference
const INK: Color = Color::Rgb(25, 25, 25);
const DARKGREY: Color = Color::Rgb(70, 70, 70);

const TITLE: [&str; 3] = [
    "Public Domain und Open Source",
    "ist die bessere Grundlage für",
    "Innovation und gesunden Wettbewerb",
];
const AUTHOR: &str = "ZENO DAVATZ";
const ROLE: [&str; 1] = ["ywesee GmbH"];

const BODY: [&str; 10] = [
    "Vor über zwanzig Jahren stellte ich mir eine einfache Frage: Wem gehören eigentlich die \
Arzneimittel-Informationen, die jede Ärztin und jeder Apotheker täglich braucht? Die von Swissmedic \
geprüften Fach- und Patienteninformationen entstehen im Rahmen eines staatlichen Zulassungsverfahrens. \
Sie sind die Grundlage jeder sicheren Verschreibung. Und doch wurden sie jahrelang behandelt, als wären \
sie das private Eigentum eines einzigen Unternehmens.",
    "Als ich diese Daten über die quelloffene Plattform oddb.org frei und für alle zugänglich machte, \
folgte 2003 prompt ein superprovisorisches Verbot. Der Vorwurf lautete «Raubkopieren» – dabei ging es \
um nichts anderes als das Teilen von öffentlich finanziertem Wissen. Damit begann ein Rechtsstreit, der \
mich – und die Frage nach dem freien Zugang zu medizinischem Wissen – durch sämtliche Instanzen führen \
sollte.",
    "Das Zivilgericht Basel-Stadt hob das Verbot bereits 2004 auf: Behördlich geprüfte \
Arzneimittelinformationen sind kein urheberrechtlich geschütztes Werk. 2007 wurde die Klage \
vollumfänglich abgewiesen, 2008 bestätigte das Bundesgericht diesen Entscheid letztinstanzlich (Urteil \
4A_404/2007). Die Botschaft war eindeutig: Was im Auftrag des Staates entsteht und der öffentlichen \
Sicherheit dient, lässt sich nicht beliebig privatisieren.",
    "Parallel dazu prüfte die Wettbewerbskommission (WEKO) den Markt. Schon 2005 hielt sie in der \
Untersuchung 32-0178 fest, dass der damalige Anbieter marktbeherrschend war. 2016 sprach die WEKO eine \
Sanktion von über 4,5 Millionen Franken aus (Verfügung 32-0249) – wegen Exklusivklauseln, die \
Softwarehäuser daran hinderten, die Daten frei zu beziehen. Das Bundesverwaltungsgericht bestätigte den \
Kern 2022 (Urteil B-2597/2017), das Bundesgericht verengte den Vorwurf 2025 zwar deutlich (Urteil \
2C_244/2022), hielt aber am entscheidenden Punkt fest: Niemand darf Softwarehäusern verbieten, gleich \
strukturierte Drittdaten zu Medikamenteninformationen zu verwenden.",
    "Warum erzähle ich Ihnen das? Weil dieser über zwei Jahrzehnte dauernde Streit weit über meinen \
Einzelfall hinausweist. Er handelt von einer Frage, die unsere gesamte Gesundheitsbranche \
betrifft: Wem gehören die Daten, auf denen Versorgung, Forschung und Innovation \
aufbauen?",
    "Daten sind die Grundlage, der Nährboden für Innovation und für die \
Weiterentwicklung neuer Ideen. Was im Auftrag des Staates geprüft und mit öffentlichen Mitteln \
finanziert wird, gehört in die Public Domain – quelloffen und frei nutzbar für alle. Werden solche \
Daten hinter Exklusivverträgen und Lizenzgebühren eingeschlossen, ersticken wir genau jene Innovation, \
die wir im Schweizer Gesundheitswesen so dringend benötigen.",
    "Mein Fall zeigt: Ein langer Atem lohnt sich. Über zwanzig Jahre, mehrere Gerichtsverfahren, \
zwei Kartelluntersuchungen – das alles hat sich mehr als gelohnt: dass öffentlich geprüftes Wissen \
öffentlich nutzbar ist.",
    "Im Silicon Valley investieren selbst direkte Konkurrenten ineinander: \
Google steckte schon 2015 rund 900 Millionen Dollar in SpaceX und finanziert heute mit Milliarden den \
KI-Entwickler Anthropic – obwohl dieser mit Googles eigener KI konkurriert. Auch Amazon hat rund acht \
Milliarden Dollar in Anthropic investiert. Wettbewerb und Zusammenarbeit schliessen sich dort nicht aus. \
Autistische Monopole dagegen verhindern Innovation und halten die Kosten hoch.",
    "Bessere Rahmenbedingungen beginnen nicht erst in Bundesbern. Sie beginnen \
bei der Frage, ob wir Wissen und Software als gemeinsame Allmende begreifen – als Open Source und Public \
Domain, die allen nützt – oder als Festung, die einige wenige schützt.",
    "Das Schweizer Gesundheitswesen krankt an einem zu starren Gärtchen-Diagnose-Denken: mein Gärtchen, dein \
Gärtchen. Um beim Lernen weiterzukommen, muss man Wissen teilen. Mein Aufruf an Sie: Öffnen Sie Ihre \
Daten und Ihren Code dort, wo es den Patientinnen und Patienten dient. Setzen Sie auf offene \
Schnittstellen, Open Source und die Public Domain statt auf Exklusivklauseln. Gerade im Gesundheitswesen \
gilt: Sharing is Caring – Teilen ist gelebte Fürsorge. Wer heute auf Offenheit setzt, schafft die \
Rahmenbedingungen für die Innovation von morgen.",
];

const RUBRIC: &str = "\u{25B6} In dieser Rubrik äussern Fachleute aus Gesundheit und Life Sciences \
ihre Meinung zu aktuellen Themen.";

fn load_font_family(font_dir: &str) -> Result<genpdf::fonts::FontFamily<genpdf::fonts::FontData>> {
    let load = |file: &str| -> Result<genpdf::fonts::FontData> {
        let path = PathBuf::from(font_dir).join(file);
        let data = std::fs::read(&path).map_err(|e| anyhow!("read {}: {}", path.display(), e))?;
        genpdf::fonts::FontData::new(data, None).map_err(|e| anyhow!("parse font {}: {}", file, e))
    };
    Ok(genpdf::fonts::FontFamily {
        regular: load("DejaVuSerif.ttf")?,
        bold: load("DejaVuSerif-Bold.ttf")?,
        italic: load("DejaVuSerif-Italic.ttf")?,
        bold_italic: load("DejaVuSerif-BoldItalic.ttf")?,
    })
}

fn push_lines(doc: &mut genpdf::Document, text: &str, style: Style, align: Alignment) {
    for line in text.lines() {
        let mut p = Paragraph::default();
        p.push_styled(line.to_string(), style);
        doc.push(p.aligned(align));
    }
}

fn main() -> Result<()> {
    let font_dir = std::env::var("FONT_DIR").unwrap_or_else(|_| DEFAULT_FONT_DIR.to_string());
    let out = std::env::args()
        .nth(1)
        .unwrap_or_else(|| {
            "../Public_Domain_Open_Source_Grundlage_Innovation_und_gesunden_Wettbewerb.pdf".to_string()
        });

    let family = load_font_family(&font_dir)?;
    let mut doc = genpdf::Document::new(family);
    // PDF metadata title: keep ASCII-only — printpdf 0.3 does not encode the
    // document-info title as UTF-8, so umlauts/en-dashes would show as mojibake
    // in the viewer's title bar (the visible article text is unaffected).
    doc.set_title("Carte Blanche: Public Domain und Open Source als bessere Grundlage von Innovation und gesundem Wettbewerb");
    doc.set_minimal_conformance();
    doc.set_line_spacing(0.88);
    let mut deco = genpdf::SimplePageDecorator::new();
    deco.set_margins(16);
    doc.set_page_decorator(deco);

    // --- rubric label ---
    {
        let mut p = Paragraph::default();
        p.push_styled(
            "\u{25CF} CARTE BLANCHE".to_string(),
            Style::new().with_color(ACCENT).with_font_size(9).bold(),
        );
        doc.push(p);
    }
    doc.push(Break::new(3.0));

    // --- big title (each line as its own paragraph + explicit break so the
    // large glyphs never overlap) ---
    let title_style = Style::new()
        .with_color(Color::Rgb(20, 20, 20))
        .with_font_size(19)
        .bold();
    for line in TITLE.iter() {
        let mut p = Paragraph::default();
        p.push_styled((*line).to_string(), title_style);
        doc.push(p.aligned(Alignment::Center));
        doc.push(Break::new(0.4));
    }
    doc.push(Break::new(0.8));

    // --- body (each paragraph gets a first-line indent, like the reference) ---
    let body_style = Style::new().with_color(INK).with_font_size(8);
    let indent = "\u{2003}\u{2003}"; // two em-spaces ~ a tab stop
    for (i, para) in BODY.iter().enumerate() {
        let mut p = Paragraph::default();
        p.push_styled(indent.to_string(), body_style);
        if i == 0 {
            p.push_styled("\u{25B6} ".to_string(), Style::from(ACCENT).bold());
        }
        p.push_styled((*para).to_string(), body_style);
        doc.push(p.aligned(Alignment::Left));
        doc.push(Break::new(0.12));
    }

    // --- author block ---
    doc.push(Break::new(0.3));
    push_lines(
        &mut doc,
        AUTHOR,
        Style::new().with_color(Color::Rgb(20, 20, 20)).with_font_size(10).bold(),
        Alignment::Left,
    );
    push_lines(
        &mut doc,
        &ROLE.join("\n"),
        Style::new().with_color(DARKGREY).with_font_size(9),
        Alignment::Left,
    );

    // --- rubric footnote ---
    doc.push(Break::new(0.6));
    push_lines(
        &mut doc,
        RUBRIC,
        Style::new().with_color(Color::Rgb(60, 60, 60)).with_font_size(9).italic(),
        Alignment::Left,
    );

    doc.render_to_file(&out)
        .map_err(|e| anyhow!("render {}: {}", out, e))?;
    println!("written {}", out);
    Ok(())
}
