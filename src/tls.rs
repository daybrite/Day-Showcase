//! Shared thread-local storage for settings and demo random sequences. Empty initialization
//! preserves lazy settings installation and clock seeding; one key serves all three slots.

use std::cell::{Cell, RefCell};

day::reactive::tls_slots! { shared;
    /// The installed settings surface used by About and the phone's settings section.
    pub(crate) static PREFS: RefCell<Option<day_piece_settings::Prefs>> =
        const { RefCell::new(None) };
    /// Animation seeds from the clock on its first random request.
    pub(crate) static ANIMATION_SEED: Cell<u64> = const { Cell::new(0) };
    /// List shuffles advance a deterministic sequence for the life of the app.
    pub(crate) static SHUFFLE_SEED: Cell<u64> = const { Cell::new(0x9E37_79B9_7F4A_7C15) };
}
