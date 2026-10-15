#!/usr/bin/env python3
"""Post a text update (optionally with a link) to LinkedIn.

Reuses the posting logic of https://github.com/zdavatz/swissdamed2sqlite
(`src/linkedin.rs`): refresh the OAuth token if possible, then create a PUBLIC
post via the REST Posts API. This variant publishes a text post (LinkedIn
auto-generates a preview from any URL in the text) instead of an image post.

Credentials are read from the SAME files as swissdamed2sqlite / li_push_rs and
are NEVER stored in this repository:
    linkedin_credentials.json  -> {"client_id", "client_secret"}
    linkedin_token.json        -> {"access_token","refresh_token","person_id",...}
Lookup order: $SWISSDAMED_CREDENTIALS_DIR, ~/.config/swissdamed2sqlite, cwd, $HOME.
Obtain linkedin_token.json once via the OAuth flow (scopes: openid profile
w_member_social); see CLAUDE.md -> "Posting to LinkedIn".

Usage:
    python3 tools/li_post.py --text "Hello $(date)\n\nhttps://example.com"
    python3 tools/li_post.py --file post.txt
    echo "my post" | python3 tools/li_post.py
    python3 tools/li_post.py --file post.txt --dry-run   # print body, post nothing
"""
import argparse, json, os, sys, urllib.request, urllib.error
from pathlib import Path

LINKEDIN_VERSION = "202603"

# LinkedIn "little text" reserved characters must be backslash-escaped in commentary.
_ESC = set("()<>@|{}[]*_~\\")
def escape_little_text(s: str) -> str:
    return "".join(("\\" + c) if c in _ESC else c for c in s)

def cred_dirs():
    d = os.environ.get("SWISSDAMED_CREDENTIALS_DIR")
    if d:
        yield Path(d)
    home = Path.home()
    yield home / ".config" / "swissdamed2sqlite"
    yield Path.cwd()
    yield home

def find_file(name: str) -> Path:
    for d in cred_dirs():
        p = d / name
        if p.is_file():
            return p
    raise SystemExit(f"{name} not found (looked in: "
                     + ", ".join(str(d) for d in cred_dirs()) + ")")

def http(method, url, headers, data=None):
    body = json.dumps(data).encode() if isinstance(data, (dict, list)) else data
    req = urllib.request.Request(url, data=body, method=method, headers=headers)
    try:
        r = urllib.request.urlopen(req, timeout=120)
        return r.status, dict(r.headers), r.read().decode()
    except urllib.error.HTTPError as e:
        return e.code, dict(e.headers), e.read().decode()

def refresh(creds, token, token_path):
    if not token.get("refresh_token"):
        return token
    form = (f"grant_type=refresh_token&refresh_token={token['refresh_token']}"
            f"&client_id={creds['client_id']}&client_secret={creds['client_secret']}")
    st, _, txt = http("POST", "https://www.linkedin.com/oauth/v2/accessToken",
                      {"Content-Type": "application/x-www-form-urlencoded"}, form.encode())
    if st == 200:
        d = json.loads(txt)
        if d.get("access_token"):
            token["access_token"] = d["access_token"]
            token["refresh_token"] = d.get("refresh_token", token["refresh_token"])
            token_path.write_text(json.dumps(token, indent=2))
            os.chmod(token_path, 0o600)
            print("[linkedin] token refreshed", file=sys.stderr)
    else:
        print(f"[linkedin] refresh failed ({st}); using existing token", file=sys.stderr)
    return token

def main():
    ap = argparse.ArgumentParser()
    g = ap.add_mutually_exclusive_group()
    g.add_argument("--text", help="post commentary (use \\n for newlines)")
    g.add_argument("--file", help="read commentary from this file")
    ap.add_argument("--dry-run", action="store_true", help="print the request body, post nothing")
    args = ap.parse_args()

    if args.text is not None:
        commentary = args.text.replace("\\n", "\n")
    elif args.file:
        commentary = Path(args.file).read_text()
    elif not sys.stdin.isatty():
        commentary = sys.stdin.read()
    else:
        raise SystemExit("no post text: pass --text, --file, or pipe it on stdin")
    commentary = commentary.strip()
    if not commentary:
        raise SystemExit("post text is empty")

    creds_path = find_file("linkedin_credentials.json")
    token_path = find_file("linkedin_token.json")
    creds = json.loads(creds_path.read_text())
    token = json.loads(token_path.read_text())
    if not token.get("person_id"):
        raise SystemExit("linkedin_token.json has empty person_id (re-run the OAuth flow)")
    token = refresh(creds, token, token_path)

    owner = f"urn:li:person:{token['person_id']}"
    body = {
        "author": owner,
        "commentary": escape_little_text(commentary),
        "visibility": "PUBLIC",
        "distribution": {"feedDistribution": "MAIN_FEED",
                         "targetEntities": [], "thirdPartyDistributionChannels": []},
        "lifecycleState": "PUBLISHED",
        "isReshareDisabledByAuthor": False,
    }
    if args.dry_run:
        print(json.dumps(body, indent=2, ensure_ascii=False))
        return

    st, hdrs, txt = http("POST", "https://api.linkedin.com/rest/posts",
        {"Authorization": f"Bearer {token['access_token']}",
         "Content-Type": "application/json",
         "LinkedIn-Version": LINKEDIN_VERSION, "X-Restli-Protocol-Version": "2.0.0"}, body)
    if st not in (200, 201):
        raise SystemExit(f"create post failed ({st}): {txt}")
    pid = hdrs.get("x-restli-id") or hdrs.get("X-RestLi-Id") or "(unknown)"
    print(f"[linkedin] Published: https://www.linkedin.com/feed/update/{pid}/")

if __name__ == "__main__":
    main()
