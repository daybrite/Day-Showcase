//! Generate typed resource constants from `resource/` (§18.5) — the same one-liner `day new`
//! scaffolds into every app. `day-build` writes `$OUT_DIR/day_resources.rs`, surfaced as the `res`
//! module in lib.rs, so the showcase references its bundled icons/data/fonts by checked symbol.
fn main() {
    day_build::generate_resources().expect("day-build: resource codegen");
    // Bake the app identity (Day.toml `[app].id`, exported by `day build`/`day launch` as
    // `DAY_APP_ID` — crates/day-cli/src/ops.rs::apply_app_identity) so the About page can show
    // the bundle id without a runtime manifest read. Same pattern as day-break's build.rs:
    // re-exporting through `cargo:rustc-env` makes a value change invalidate the compile.
    println!("cargo:rerun-if-env-changed=DAY_APP_ID");
    if let Ok(id) = std::env::var("DAY_APP_ID") {
        println!("cargo:rustc-env=DAY_SHOWCASE_APP_ID={id}");
    }

    // The git ref the "Show Source" button (pages/*.rs → GitHub) points at: the `vX.Y.Z` tag for a
    // released build, else `main` for a development build. A tagged release is built by the
    // daybrite/actions pipeline from a tag push, where GITHUB_REF_TYPE=tag and GITHUB_REF_NAME is
    // the tag; bake that so the link lands on the exact released source. Baked at compile so a
    // stale ref can never outlive its build.
    println!("cargo:rerun-if-env-changed=GITHUB_REF_TYPE");
    println!("cargo:rerun-if-env-changed=GITHUB_REF_NAME");
    println!("cargo:rustc-env=DAY_SHOWCASE_SOURCE_REF={}", source_ref());
}

/// The source ref to link against: a release tag when this is a tagged build, else `main`.
fn source_ref() -> String {
    // The CI release path: the workflow only fires on `v[0-9]+.[0-9]+.[0-9]+*` tags, so a `tag`
    // ref type already implies a release tag — trust its name.
    if std::env::var("GITHUB_REF_TYPE").as_deref() == Ok("tag")
        && let Ok(name) = std::env::var("GITHUB_REF_NAME")
        && name.starts_with('v')
    {
        return name;
    }
    // A local release (`day pack` off a tagged checkout): an exact `vX.Y.Z` tag on HEAD. Any other
    // state (a branch, a detached non-tag commit, or no git at all) is a dev build → `main`.
    if let Some(tag) = git_exact_tag() {
        return tag;
    }
    "main".to_string()
}

/// The exact semantic-version tag pointing at HEAD, or `None` (not on a tag, or not a git checkout).
fn git_exact_tag() -> Option<String> {
    let out = std::process::Command::new("git")
        .args(["describe", "--tags", "--exact-match", "HEAD"])
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let tag = String::from_utf8(out.stdout).ok()?.trim().to_string();
    tag.starts_with('v').then_some(tag)
}
