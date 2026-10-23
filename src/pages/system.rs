use day::prelude::*;

use crate::widgets::{battery_line, page};

/// Device & sensors (docs/battery.md, docs/sensors.md, docs/network.md): every headless
/// device-state part in one grouped form: the battery visualization with preview controls,
/// the connectivity snapshot, the motion sensors, and the device identity. Each group is a
/// form `section`; the readout rows are `labeled`, so their labels align form-wide.
pub(crate) fn system_page() -> AnyPiece {
    page(
        crate::res::str::nav_system(),
        day::id!(system_title),
        form((
            sensors_section(),
            sensor_section(
                day_part_sensors::SensorKind::Magnetometer,
                crate::res::str::sensor_magnetometer(),
                crate::res::str::sensor_unit_magnetic,
                day::id!(sensor_magnet),
            ),
            sensor_section(
                day_part_sensors::SensorKind::Accelerometer,
                crate::res::str::sensor_accelerometer(),
                crate::res::str::sensor_unit_acceleration,
                day::id!(sensor_accel),
            ),
            sensor_section(
                day_part_sensors::SensorKind::Gyroscope,
                crate::res::str::sensor_gyroscope(),
                crate::res::str::sensor_unit_rotation,
                day::id!(sensor_gyro),
            ),
            location_section(),
            device_section(),
            battery_section(),
        ))
        .any(),
    )
    .any()
}

fn battery_section() -> impl Piece {
    // Seed the preview signals from the device's real reading (a demo value when there's none).
    let status = day_part_battery::status();
    let level = Signal::new(
        status
            .and_then(|b| b.percent())
            .map(f64::from)
            .unwrap_or(80.0),
    );
    let charging = Signal::new(status.map(|b| b.is_charging()).unwrap_or(false));
    let reading = Signal::new(battery_line().format());
    section((
        crate::widgets::support_note(crate::support::battery()),
        // Centred: the drawing is a fixed 260x120 box, so on any window wider than that it
        // would otherwise sit against the leading edge with a growing gap beside it. The column
        // takes the full width (`grow_w`) and centers its one child inside it.
        column((battery_view(level, charging),))
            .align(HAlign::Center)
            .grow_w(),
        labeled(
            crate::res::str::battery_level(),
            row((
                slider(level).range(0.0..=100.0).id(day::id!(battery_level)),
                label(move || format!("{:.0}%", level.get()))
                    .tabular()
                    .id(day::id!(battery_level_value)),
            ))
            .spacing(Space::S),
        ),
        labeled(
            crate::res::str::battery_charging(),
            toggle(charging).id(day::id!(battery_charging)),
        ),
        row((
            button(crate::res::str::battery_refresh())
                .action(move || {
                    reading.set(battery_line().format());
                    if let Some(b) = day_part_battery::status() {
                        if let Some(p) = b.percent() {
                            level.set(f64::from(p));
                        }
                        charging.set(b.is_charging());
                    }
                })
                .tint(crate::widgets::primary())
                .id(day::id!(battery_refresh)),
            label(move || reading.get()).id(day::id!(battery_reading)),
        ))
        .spacing(Space::S),
    ))
    .title(crate::res::str::nav_battery())
}

/// The sensors group's opener: the platform note and the motion permission's row. The three
/// sensors follow as sections of their own ([`sensor_section`]), each titled by its name, so the
/// readout has a line to itself.
fn sensors_section() -> impl Piece {
    section((
        crate::widgets::support_note(crate::support::sensors()),
        permission_row(
            crate::res::str::sensor_permission(),
            day_part_permissions::Permission::Motion,
            day::id!(sensors_perm),
        ),
    ))
    .title(crate::res::str::nav_sensors())
}

