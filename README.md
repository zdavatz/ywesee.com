# ywesee.com

ywesee Website (PmWiki) and all documents.

This repository contains the production deployment of **ywesee.com** — a
[PmWiki](https://www.pmwiki.org/) installation — together with its wiki content,
uploaded documents, log archives and Webalizer traffic statistics.

See [CLAUDE.md](CLAUDE.md) for the repository layout, configuration and common operations.

## Summaries / Briefings

- **Documed / Galenica (HCI Solutions) vs. ywesee** — chronological document overview
  with links to every original reference (court rulings, WEKO/ComCo decisions, filings
  2002–2025):
  [Markdown](Zusammenfassung_Documed-Galenica_vs_ywesee.md) ·
  [PDF](Zusammenfassung_Documed-Galenica_vs_ywesee.pdf).
  Source page: <https://ywesee.com/Main/DocumedGalenicaVsYwesee>
- **Public Domain und Open Source ist die bessere Grundlage für Innovation und gesunden
  Wettbewerb** — opinion piece by Zeno Davatz on the same case, written for the rubric
  «Meinung» of the magazine *Innovation Healthcare* (B2B Swiss Medien AG, November 2026).
  Version as sent to the editor on 8.10.2026, with links to the cited rulings:
  [Markdown](Public_Domain_Open_Source_Grundlage_Innovation_und_gesunden_Wettbewerb.md) ·
  [PDF](Public_Domain_Open_Source_Grundlage_Innovation_und_gesunden_Wettbewerb.pdf).
- **Stellungnahme der ywesee GmbH zur Änderung des Heilmittelgesetzes 3b (HMG 3b)** —
  consultation statement submitted to the Federal Office of Public Health (BAG) on
  13.10.2026. It asks that officially approved medicinal product information and the
  related reference data be free to reuse by law (free of charge, machine-readable,
  no contractual restrictions), citing the rulings from the case above:
  [Markdown](Stellungnahme_ywesee_HMG_3b.md) ·
  [PDF](Stellungnahme_ywesee_HMG_3b.pdf).

## Notes

- Secrets (`doc/local/config.php`, `etc/htaccess`) are git-ignored; a sanitized
  `doc/local/config.php.example` is included instead.
- The summary PDF is generated with a Python virtualenv (`fpdf2`). The opinion-piece PDF and
  the consultation-statement PDF are built with the Rust generators in `carteblanche/`
  (`cargo run --release` and `cargo run --release --bin stellungnahme`) — see CLAUDE.md.
