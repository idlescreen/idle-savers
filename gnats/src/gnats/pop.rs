// SPDX-License-Identifier: MIT

use super::types::{Attractor, Firefly, Star};
use crate::runner::LcgRng;
use crate::runner::hsl_to_rgb;

pub fn spawn_firefly(
    fireflies: &mut Vec<Firefly>,
    rng: &mut LcgRng,
    cols: usize,
    rows: usize,
    accent: (u8, u8, u8),
) {
    let (cx, cy) = if crate::runner::is_secondary_monitor() {
        (cols as f32 / 2.0, rows as f32 / 2.0)
    } else {
        let primary = crate::runner::get_primary_monitor_bounds(cols, rows);
        (
            (primary.start_col + primary.width() / 2) as f32,
            (primary.start_row + primary.height() / 2) as f32,
        )
    };

    let border = rng.next_range(0.0, 4.0) as u32;
    let (x, y) = match border {
        0 => (rng.next_range(0.0, cols as f32), 0.0),
        1 => (cols as f32, rng.next_range(0.0, rows as f32)),
        2 => (rng.next_range(0.0, cols as f32), rows as f32),
        _ => (0.0, rng.next_range(0.0, rows as f32)),
    };

    let dx = cx - x;
    let dy = cy - y;
    let len = (dx * dx + dy * dy).sqrt().max(0.1);
    let speed = rng.next_range(12.0, 24.0);

    let color_roll = rng.next_range(0.0, 1.0);
    let (acc_h, _acc_s, _acc_l) = crate::runner::rgb_to_hsl(accent.0, accent.1, accent.2);
    let color = if color_roll < 0.45 {
        accent
    } else if color_roll < 0.72 {
        hsl_to_rgb((acc_h + 120.0).rem_euclid(360.0), 0.95, 0.60)
    } else {
        hsl_to_rgb((acc_h - 120.0).rem_euclid(360.0), 0.95, 0.60)
    };

    let size = rng.next_range(0.0, 4.0) as u8;
    let speed_mult = rng.next_range(0.7, 1.3);

    fireflies.push(Firefly {
        x,
        y,
        vx: (dx / len) * speed,
        vy: (dy / len) * speed,
        color,
        size,
        speed_mult,
        history: Vec::new(),
        blink_phase: rng.next_range(0.0, std::f32::consts::TAU),
        blink_rate: rng.next_range(1.6, 3.8),
    });
}

pub fn create_attractors(
    attractors: &mut Vec<Attractor>,
    cols: usize,
    rows: usize,
    accent: (u8, u8, u8),
) {
    attractors.clear();
    let (acc_h, _acc_s, _acc_l) = crate::runner::rgb_to_hsl(accent.0, accent.1, accent.2);
    let (cx, cy) = if crate::runner::is_secondary_monitor() {
        (cols as f32 / 2.0, rows as f32 / 2.0)
    } else {
        let primary = crate::runner::get_primary_monitor_bounds(cols, rows);
        (
            (primary.start_col + primary.width() / 2) as f32,
            (primary.start_row + primary.height() / 2) as f32,
        )
    };

    attractors.push(Attractor {
        x: cx,
        y: cy,
        color: accent,
        phase: 0.0,
        speed: 0.6,
    });
    attractors.push(Attractor {
        x: cx,
        y: cy,
        color: hsl_to_rgb((acc_h + 120.0).rem_euclid(360.0), 0.95, 0.60),
        phase: 2.0,
        speed: 0.45,
    });
    attractors.push(Attractor {
        x: cx,
        y: cy,
        color: hsl_to_rgb((acc_h - 120.0).rem_euclid(360.0), 0.95, 0.60),
        phase: 4.0,
        speed: 0.75,
    });
}