/// Each sensor owns a subscription: pausing drops it, resuming starts a new generation.
/// Only current-generation samples reach the displayed state, so an already queued callback
/// cannot change a paused readout or leak an old reading into a resumed stream.
fn sensor_section(
    kind: day_part_sensors::SensorKind,
    title: day::LocalizedText,
    unit: fn() -> day::LocalizedText,
    id: day::ElementId,
) -> impl Piece {
    use std::{cell::RefCell, rc::Rc};

    let available = day_part_sensors::is_available(kind);
    use day_part_sensors::Calibration;
    let can_calibrate = available
        && day_part_sensors::is_available(kind.with_calibration(Calibration::Uncalibrated));
    let calibrated = Signal::new(true);
    let running = Signal::new(available);
    let reading = Signal::new(None::<day_part_sensors::SensorReading>);
    let series = Signal::new(Vec::<day_part_sensors::SensorReading>::new());
    let incoming = Signal::new(None::<(u64, day_part_sensors::SensorReading)>);
    let generation = Rc::new(std::cell::Cell::new(0_u64));
    let subscription = Rc::new(RefCell::new(None));
    // A mode change starts a fresh history, including when paused. Pause/resume itself
    // retains the last reading; no graph ever joins calibrated and uncalibrated samples.
    watch(
        move || calibrated.get(),
        move |_, _| {
            reading.set(None);
            series.set(Vec::new());
        },
    );
    day::reactive::bind(move || (running.get(), calibrated.get()), {
        let generation = generation.clone();
        let subscription = subscription.clone();
        move |(active, corrected)| {
            let epoch = generation.get().wrapping_add(1);
            generation.set(epoch);
            subscription.borrow_mut().take();
            if *active {
                let setter = incoming.setter();
                *subscription.borrow_mut() = Some(day_part_sensors::watch(
                    kind.with_calibration(if *corrected {
                        Calibration::Calibrated
                    } else {
                        Calibration::Uncalibrated
                    }),
                    move |sample| {
                        setter.set(Some((epoch, sample)));
                    },
                ));
            }
        }
    });
    Scope::current().on_cleanup(move || drop(subscription.borrow_mut().take()));
    watch(
        move || incoming.get(),
        move |sample, _| {
            if let Some((epoch, sample)) = sample
                && running.get()
                && *epoch == generation.get()
            {
                reading.set(Some(*sample));
                series.update(|history| {
                    if history.len() == CHART_SAMPLES {
                        history.remove(0);
                    }
                    history.push(*sample);
                });
            }
        },
    );

    // Fixed character slots align signs, decimal points, and all three axes. Monospacing
    // includes the padding spaces (tabular figures alone do not); reserving fixes the box.
    // Formatting stays on the UI thread, where the localization bundles are installed.
    let widest = crate::res::str::sensor_reading(crate::res::str::SensorReading {
        x: "-000.00",
        y: "-000.00",
        z: "-000.00",
        unit: unit().format(),
    })
    .format();
    let toggle_id = day::id!(sensor_running, id.as_str());
    let calibration_id = day::id!(sensor_calibrated, id.as_str());
    let chart_id = day::id!(sensor_chart, id.as_str());
    section((
        labeled(
            crate::res::str::sensor_live(),
            toggle(running).enabled(available).id(toggle_id),
        ),
        when(
            move || can_calibrate,
            move || {
                labeled(
                    crate::res::str::sensor_calibrated(),
                    toggle(calibrated).id(calibration_id.clone()),
                )
            },
        ),
        when(
            move || available,
            move || {
                let (visual, hint) = match kind {
                    day_part_sensors::SensorKind::Magnetometer => (
                        column((compass_view(reading, calibrated),))
                            .align(HAlign::Center)
                            .grow_w()
                            .any(),
                        label(move || {
                            if calibrated.get() {
                                crate::res::str::sensor_compass_hint().format()
                            } else {
                                crate::res::str::sensor_uncalibrated_hint().format()
                            }
                        })
                        .font(Font::Footnote)
                        .secondary()
                        .any(),
                    ),
                    day_part_sensors::SensorKind::Accelerometer => (
                        column((acceleration_view(reading),))
                            .align(HAlign::Center)
                            .grow_w()
                            .any(),
                        label(crate::res::str::sensor_tilt_hint())
                            .font(Font::Footnote)
                            .secondary()
                            .any(),
                    ),
                    day_part_sensors::SensorKind::Gyroscope => (
                        rotation_view(reading).any(),
                        label(crate::res::str::sensor_rotation_hint())
                            .font(Font::Footnote)
                            .secondary()
                            .any(),
                    ),
                };
                column((
                    visual,
                    hint,
                    strip_chart(series, chart_id.clone()),
                    label(crate::res::str::chart_axes())
                        .font(Font::Caption)
                        .secondary(),
                ))
                .any()
            },
        ),
        label(move || match reading.get() {
            Some(r) => crate::res::str::sensor_reading(crate::res::str::SensorReading {
                x: format!("{:+7.2}", r.x),
                y: format!("{:+7.2}", r.y),
                z: format!("{:+7.2}", r.z),
                unit: unit().format(),
            })
            .format(),
            None if available => crate::res::str::sensor_waiting().format(),
            None => crate::res::str::sensor_unavailable().format(),
        })
        .monospace()
        .tabular()
        .font(Font::Footnote)
        .id(id.clone())
        .reserving(widest),
    ))
    .title(title)
}

