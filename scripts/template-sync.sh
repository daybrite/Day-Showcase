#!/usr/bin/env bash
# Keep the Content List page a faithful copy of the `day new` template.
#
# The page under src/pages/content_list/ is the scaffold's own item list, editor and model
# (crates/day-cli/templates/app/src/{model.rs,pages/navigate.rs,pages/detail.rs} in the day
# repository) so the showcase demonstrates exactly what a new app starts with. The three files
# and the four glyphs they draw are GENERATED from the template by this script; CI runs it with
# --check so a template change that is not mirrored here fails the build rather than drifting.
#
# The only edits applied, so the copy can live as a module of this crate instead of at a crate
# root of its own (each is one `sed` line below):
#   1. `use crate::Section;`          -> `use super::Section;`      (the module's own route shim)
#   2. `use crate::model::…`          -> `use super::model::…`       (sibling module, not crate root)
# `use crate::res;` stays as it is: this crate exposes its resources at `crate::res` too.
#
#     scripts/template-sync.sh [--check] [path/to/day]     (default ../day, or $DAY_REPO)
set -euo pipefail
cd "$(dirname "$0")/.."

check=0
day="${DAY_REPO:-../day}"
for arg in "$@"; do
    case "$arg" in
        --check) check=1 ;;
        *) day="$arg" ;;
    esac
done
tpl="$day/crates/day-cli/templates/app"
if [ ! -f "$tpl/src/model.rs" ]; then
    echo "template-sync: no scaffold template at $tpl (pass the day checkout, or set DAY_REPO)" >&2
    exit 2
fi

adapt() {
    sed -e 's|^use crate::Section;$|use super::Section;|' \
        -e 's|^use crate::model::|use super::model::|'
}

# (template file, checked-in copy, adapt?)
entries=(
    "src/model.rs|src/pages/content_list/model.rs|1"
    "src/pages/navigate.rs|src/pages/content_list/navigate.rs|1"
    "src/pages/detail.rs|src/pages/content_list/detail.rs|1"
    "resource/vectors/kind_note.svg|resource/vectors/kind_note.svg|0"
    "resource/vectors/kind_task.svg|resource/vectors/kind_task.svg|0"
    "resource/vectors/kind_idea.svg|resource/vectors/kind_idea.svg|0"
    "resource/vectors/check.svg|resource/vectors/check.svg|0"
)

status=0
for entry in "${entries[@]}"; do
    IFS='|' read -r from to adapt_it <<<"$entry"
    if [ "$adapt_it" = 1 ]; then
        want="$(adapt <"$tpl/$from"; echo x)"; want="${want%x}"
    else
        want="$(cat "$tpl/$from"; echo x)"; want="${want%x}"
    fi
    if [ "$check" = 1 ]; then
        if [ ! -f "$to" ] || [ "$(cat "$to"; echo x)" != "${want}x" ]; then
            echo "template-sync: $to differs from the template's $from" >&2
            diff -u "$to" <(printf '%s' "$want") >&2 || true
            status=1
        fi
    else
        printf '%s' "$want" >"$to"
        echo "template-sync: wrote $to"
    fi
done
if [ "$check" = 1 ] && [ "$status" = 0 ]; then
    echo "template-sync: src/pages/content_list matches the template"
fi
exit "$status"
