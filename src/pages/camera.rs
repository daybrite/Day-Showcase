use std::sync::Arc;

use day::prelude::*;
use day_part_permissions::{self as perms, Permission, Status};
use day_piece_camera::{CameraState, Facing, Photo, camera};
use day_piece_remote_image::remote_image;

use crate::widgets::page;

/// Camera (day-piece-camera): the platform's viewfinder, a shutter, and the last photo.
///
/// The page asks for the permission itself, through the Allow button, and the viewfinder runs
/// only once it is held: the piece checks the status before every start and never prompts on
/// its own here. That keeps an OS dialog out of the walkthrough, which cannot dismiss one, and
/// puts the moment of asking where a real app would put it: after the user has seen what the
/// page is for.
///
/// Listed only where the piece renders (`day_piece_camera::support()`, lib.rs `destinations`):
/// the desktops and the web have no camera arm.
pub(crate) fn camera_page() -> AnyPiece {
    let state = Signal::new(CameraState::Idle);
    let photo: Signal<Option<Photo>> = Signal::new(None);
    let facing = Signal::new(Facing::Back);
    let shutter = Trigger::new();
    // The session runs while this is true; it turns true once the permission is held, whether
    // that was restored at build or granted by the button below.
    let active = Signal::new(perms::status(Permission::Camera) == Status::Granted);

    // The photo's bytes, read once per capture for the image below. A photo is a cache file;
    // the piece hands over its path and the app decides what to do with it.
    let bytes: Signal<Option<Arc<Vec<u8>>>> = Signal::new(None);
    Effect::new(move || {
        let next = photo.get().and_then(|p| p.bytes().ok());
        bytes.set(next);
    });

    page(
        crate::res::str::nav_camera(),
        "camera-title",
        form((
            permission_section(active),
            section((
                column((camera()
                    .facing(facing)
                    .active(active)
                    .capture(shutter)
                    .state(state)
                    .photo(photo)
                    .frame(320.0, 240.0)
                    .id("camera-view"),))
                .align(HAlign::Center)
                .grow_w(),
                row((
                    button(crate::res::str::camera_take_photo())
                        .prominent()
                        .enabled(move || state.get() == CameraState::Running)
                        .action(move || shutter.notify())
                        .id("camera-shoot"),
                    button(crate::res::str::camera_flip())
                        .bordered()
                        .enabled(move || state.get() == CameraState::Running)
                        .action(move || facing.update(|f| *f = f.flipped()))
                        .id("camera-flip"),
                ))
                .spacing(8.0),
                labeled(
                    crate::res::str::camera_state_label(),
                    // The locale-independent label, so a script can assert it.
                    label(move || state.get().label().to_string()).id("camera-state"),
                ),
            ))
            .title(crate::res::str::camera_viewfinder_section()),
            section((
                when(
                    move || bytes.get().is_some(),
                    move || {
                        column((remote_image(bytes).frame(240.0, 180.0).id("camera-photo"),))
                            .align(HAlign::Center)
                            .grow_w()
                    },
                ),
                labeled(
                    crate::res::str::camera_photo_label(),
                    label(move || match photo.get() {
                        // The generated accessor takes the message's variables in sorted
                        // order: height, name, width.
                        Some(p) => crate::res::str::camera_photo_size(
                            p.height as i64,
                            p.file_name(),
                            p.width as i64,
                        )
                        .format(),
                        None => crate::res::str::camera_no_photo().format(),
                    })
                    .id("camera-photo-size"),
                ),
            ))
            .title(crate::res::str::camera_last_photo_section()),
        )),
    )
    .any()
}

/// The camera permission's live status and the button that asks for it. Granting flips
/// `active`, which is what starts the viewfinder; the System page's rows stop at the status.
fn permission_section(active: Signal<bool>) -> impl Piece {
    let perm = Permission::Camera;
    let status = Signal::new(format!(
        "{} / {}",
        perms::gate(perm).label(),
        perms::status(perm).label()
    ));
    let can_prompt = perms::can_prompt(perm);
    let gated = perms::gate(perm) != perms::Gate::Absent;
    let action_label = if can_prompt {
        crate::res::str::perm_request()
    } else {
        crate::res::str::perm_open_settings()
    };
    section((labeled(
        crate::res::str::camera_permission_label(),
        row((
            label(move || status.get()).id("camera-perm"),
            when(
                move || gated && !active.get(),
                move || {
                    button(action_label.clone())
                        .bordered()
                        .action(move || {
                            if can_prompt {
                                // The completion runs on an unspecified thread; Setters cross
                                // back to the signals.
                                let set_status = status.setter();
                                let set_active = active.setter();
                                perms::request(perm, move |s| {
                                    set_status.set(format!(
                                        "{} / {}",
                                        perms::gate(perm).label(),
                                        s.label()
                                    ));
                                    if s == Status::Granted {
                                        set_active.set(true);
                                    }
                                });
                            } else {
                                perms::open_settings(perm);
                                status.set(format!(
                                    "{} / {}",
                                    perms::gate(perm).label(),
                                    perms::status(perm).label()
                                ));
                            }
                        })
                        .id("camera-perm-action")
                },
            ),
        ))
        .spacing(8.0)
        .fit(RowFit::ColumnAt(WidthClass::Compact)),
    ),))
    .title(crate::res::str::camera_permission_section())
}