/// Angle of magnetic north relative to the device's portrait top edge. The raw x/y
/// projection is meaningful with the phone flat; z is still exposed in the readout.
fn magnetic_direction(reading: day_part_sensors::SensorReading) -> Option<f64> {
    (reading.x.hypot(reading.y) > 0.001).then(|| reading.x.atan2(reading.y))
}

fn compass_view(
    reading: Signal<Option<day_part_sensors::SensorReading>>,
    calibrated: Signal<bool>,
) -> impl Piece {
    canvas(move |d, size| {
        // Canvas records once before layout, when its size is still zero.
        if size.width < 60.0 || size.height < 60.0 {
            return;
        }

        let c = Point::new(size.width / 2.0, size.height / 2.0);
        let radius = size.width.min(size.height) / 2.0 - 14.0;
        let angle = reading.get().and_then(magnetic_direction);
        let ink = Color::rgba(0.5, 0.5, 0.55, 1.0);
        let point = |a: f64, r: f64| Point::new(c.x + a.sin() * r, c.y - a.cos() * r);
        d.stroke(
            Shape::Ellipse(Rect::new(
                c.x - radius,
                c.y - radius,
                radius * 2.0,
                radius * 2.0,
            )),
            ink,
            1.0,
        );
        // The fixed marker is the phone's top; the rose rotates with the magnetic field.
        d.fill(
            Shape::Polygon(vec![
                Point::new(c.x, c.y - radius - 2.0),
                Point::new(c.x - 4.0, c.y - radius - 10.0),
                Point::new(c.x + 4.0, c.y - radius - 10.0),
            ]),
            ink,
        );
        let Some(angle) = angle else { return };
        for i in 0..24 {
            let a = angle + f64::from(i) * std::f64::consts::TAU / 24.0;
            d.stroke(
                Shape::Line(point(a, radius - 2.0), point(a, radius - 6.0)),
                ink,
                1.0,
            );
        }
        for (i, name) in [
            crate::res::str::sensor_north(),
            crate::res::str::sensor_east(),
            crate::res::str::sensor_south(),
            crate::res::str::sensor_west(),
        ]
        .into_iter()
        .enumerate()
        {
            d.text(
                &name.format(),
                point(
                    angle + i as f64 * std::f64::consts::FRAC_PI_2,
                    radius - 18.0,
                ),
                TextStyle {
                    size: 13.0,
                    color: if i == 0 { crate::palette::CORAL } else { ink },
                    anchor: TextAnchor::CENTERED,
                    ..Default::default()
                },
            );
        }
        let side = angle + std::f64::consts::FRAC_PI_2;
        for (direction, color) in [
            (angle, crate::palette::CORAL),
            (angle + std::f64::consts::PI, ink),
        ] {
            d.fill(
                Shape::Polygon(vec![
                    point(direction, radius - 32.0),
                    point(side, 5.0),
                    point(side + std::f64::consts::PI, 5.0),
                ]),
                color,
            );
        }
        d.fill(
            Shape::Ellipse(Rect::new(c.x - 3.0, c.y - 3.0, 6.0, 6.0)),
            crate::palette::TEAL,
        );
    })
    .a11y(move |a| {
        a.role(Role::Meter)
            .label(crate::res::str::sensor_compass())
            .value(move || {
                if !calibrated.get() {
                    return crate::res::str::sensor_uncalibrated_hint().format();
                }
                reading.get().and_then(magnetic_direction).map_or_else(
                    || crate::res::str::sensor_waiting().format(),
                    |angle| {
                        crate::res::str::sensor_heading(format!(
                            "{:.0}",
                            (-angle.to_degrees()).round().rem_euclid(360.0)
                        ))
                        .format()
                    },
                )
            })
    })
    .id(day::id!(sensor_compass))
    .frame(176.0, 176.0)
}

