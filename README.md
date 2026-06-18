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

## Notes

- Secrets (`doc/local/config.php`, `etc/htaccess`) are git-ignored; a sanitized
  `doc/local/config.php.example` is included instead.
- PDFs are generated with a Python virtualenv (`fpdf2`) — see CLAUDE.md.
