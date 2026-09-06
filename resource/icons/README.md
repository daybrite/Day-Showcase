# Day — Dawn Sky (7b) icon

Master art: `day-icon.svg` — rising-sun / rotated-D dome in a rust→marigold gradient
(#E8541C → #FFD84D), marigold alternating long/short rays, warm-orange horizon (#FF8A3C),
teal-blue predawn ground (#123246). Its top-level `day:background` / `day:foreground*` ids drive
the Android adaptive split, the iOS layered icon and the monochrome variant.

Nothing derived is checked in. `day prepare` (run by every `day build`, and by the VS Code
extension before it opens a host project) renders the master into `build/day/host/`, and the
checked-in Xcode, Gradle and hvigor projects read from there — see `docs/icons.md` in the `day`
repository for the layout, and `day prepare --check` for the CI drift gate. Edit the master and
build; there is no export step to remember.

## Hand-drawn variants

These are the designer's per-platform renderings of the same motif, kept for reference and
re-export. The pipeline does not read them. An override is a master for one family, full-bleed
square art that `day prepare` shapes the way it shapes `day-icon.svg`: copied to
`resource/icons/<family>.svg` (`ios.svg`, `android.svg`, …), the iOS and Android variants
below would qualify. The macOS variant already carries the rounded body and margin the
pipeline adds itself, so it stays a reference drawing.

- `ios/day-icon-ios.svg` — the motif as a full-bleed square, the shape iOS masks itself.
- `macos/day-icon-macos.svg` — the motif in Apple's rounded body with the transparent margin
  (824 pt art on a 1024 canvas).
- `android/ic_launcher_foreground.svg` — the dome and rays inside the 66 dp safe zone of the
  108 dp adaptive canvas, transparent background.
- `android/ic_launcher_background.svg` — the solid #123246 ground.