/// A bubble level driven by the measured x/y acceleration, with one g at the rim.
/// Translational acceleration also moves the bubble: this is not fused device attitude.
fn acceleration_view(reading: Signal<Option<day_part_sensors::SensorReading>>) -> impl Piece {
    canvas(move |d, size| {
        // Canvas records once before layout, when its size is still zero.
        if size.width < 60.0 || size.height < 60.0 {
            return;
        }

        let c = Point::new(size.width / 2.0, size.height / 2.0);
        let radius = size.width.min(size.height) / 2.0 - 12.0;
        let ink = Color::rgba(0.5, 0.5, 0.55, 0.8);
        for r in [radius, radius * 0.5, 5.0] {
            d.stroke(
                Shape::Ellipse(Rect::new(c.x - r, c.y - r, 2.0 * r, 2.0 * r)),
                ink,
                1.0,
            );
        }
        d.stroke(
            Shape::Line(Point::new(c.x - radius, c.y), Point::new(c.x + radius, c.y)),
            ink,
            1.0,
        );
        d.stroke(
            Shape::Line(Point::new(c.x, c.y - radius), Point::new(c.x, c.y + radius)),
            ink,
            1.0,
        );
        if let Some(r) = reading.get() {
            let p = acceleration_position(r);
            let x = c.x + p.x * (radius - 9.0);
            let y = c.y + p.y * (radius - 9.0);
            d.fill(
                Shape::Ellipse(Rect::new(x - 9.0, y - 9.0, 18.0, 18.0)),
                crate::palette::TEAL,
            );
            d.stroke(
                Shape::Ellipse(Rect::new(x - 9.0, y - 9.0, 18.0, 18.0)),
                ink,
                1.0,
            );
        }
    })
    .a11y(|a| {
        a.role(Role::Image)
            .label(crate::res::str::sensor_tilt())
            .hint(crate::res::str::sensor_tilt_hint())
    })
    .id(day::id!(sensor_accel_live))
    .frame(152.0, 152.0)
}

fn acceleration_position(reading: day_part_sensors::SensorReading) -> Point {
    let scale = reading.x.hypot(reading.y).max(9.80665);
    Point::new(reading.x / scale, -reading.y / scale)
}

/// Signed turn-rate gauges: zero at twelve o'clock, ±π rad/s at ±135 degrees.
/// A stationary phone returns to zero; we deliberately do not integrate a drifting attitude.
fn rotation_view(reading: Signal<Option<day_part_sensors::SensorReading>>) -> impl Piece {
    canvas(move |d, size| {
        // Canvas records once before layout, when its size is still zero.
        if size.width < 60.0 || size.height < 60.0 {
            return;
        }

        let names = [
            crate::res::str::sensor_axis_x(),
            crate::res::str::sensor_axis_y(),
            crate::res::str::sensor_axis_z(),
        ];
        let colors = [
            crate::palette::CORAL,
            crate::palette::TEAL,
            crate::palette::VIOLET,
        ];
        let ink = Color::rgba(0.5, 0.5, 0.55, 0.8);
        let radius = (size.width / 6.0 - 8.0).min(42.0);
        let values = reading.get().map(|r| [r.x, r.y, r.z]);
        for i in 0..3 {
            let c = Point::new(size.width * (i as f64 + 0.5) / 3.0, size.height / 2.0);
            let point =
                |angle: f64, r: f64| Point::new(c.x + angle.sin() * r, c.y - angle.cos() * r);
            d.stroke(
                Shape::Ellipse(Rect::new(
                    c.x - radius,
                    c.y - radius,
                    radius * 2.0,
                    radius * 2.0,
                )),
                ink,
                1.0,
            );
            for tick in -3..=3 {
                let angle = f64::from(tick) * std::f64::consts::FRAC_PI_4;
                d.stroke(
                    Shape::Line(point(angle, radius - 2.0), point(angle, radius - 7.0)),
                    ink,
                    1.0,
                );
            }
            d.text(
                &names[i].format(),
                Point::new(c.x, c.y + radius * 0.52),
                TextStyle {
                    size: 12.0,
                    color: colors[i],
                    anchor: TextAnchor::CENTERED,
                    ..Default::default()
                },
            );
            if let Some(values) = values {
                let angle = rotation_angle(values[i]);
                let side = angle + std::f64::consts::FRAC_PI_2;
                d.fill(
                    Shape::Polygon(vec![
                        point(angle, radius - 9.0),
                        point(side, 3.0),
                        point(side + std::f64::consts::PI, 3.0),
                    ]),
                    colors[i],
                );
                d.fill(
                    Shape::Ellipse(Rect::new(c.x - 3.0, c.y - 3.0, 6.0, 6.0)),
                    colors[i],
                );
            }
        }
    })
    .a11y(|a| {
        a.role(Role::Image)
            .label(crate::res::str::sensor_rotation())
            .hint(crate::res::str::sensor_rotation_hint())
    })
    .id(day::id!(sensor_gyro_live))
    .height(106.0)
}

