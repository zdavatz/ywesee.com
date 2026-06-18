# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

This is the production deployment of **ywesee.com**, the website of ywesee GmbH. It is a
[PmWiki](https://www.pmwiki.org/) installation (engine `pmwiki-2.2.84`, see `doc/scripts/version.php`)
plus log archives and Webalizer-generated traffic statistics. There is no build step, package manager,
or test suite of the usual kind — PmWiki is a self-contained PHP application served directly by a
webserver. This is **not** a git repository.

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

## Wiki content format (`wiki.d/`)

Each file is one page named `Group.PageName` (e.g. `AIPS.Index`, `AmiKo.Android`). Files named
`Group.PageName,del-<timestamp>` are deleted-page revisions kept for history. The format is a flat
key=value text store (PmWiki's `PageStore`), URL-encoded, holding the current `text=` plus full
revision history (`diff:`, `author:`, `host:` keyed by timestamp). **Do not edit these files by hand** —
edit pages through the wiki's web UI (`?action=edit`) so revision history, checksums, and diffs stay
consistent.

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

## Working conventions

- Treat everything under `doc/scripts/` and `doc/wikilib.d/` as upstream PmWiki — customize via
  `doc/local/config.php` and `doc/cookbook/` instead of patching the core, so the engine stays upgradable.
- For PmWiki specifics (markup, variables, recipes), consult https://www.pmwiki.org/ rather than guessing.
