# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

This is the production deployment of **ywesee.com**, the website of ywesee GmbH. It is a
[PmWiki](https://www.pmwiki.org/) installation (engine `pmwiki-2.2.84`, see `doc/scripts/version.php`)
plus log archives and Webalizer-generated traffic statistics. There is no build step, package manager,
or test suite of the usual kind — PmWiki is a self-contained PHP application served directly by a
webserver. The tree is tracked in git (`origin` = <https://github.com/zdavatz/ywesee.com>, branch `main`).

## Layout

- `doc/` — the PmWiki document root (this is what the webserver serves)
  - `pmwiki.php` — the single front-controller; all page requests route through it
  - `local/config.php` — **the main customization file**; site-specific PHP config (see below)
  - `scripts/` — PmWiki core engine scripts (do not edit; these are upstream)
  - `cookbook/` — third-party PmWiki add-on "recipes" (spellchecker, sectionedit, paypalcart, etc.)
  - `wiki.d/` — **live wiki page content**, one flat file per page (see page format below)
  - `wikilib.d/` — bundled/default wiki pages shipped with PmWiki
  - `uploads/` — user-uploaded page attachments
  - `pub/skins/ywesee/` — the active custom theme (`gila.tmpl` template, `gila.css`, logo)
- `etc/htaccess`, `doc/.htaccess` — Apache config; `.htaccess` enables mod_rewrite "Clean URLs"
  that rewrite `/PageName` to `pmwiki.php?n=PageName`
- `log/` — yearly Apache access-log archives (`YYYY.tar.bz2`) plus `rebuild-webalizer`
- `webalizer/`, `webalizer.bak/` — generated HTML traffic-stats output (do not hand-edit)
- `carteblanche/` — Rust (`genpdfi`) generators for the opinion-piece PDF and the consultation-statement
  PDF (see below)

## Cloning / checking out on macOS

`git clone` on macOS ends with `fatal: unable to checkout working tree`. The clone itself is complete;
only the checkout aborts, because 21 paths cannot exist on a default APFS volume:

- **13 files with ISO-8859-1 umlauts in the name** (`Illegal byte sequence`) — APFS only accepts UTF-8
  filenames. E.g. `doc/wiki.d/Main.Komplement<E4>rmedizin`, `doc/wiki.d/Fachinfo.<DC>berblick`,
  `doc/uploads/Main/Zivilgericht_Basel_Verf<FC>gung_14.3.2007.pdf`.
- **8 pairs differing only in case** — the filesystem is case-insensitive, so only one file of each pair
  lands on disk. E.g. `doc/uploads/Android/Apydan.jpg` / `apydan.jpg`, `doc/wiki.d/Ywesee.KIndle` /
  `Ywesee.Kindle`, `doc/wiki.d/ODDB.RecentChanges` / `Oddb.RecentChanges`.

Repair in place instead of re-cloning (the aborted checkout leaves every file staged as deleted):

```sh
git reset -q                                  # rebuild the index from HEAD, working tree untouched
git ls-files -z -m -d | sort -zu | git update-index -z --skip-worktree --stdin
git status --short                            # must be empty
```

Run the second command only right after a fresh clone, when the 21 paths are the only differences —
it hides *every* modified or deleted file from git. `git ls-files -v | grep '^S'` lists the hidden paths.

Consequences on a Mac:

- The 13 umlaut files are not on disk; read them with `git show HEAD:<path>`.
- For each case pair the one file on disk may hold the content of either twin — do not edit those 8 files
  here.
- `git add -A` / commit / push are safe: skip-worktree paths are left alone, nothing gets deleted upstream.
- **Do not rename these files to UTF-8 or merge the case twins** to make the checkout work — the live
  PmWiki server is ISO-8859-1 and case-sensitive, so the page names and attachment links depend on the
  exact bytes. For a complete working copy, clone on Linux.

## Wiki content format (`wiki.d/`)

Each file is one page named `Group.PageName` (e.g. `AIPS.Index`, `AmiKo.Android`). Files named
`Group.PageName,del-<timestamp>` are deleted-page revisions kept for history. The format is a flat
key=value text store (PmWiki's `PageStore`), URL-encoded, holding the current `text=` plus full
revision history (`diff:`, `author:`, `host:` keyed by timestamp). **Do not edit these files by hand** —
edit pages through the wiki's web UI (`?action=edit`) so revision history, checksums, and diffs stay
consistent.

### Homepage news entries (`Main.Home`)

The homepage is the page `Main.Home`; its news list lives in the `text=` field. Each item is three
`%0a`-separated lines — `!!!<D.M.YYYY>`, then `'''<Kategorie>'''`, then `* <Text>` — and the newest
item goes **at the top**, right after the `Attach:yweseelogo.gif` logo line and before the previous
`!!!<date>`. Link a PDF with `[[Attach:<file>.pdf | <Titel>]]` and first copy that PDF into
`doc/uploads/Main/` so the attachment resolves.

When there is no web UI to edit through, edit the flat file directly but **byte-precise**, because it is
ISO-8859-1: write umlauts as the latin-1 byte (`ü`=`0xFC`, `ä`=`0xE4`, `ö`=`0xF6`, `Ä`=`0xC4` …), never
UTF-8 — do it with a small Python script operating on `bytes`, not the text editor. Keep PmWiki
consistent by also: bumping `rev=`, setting the head `time=` to `date +%s`, and prepending one revision
to the history (`author:<t>=zeno`, a reverse `diff:<new>:<prev>:=…` where added lines start with `%3c `
and removed lines with `> `, and `host:<t>=…`). See the scripts under the session scratchpad used for the
13.10.2026 entry for the exact diff format (`3,5d2` for an insert, `4,5c4,5` for a replacement).

## Configuration (`doc/local/config.php`)

All site behavior is driven from here. Key things it sets:
- `$ScriptUrl`/`$PubDirUrl` → `ywesee.com`, skin → `ywesee`, title → "ywesee GmbH"
- Authentication via `scripts/authuser.php` (`$AuthUser[...]`) and `$DefaultPasswords` (admin/edit/upload)
- Uploads enabled (`$EnableUpload`, 50 MB max, `.epub` allowed)
- Add-ons enabled by `include_once('cookbook/...')` — e.g. spellchecker; RSS/feeds via `scripts/feeds.php`
- Clean URLs require `$EnablePathInfo = 1`

Note: this file contains plaintext credentials and a Google Analytics ID. Treat it as sensitive; do not
copy its secrets elsewhere or commit them to a public location.

## Common operations

- **No build/lint/test.** PmWiki runs interpreted; changes to `config.php`, skins, or cookbook scripts
  take effect on next page load.
- **Rebuild traffic stats:** `cd log && ./rebuild-webalizer <lookandfeel> [<config-file>]` — runs
  `webalizer` over the access logs into `../webalizer/<lookandfeel>`. Expects a config at
  `/etc/webalizer/<lookandfeel>.conf` if none is passed, and uses a lock file at
  `/var/lock/update_vhost_stats`.
- **Customize the site:** edit `doc/local/config.php`; for layout edit `doc/pub/skins/ywesee/gila.tmpl`
  and `gila.css`.

## Summaries / generated documents

- Top-level `Zusammenfassung_*.md` / `Zusammenfassung_*.pdf` files are human-facing briefings
  compiled from the wiki content and the linked PDFs under `doc/uploads/Main/`. Each summary
  must include clickable links to the original references (the `https://ywesee.com/uploads/Main/<file>`
  attachments). The Markdown is the source of truth; the PDF mirrors it.
- Wiki attachment URLs map to `https://ywesee.com/uploads/Main/<filename>`; URL-encode special
  characters (e.g. the ISO-8859-1 `ü` byte becomes `%FC`).

## Opinion piece (`Public_Domain_Open_Source_…`)

- `Public_Domain_Open_Source_Grundlage_Innovation_und_gesunden_Wettbewerb.md` / `.pdf` is Zeno Davatz's
  opinion article for the magazine «Innovation Healthcare» (B2B Swiss Medien AG, rubric **Meinung**,
  November 2026 issue; first drafted as a «Carte Blanche», hence the directory name). The repository
  holds the version sent to the editor on 8.10.2026, which already includes her edits.
- The text lives in **two places that must stay identical**: the Markdown file and the `BODY` array in
  `carteblanche/src/main.rs`. The PDF is built from the Rust source, not from the Markdown:
  `cd carteblanche && cargo run --release` (needs the DejaVu fonts, `$FONT_DIR`, default
  `/usr/share/fonts/dejavu`).
- The PDF must fit **one A4 page** — check with `pdfinfo` after every text change and adjust
  `set_line_spacing` / the paragraph `Break` in `main.rs` if it spills over.
- Cited rulings are linked through the `LINKS` table in `main.rs` (phrase → `uploads/Main` URL); a
  phrase is only linked if it appears verbatim in `BODY`, so reword both together.
- Editorial rule from the magazine: keep verifiable facts and the author's opinion apart. Statements
  about what a court decided must be backed by the ruling (e.g. BGer 2C_244/2022, Dispositiv Ziff. 2 and
  E. 10.8.5); conclusions drawn from it are phrased as «Für mich …».

## Consultation statement (`Stellungnahme_ywesee_HMG_3b.*`)

- ywesee's statement on the revision of the Heilmittelgesetz (HMG 3b, consultation deadline 16.10.2026),
  structured like the BAG answer form and submitted to the BAG by e-mail on 13.10.2026 — the files record
  what was sent, so do not reword them. Here the **Markdown is the single source**; the PDF is rendered from
  it by a second binary in the same crate: `cd carteblanche && cargo run --release --bin stellungnahme
  [input.md] [output.pdf]`. Plain `cargo run --release` still builds the opinion piece (`default-run`).
- The renderer (`carteblanche/src/stellungnahme.rs`) only understands `# ` headings, `> ` quoted law text,
  `- ` bullets, `**bold**` and one paragraph per line; addressee, date and title are constants in that file.

## Generating PDFs

- **Always use a Python virtualenv** — never install into the system Python:
  `python3 -m venv venv && ./venv/bin/pip install fpdf2`. If `ensurepip` is missing, create the
  venv with `--without-pip` and bootstrap via `get-pip.py`.
- `fpdf2` with the built-in Helvetica font is **latin-1 only**: replace en/em dashes and curly
  quotes before rendering (`–`/`—`→`-`, `‘’“”`→`'`/`"`); `«»` and `·` are fine. Use clickable
  links via `write(..., link=url)` / `cell(..., link=url)`.

## Working conventions

- Treat everything under `doc/scripts/` and `doc/wikilib.d/` as upstream PmWiki — customize via
  `doc/local/config.php` and `doc/cookbook/` instead of patching the core, so the engine stays upgradable.
- For PmWiki specifics (markup, variables, recipes), consult https://www.pmwiki.org/ rather than guessing.