fn rotation_angle(rate: f64) -> f64 {
    (rate / std::f64::consts::PI).clamp(-1.0, 1.0) * std::f64::consts::PI * 0.75
}

/// The chart holds the latest 120 delivered events; the time span depends on the device's rate.
const CHART_SAMPLES: usize = 120;

/// A live x/y/z strip chart: three polylines over a rolling window, newest on the right.
///
/// Unlike `battery_view` below, this does not mirror under a right-to-left locale.
/// A battery is a picture of an object and mirrors with the layout; a time series is a chart whose
/// x axis is time, and time reads left-to-right in a chart regardless of the reading direction of
/// the surrounding text.
fn strip_chart(
    series: Signal<Vec<day_part_sensors::SensorReading>>,
    id: day::ElementId,
) -> impl Piece {
    canvas(move |d, size| {
        if size.width <= 2.0 || size.height <= 2.0 {
            return;
        }
        let samples = series.get();
        // A baseline is drawn even when empty, so the chart reads as "nothing yet" rather than as a
        // rendering failure.
        let mid = size.height / 2.0;
        d.stroke(
            Shape::Line(Point::new(0.0, mid), Point::new(size.width, mid)),
            Color::rgba(0.5, 0.5, 0.55, 0.35),
            1.0,
        );
        if samples.len() < 2 {
            return;
        }
        // Scale to the window's own peak so a gentle signal is still legible; the floor keeps a
        // near-still device from amplifying its noise to full height.
        let peak = samples
            .iter()
            .flat_map(|r| [r.x.abs(), r.y.abs(), r.z.abs()])
            .fold(1.0f64, f64::max);
        let dx = size.width / (CHART_SAMPLES.max(2) - 1) as f64;
        // Right-align the window so new samples enter at the right edge and scroll left.
        let x0 = size.width - dx * (samples.len() - 1) as f64;
        for (axis, color) in [
            (0usize, crate::palette::CORAL),
            (1, crate::palette::TEAL),
            (2, crate::palette::VIOLET),
        ] {
            for i in 1..samples.len() {
                let v = |r: &day_part_sensors::SensorReading| match axis {
                    0 => r.x,
                    1 => r.y,
                    _ => r.z,
                };
                let y = |r: &day_part_sensors::SensorReading| mid - (v(r) / peak) * (mid - 2.0);
                d.stroke(
                    Shape::Line(
                        Point::new(x0 + dx * (i - 1) as f64, y(&samples[i - 1])),
                        Point::new(x0 + dx * i as f64, y(&samples[i])),
                    ),
                    color,
                    1.5,
                );
            }
        }
    })
    .id(id)
    .height(56.0)
}

