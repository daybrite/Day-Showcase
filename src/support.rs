//! Which demos this target can actually run (docs/coverage-matrix.md).
//!
//! Two rules, by granularity. A PAGE whose central feature the target cannot run is not in the
//! sidebar at all (lib.rs `destinations`): a target with no toolbar has nothing to show on a
//! Toolbars page, and a screenshot of the excuse helps nobody. A SECTION inside a page that the
//! target cannot run stays on screen and carries a banner instead
//! ([`crate::widgets::support_note`]): a visitor comparing two platforms in the gallery should see
//! the same page with an honest note, since a section that vanished would read as a bug in the
//! showcase rather than as a fact about the platform.
//!
//! **Ask the runtime wherever it can answer.** `capability(...)` and a part's own `available()` /
//! `is_supported()` are the truth, they follow the framework as backends gain arms, and they
//! cannot go stale. Only a feature with no such answer gets a declared list here, and each one
//! says why it has to be declared.

use day::prelude::*;

/// A feature the toolkit reports on directly.
pub(crate) fn cap(c: Cap) -> Support {
    capability(c)
}

/// Text to speech: the part answers for this host, engine and all (docs/speech.md).
pub(crate) fn speech() -> Support {
    day_part_speech::available()
}

/// Haptics: the part knows whether this platform has an engine.
pub(crate) fn haptics() -> Support {
    from_bool(day_part_haptics::is_supported())
}

/// Local notifications: likewise.
pub(crate) fn notifications() -> Support {
    from_bool(day_part_local_notify::is_supported())
}

/// Crash reporting (day-break, docs/break.md).
///
/// Declared: the crash handlers are signal/exception hooks with no capability to query, and the
/// web build has no equivalent at all — a wasm trap ends the page, and there is nothing to catch
/// it with or anywhere to persist a report to for the next launch.
pub(crate) fn crash_reporting() -> Support {
    unsupported_on(&["DOM"])
}

/// The device battery (docs/battery.md).
///
/// Declared rather than read from `status()`, which answers `None` both for "this platform has no
/// battery API" and for "this machine has no battery" — a desktop tower would otherwise wear a
/// banner saying Day does not support batteries.
pub(crate) fn battery() -> Support {
    unsupported_on(&["DOM"])
}

/// Motion sensors (docs/sensors.md).
///
/// Declared: the part streams readings and reports absence as an empty stream, which is also what
/// a device with no gyroscope looks like.
pub(crate) fn sensors() -> Support {
    unsupported_on(&["XAML"])
}

fn from_bool(supported: bool) -> Support {
    if supported {
        Support::Native
    } else {
        Support::Unsupported
    }
}

/// `Unsupported` when this binary's toolkit is named, `Native` otherwise. The toolkit is the axis
/// these gaps fall on; `day::toolkit_name()` is the one it is spelled in.
fn unsupported_on(toolkits: &[&str]) -> Support {
    if toolkits.contains(&day::toolkit_name()) {
        Support::Unsupported
    } else {
        Support::Native
    }
}
