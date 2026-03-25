#!/usr/bin/env bash
# publish.sh — Publish an Anatoly report and optionally open an issue on the upstream repo.
# Usage: ./publish.sh [--no-issue] [--dry] [--run <id>]
#
# Run this from the project you audited (where .anatoly/ lives).
# Requires: git, gh (GitHub CLI)

set -euo pipefail

REPORTS_REPO="r-via/anatoly-reports"
REPORTS_REPO_URL="https://github.com/${REPORTS_REPO}.git"
SAFE_NWO='^[a-zA-Z0-9._-]+$'

# ── Parse args ──────────────────────────────────────────────────────────────
NO_ISSUE=false
DRY=false
RUN_ID=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --no-issue) NO_ISSUE=true; shift ;;
    --dry)      DRY=true; shift ;;
    --run)      RUN_ID="$2"; shift 2 ;;
    *)          echo "Unknown option: $1"; exit 1 ;;
  esac
done

# ── Resolve run directory ───────────────────────────────────────────────────
ANATOLY_DIR=".anatoly/runs"

if [[ -n "$RUN_ID" ]]; then
  RUN_DIR="${ANATOLY_DIR}/${RUN_ID}"
else
  # Find latest run (symlink or most recent directory)
  if [[ -L "${ANATOLY_DIR}/latest" ]]; then
    RUN_DIR="${ANATOLY_DIR}/$(readlink "${ANATOLY_DIR}/latest")"
  elif [[ -f "${ANATOLY_DIR}/latest" ]]; then
    RUN_DIR="${ANATOLY_DIR}/$(cat "${ANATOLY_DIR}/latest")"
  else
    RUN_DIR="${ANATOLY_DIR}/$(ls -1 "${ANATOLY_DIR}" | grep -v latest | sort | tail -1)"
  fi
fi

if [[ ! -f "${RUN_DIR}/report.md" ]]; then
  echo "Error: No report.md found in ${RUN_DIR}. Run 'anatoly run' first." >&2
  exit 1
fi

RUN_ID="$(basename "$RUN_DIR")"
REPORT_PATH="${RUN_DIR}/report.md"

# ── Detect upstream repo ────────────────────────────────────────────────────
detect_nwo() {
  local remote="$1"
  local url
  url="$(git remote get-url "$remote" 2>/dev/null)" || return 1

  local owner repo
  if [[ "$url" =~ github\.com[/:]([^/]+)/([^/.]+) ]]; then
    owner="${BASH_REMATCH[1]}"
    repo="${BASH_REMATCH[2]}"
    if [[ "$owner" =~ $SAFE_NWO ]] && [[ "$repo" =~ $SAFE_NWO ]]; then
      echo "${owner}/${repo}"
      return 0
    fi
  fi
  return 1
}

UPSTREAM=""

# 1. Try "upstream" remote
UPSTREAM="$(detect_nwo upstream 2>/dev/null)" || true

# 2. Try gh fork parent
if [[ -z "$UPSTREAM" ]]; then
  PARENT="$(gh repo view --json parent --jq '.parent | "\(.owner.login)/\(.name)"' 2>/dev/null)" || true
  if [[ -n "$PARENT" && "$PARENT" != "null/null" && "$PARENT" != "/" ]]; then
    UPSTREAM="$PARENT"
  fi
fi

# 3. Fallback to origin
if [[ -z "$UPSTREAM" ]]; then
  UPSTREAM="$(detect_nwo origin 2>/dev/null)" || true
fi

if [[ -z "$UPSTREAM" ]]; then
  echo "Error: Could not detect upstream repository." >&2
  echo "Add an 'upstream' remote or run from a GitHub fork." >&2
  exit 1
fi

OWNER="${UPSTREAM%%/*}"
REPO="${UPSTREAM##*/}"
PROJECT_SLUG="${OWNER}--${REPO}"
REPORT_SUBDIR="${PROJECT_SLUG}/${RUN_ID}"

echo ""
echo "Anatoly Report Upstream"
echo "  Target repo:   ${UPSTREAM}"
echo "  Report path:   ${REPORT_SUBDIR}"
echo "  Reports repo:  ${REPORTS_REPO}"
echo ""

if $DRY; then
  echo "Dry run — no changes will be made."
  exit 0
fi

# ── Clone, copy, push ──────────────────────────────────────────────────────
TMPDIR="$(mktemp -d)"
trap 'rm -rf "$TMPDIR"' EXIT

echo "Cloning anatoly-reports..."
git clone --depth 1 "$REPORTS_REPO_URL" "$TMPDIR" --quiet

DEST_DIR="${TMPDIR}/${REPORT_SUBDIR}"
mkdir -p "$DEST_DIR"

# Copy report + axes
cp "$REPORT_PATH" "$DEST_DIR/report.md"
if [[ -d "${RUN_DIR}/axes" ]]; then
  cp -r "${RUN_DIR}/axes" "$DEST_DIR/axes"
fi

# Commit and push
cd "$TMPDIR"
git add .

REPORT_URL="https://github.com/${REPORTS_REPO}/blob/main/${REPORT_SUBDIR}/report.md"

if ! git diff --cached --quiet 2>/dev/null; then
  git commit -m "audit: ${UPSTREAM} (${RUN_ID})" --quiet
  git push --quiet
  echo "Report published: ${REPORT_URL}"
else
  echo "Report already published: ${REPORT_URL}"
fi

cd - > /dev/null

# ── Open issue ──────────────────────────────────────────────────────────────
if $NO_ISSUE; then
  echo ""
  exit 0
fi

REPORT_CONTENT="$(cat "$REPORT_PATH")"

# Build issue body
ISSUE_BODY="Hey! My name is Rémi. I created [Anatoly](https://github.com/r-via/anatoly), a free open-source audit tool for codebases. It helps clean up vibe-coded projects by checking for common issues across multiple axes — things like dead code, duplications, missing docs, correctness bugs, over-engineering, test gaps, and best practices.

I ran it on your project and here is the report. Hopefully you will find something useful in there! Feel free to reach out if you have any questions or feedback.
"

# Extract Executive Summary (between ## Executive Summary and next ##)
SUMMARY="$(echo "$REPORT_CONTENT" | sed -n '/^## Executive Summary$/,/^## /{/^## Executive Summary$/p;/^## [^E]/!p}' | head -n -1)"
if [[ -n "$SUMMARY" ]]; then
  ISSUE_BODY+="
${SUMMARY}
"
fi

# Extract Axis Summary
AXIS_SUMMARY="$(echo "$REPORT_CONTENT" | sed -n '/^## Axis Summary$/,/^## /{/^## Axis Summary$/p;/^## [^A]/!p}' | head -n -1)"
if [[ -n "$AXIS_SUMMARY" ]]; then
  ISSUE_BODY+="
${AXIS_SUMMARY}
"
fi

ISSUE_BODY+="
---

**[View full report](${REPORT_URL})**

*Generated by [Anatoly](https://github.com/r-via/anatoly) — Deep Audit Agent for codebases*"

echo ""
ISSUE_URL="$(echo "$ISSUE_BODY" | gh issue create --repo "$UPSTREAM" --title "Anatoly Audit Report" --body-file -)"
echo "Issue created: ${ISSUE_URL}"
echo ""