/// A permission row: its live status, and a button that either asks or opens Settings.
///
/// Raw motion sensors need no permission on iOS or Android (docs/sensors.md), so on most targets
/// this reads `ungated / granted` and the button is hidden. The rows that really prompt are the
/// web on iOS Safari, and HarmonyOS with its declared `ohos.permission.ACCELEROMETER`.
fn permission_row(
    title: day::LocalizedText,
    perm: day_part_permissions::Permission,
    id: day::ElementId,
) -> impl Piece + use<> {
    use day_part_permissions as perms;
    let status = Signal::new(format!(
        "{} / {}",
        perms::gate(perm).label(),
        perms::status(perm).label()
    ));
    let refresh = move || {
        status.set(format!(
            "{} / {}",
            perms::gate(perm).label(),
            perms::status(perm).label()
        ))
    };
    // `can_prompt` decides which affordance to show: asking when the answer is already final would
    // do nothing, and Settings is the only remedy then.
    let can_prompt = perms::can_prompt(perm);
    // Where the platform has no such gate at all, neither asking nor Settings can change anything,
    // so offer no affordance rather than a button that does nothing.
    let gated = perms::gate(perm) != perms::Gate::Absent;
    let action_label = if can_prompt {
        crate::res::str::perm_request()
    } else {
        crate::res::str::perm_open_settings()
    };
    labeled(
        title,
        // Status text plus a Request/Settings button outgrew a phone-portrait row; the
        // button clipped at the panel's edge, so the pair stacks on a Compact window
        // (docs/size-classes.md).
        adaptive((
            label(move || status.get()).id(id.clone()),
            when(
                move || gated,
                move || {
                    button(action_label.clone())
                        .bordered()
                        .action(move || {
                            if can_prompt {
                                // The completion runs on an unspecified thread; a Setter
                                // crosses back.
                                let set = status.setter();
                                perms::request(perm, move |s| {
                                    set.set(format!(
                                        "{} / {}",
                                        perms::gate(perm).label(),
                                        s.label()
                                    ))
                                });
                            } else {
                                perms::open_settings(perm);
                                refresh();
                            }
                        })
                        .id(day::id!(permission_action, id.as_str()))
                },
            ),
        ))
        .spacing(Space::S),
    )
}

/// Live location (docs/location.md): a permission row, a Start/Stop toggle, and the fix itself.
///
/// The subscription is held in the page's scope, so leaving the page stops the GPS rather than
/// leaving it warm behind a closed screen.
fn location_section() -> impl Piece {
    use day_part_location::{Accuracy, LocationError};

    let coords = Signal::new(if day_part_location::is_available() {
        crate::res::str::location_waiting().format()
    } else {
        crate::res::str::location_unavailable().format()
    });
    let altitude = Signal::new(crate::res::str::location_unknown().format());
    let accuracy = Signal::new(crate::res::str::location_unknown().format());
    let running = Signal::new(false);
    // `Rc<RefCell<…>>` because the handle is created in a button action and dropped in another,
    // both on the UI thread, so no lock is needed.
    let watch: std::rc::Rc<std::cell::RefCell<Option<day_part_location::Watch>>> =
        std::rc::Rc::new(std::cell::RefCell::new(None));
    let held = watch.clone();
    Scope::current().on_cleanup(move || {
        held.borrow_mut().take();
    });

    section((
        permission_row(
            crate::res::str::location_permission(),
            day_part_permissions::Permission::Location,
            day::id!(location_perm),
        ),
        labeled(
            crate::res::str::nav_location(),
            label(move || coords.get())
                .selectable()
                .id(day::id!(location_coords)),
        ),
        labeled(
            crate::res::str::location_altitude(),
            label(move || altitude.get())
                .selectable()
                .id(day::id!(location_altitude)),
        ),
        labeled(
            crate::res::str::location_accuracy(),
            label(move || accuracy.get())
                .selectable()
                .id(day::id!(location_accuracy)),
        ),
        button(move || {
            if running.get() {
                crate::res::str::location_stop().format()
            } else {
                crate::res::str::location_start().format()
            }
        })
        .bordered()
        .action(move || {
            if running.get() {
                watch.borrow_mut().take();
                running.set(false);
                return;
            }
            let (set_coords, set_alt, set_acc) =
                (coords.setter(), altitude.setter(), accuracy.setter());
            let handle = day_part_location::watch(Accuracy::Balanced, move |fix| match fix {
                Ok(f) => {
                    set_coords.set(
                        crate::res::str::location_coords(crate::res::str::LocationCoords {
                            lat: format!("{:.5}", f.latitude),
                            lon: format!("{:.5}", f.longitude),
                        })
                        .format(),
                    );
                    set_alt.set(f.altitude.map_or_else(
                        || crate::res::str::location_unknown().format(),
                        |a| format!("{a:.0} m"),
                    ));
                    set_acc.set(f.accuracy_m.map_or_else(
                        || crate::res::str::location_unknown().format(),
                        |a| format!("±{a:.0} m"),
                    ));
                }
                // A denial is the interesting error: it is what the permission row above is for.
                Err(LocationError::PermissionDenied) => {
                    set_coords.set(crate::res::str::location_permission().format())
                }
                Err(e) => set_coords.set(e.to_string()),
            });
            *watch.borrow_mut() = Some(handle);
            running.set(true);
        })
        .id(day::id!(location_toggle)),
    ))
    .title(crate::res::str::nav_location())
}

