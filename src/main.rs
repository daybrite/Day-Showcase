fn main() {
    // Register app-lifecycle handlers before launch so `WillLaunch` is captured (docs/lifecycle.md).
    dayapp::install_lifecycle_handlers();
    // The window description lives beside `root` (lib.rs `window`), so the desktop and mobile
    // entry points open the same window (docs/windows.md).
    day::launch(dayapp::window(), dayapp::root);
}
