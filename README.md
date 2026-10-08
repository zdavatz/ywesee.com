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

## Notes

- Secrets (`doc/local/config.php`, `etc/htaccess`) are git-ignored; a sanitized
  `doc/local/config.php.example` is included instead.
- The summary PDF is generated with a Python virtualenv (`fpdf2`); the opinion-piece PDF is
  built with the Rust generator in `carteblanche/` (`cargo run --release`) — see CLAUDE.md.