fn device_section() -> impl Piece {
    // Read the device identity once now (headless day-part-deviceinfo); Refresh re-polls it.
    let (m, s, sim) = deviceinfo_lines();
    let model = Signal::new(m);
    let system = Signal::new(s);
    let simulator = Signal::new(sim);
    section((
        label(move || model.get())
            .selectable()
            .id(day::id!(deviceinfo_model)),
        label(move || system.get())
            .selectable()
            .id(day::id!(deviceinfo_system)),
        label(move || simulator.get())
            .selectable()
            .id(day::id!(deviceinfo_simulator)),
        button(crate::res::str::deviceinfo_refresh())
            .bordered()
            .action(move || {
                let (m, s, sim) = deviceinfo_lines();
                model.set(m);
                system.set(s);
                simulator.set(sim);
            })
            .id(day::id!(deviceinfo_refresh)),
    ))
    .title(crate::res::str::nav_deviceinfo())
}

/// Draw a battery on a canvas: rounded body + terminal nub, a level fill colored by band
/// (red < 20% ≤ amber < 50% ≤ green), a lightning bolt when charging, and a percent caption.
fn battery_view(level: Signal<f64>, charging: Signal<bool>) -> impl Piece {
    canvas(move |d, size| {
        if size.width <= 0.0 || size.height <= 0.0 {
            return;
        }
        // RTL (docs/localization): the layout engine mirrors widget *placement*, but a canvas draws
        // in its own coordinate space, so this custom drawing mirrors itself. Under a right-to-left
        // locale (e.g. `ar`) the battery flips horizontally: terminal nub on the left, charge
        // draining from the right. `mx` mirrors an x, `mrect` a rect; both are the identity in LTR.
        let rtl = is_rtl();
        let mx = |x: f64| if rtl { size.width - x } else { x };
        let mrect = |r: Rect| {
            if rtl {
                Rect::new(
                    size.width - r.max_x(),
                    r.min_y(),
                    r.size.width,
                    r.size.height,
                )
            } else {
                r
            }
        };
        let frac = (level.get() / 100.0).clamp(0.0, 1.0);
        let band = if frac < 0.2 {
            Color::hex(0xFF3B30) // red
        } else if frac < 0.5 {
            Color::hex(0xFF9F0A) // amber
        } else {
            Color::hex(0x34C759) // green
        };
        let outline = Color::rgba(0.55, 0.55, 0.6, 0.9);

        // Geometry (defined LTR; mirrored at draw time via `mrect`/`mx`). The body fills the canvas
        // minus the terminal nub past its trailing edge and a caption strip below.
        let caption_h = 26.0;
        let nub_w = (size.width * 0.05).clamp(6.0, 14.0);
        let body = Rect::new(
            2.0,
            2.0,
            size.width - nub_w - 6.0,
            size.height - caption_h - 4.0,
        );
        let nub_h = body.size.height * 0.4;
        let nub = Rect::new(
            body.max_x() + 2.0,
            body.center().y - nub_h / 2.0,
            nub_w,
            nub_h,
        );
        d.stroke(Shape::RoundedRect(mrect(body), 12.0), outline, 3.0);
        d.fill(Shape::RoundedRect(mrect(nub), 3.0), outline);

        // The charge fill, inset within the body and clipped to the level fraction. It grows from
        // the leading edge, so under RTL `mrect` makes it drain from the right.
        let well = body.inset(6.0);
        let fill_w = well.size.width * frac;
        if fill_w > 0.5 {
            let fill_rect = Rect::new(well.min_x(), well.min_y(), fill_w, well.size.height);
            d.fill(
                Shape::RoundedRect(mrect(fill_rect), 7.0_f64.min(fill_w / 2.0)),
                band,
            );
        }

        // Charging: a lightning bolt centered in the body (white with a dark edge, so it reads on
        // both the colored fill and the empty well).
        if charging.get() {
            let c = body.center();
            let (bw, bh) = (body.size.height * 0.42, body.size.height * 0.72);
            let p = |rx: f64, ry: f64| {
                Point::new(mx(c.x - bw / 2.0 + rx * bw), c.y - bh / 2.0 + ry * bh)
            };
            let bolt = vec![
                p(0.62, 0.0),
                p(0.0, 0.58),
                p(0.42, 0.58),
                p(0.38, 1.0),
                p(1.0, 0.42),
                p(0.58, 0.42),
            ];
            d.fill(
                Shape::Polygon(bolt.clone()),
                Color::rgba(1.0, 1.0, 1.0, 0.95),
            );
            d.stroke(Shape::Polygon(bolt), Color::rgba(0.0, 0.0, 0.0, 0.35), 1.5);
        }

        // Percent caption below the battery, in the band color.
        d.text(
            &format!("{:.0}%", level.get()),
            Point::new(size.width / 2.0, size.height - caption_h / 2.0),
            TextStyle {
                size: 16.0,
                color: band,
                anchor: TextAnchor::CENTERED,
                ..Default::default()
            },
        );
    })
    // Accessibility (§13): like the gauge, the canvas gets an explicit Meter role and a spoken
    // label and value; the value reads the signal, so the screen reader follows the charge.
    .a11y(move |a| {
        a.role(Role::Meter)
            .label(crate::res::str::nav_battery())
            .value(move || format!("{:.0}%", level.get()))
    })
    .id(day::id!(battery))
    .frame(260.0, 120.0)
}

/// Read the native device identity and format each field as a localized line:
/// `(model, "name version", simulator)`. Values vary by host, so nothing is asserted exactly.
fn deviceinfo_lines() -> (String, String, String) {
    let d = day_part_deviceinfo::get();
    let model = crate::res::str::deviceinfo_model(d.model).format();
    let system = crate::res::str::deviceinfo_system(crate::res::str::DeviceinfoSystem {
        name: d.system_name,
        version: d.system_version,
    })
    .format();
    // Each branch is a full literal tr(...) call so `day lint` sees both keys (never tr(if ...)).
    let sim_value = if d.is_simulator {
        crate::res::str::deviceinfo_yes().format()
    } else {
        crate::res::str::deviceinfo_no().format()
    };
    let simulator = crate::res::str::deviceinfo_simulator(sim_value).format();
    (model, system, simulator)
}

#[cfg(test)]
mod sensor_tests {
    use super::{acceleration_position, magnetic_direction, rotation_angle};
    use day_part_sensors::SensorReading;

    #[test]
    fn live_gauges_center_at_rest_and_clamp_without_changing_direction() {
        let center = acceleration_position(SensorReading {
            x: 0.0,
            y: 0.0,
            z: 9.80665,
        });
        assert_eq!((center.x, center.y), (0.0, 0.0));
        let edge = acceleration_position(SensorReading {
            x: 30.0,
            y: -40.0,
            z: 0.0,
        });
        assert!((edge.x - 0.6).abs() < 1e-9 && (edge.y - 0.8).abs() < 1e-9);
        assert_eq!(rotation_angle(0.0), 0.0);
        assert_eq!(rotation_angle(20.0), std::f64::consts::PI * 0.75);
        assert_eq!(rotation_angle(-20.0), -rotation_angle(20.0));
    }

    #[test]
    fn compass_uses_portrait_xy_axes_and_rejects_a_vertical_field() {
        for (x, y, expected) in [
            (0.0, 40.0, 0.0),
            (40.0, 0.0, 90.0),
            (0.0, -40.0, 180.0),
            (-40.0, 0.0, -90.0),
        ] {
            let angle = magnetic_direction(SensorReading { x, y, z: 25.0 }).unwrap();
            assert!((angle.to_degrees() - expected).abs() < 1e-9);
        }
        assert!(
            magnetic_direction(SensorReading {
                x: 0.0,
                y: 0.0,
                z: 40.0
            })
            .is_none()
        );
    }
}
