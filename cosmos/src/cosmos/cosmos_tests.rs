//! Unit tests for Cosmos screensaver.

use super::*;
use crate::runner::Screensaver;
use crate::runner::{LcgRng, TerminalCell};
use std::time::Duration;

#[test]
fn test_cosmos_new() {
    let cosmos = Cosmos::new();
    assert_eq!(cosmos.state, UniverseState::Darkness);
    assert_eq!(cosmos.particles.len(), 0);
    assert_eq!(cosmos.seeds.len(), 0);
}

#[test]
fn test_cosmos_update_and_draw() {
    let mut cosmos = Cosmos::new();
    cosmos.update(Duration::from_millis(16), 80, 24);
    let mut grid = vec![TerminalCell::default(); 80 * 24];
    cosmos.draw(&mut grid, 80, 24);
    assert_eq!(cosmos.last_cols, 80);
    assert_eq!(cosmos.last_rows, 24);
}

#[test]
fn test_coordinate_conversion() {
    let universe_cx = 40.0;
    let universe_cy = 12.0;
    let screen_cx = 40.0;
    let screen_cy = 12.0;
    let zoom = physics::default_zoom(80, 24);

    let test_points = vec![(40.0, 12.0), (10.0, 5.0), (80.0, 24.0), (0.0, 0.0)];
    let round_trip_tol = 0.51 / zoom + 0.01;

    for (ux, uy) in test_points {
        let (sx, sy) =
            physics::to_screen_fast(ux, uy, universe_cx, universe_cy, screen_cx, screen_cy, zoom);
        let rux = universe_cx + (sx as f32 - screen_cx) / zoom;
        let ruy = universe_cy + (sy as f32 - screen_cy) / zoom;
        assert!((ux - rux).abs() < round_trip_tol, "ux {ux} rux {rux}");
        assert!((uy - ruy).abs() < round_trip_tol, "uy {uy} ruy {ruy}");
    }
}

#[test]
fn test_logo_coordinate_round_trip() {
    let universe_cx = 105.0;
    let universe_cy = 28.5;
    let screen_cx = 105.0;
    let screen_cy = 28.5;
    let zoom = 0.58;

    for (gx, gy) in [(60.0, 20.0), (150.0, 40.0), (105.0, 28.5)] {
        let (lp_x, lp_y) = physics::screen_to_logo_universe(
            gx,
            gy,
            universe_cx,
            universe_cy,
            screen_cx,
            screen_cy,
        );
        let (sx, sy) = physics::logo_to_screen_fast(
            lp_x,
            lp_y,
            universe_cx,
            universe_cy,
            screen_cx,
            screen_cy,
        );
        assert!((gx - sx as f32).abs() < 0.51, "gx {gx} sx {sx}");
        assert!((gy - sy as f32).abs() < 0.51, "gy {gy} sy {sy}");

        let (px, py) =
            physics::logo_to_particle_universe(lp_x, lp_y, universe_cx, universe_cy, zoom);
        let (rsx, rsy) =
            physics::to_screen_fast(px, py, universe_cx, universe_cy, screen_cx, screen_cy, zoom);
        assert!((gx - rsx as f32).abs() < 0.51);
        assert!((gy - rsy as f32).abs() < 0.51);
    }
}

#[test]
fn test_lcg_rng() {
    let mut rng = LcgRng::new(42);
    let val1 = rng.next_f32();
    assert!((0.0..1.0).contains(&val1));

    let mut rng2 = LcgRng::new(42);
    let val2 = rng2.next_f32();
    assert_eq!(val1, val2);

    for _ in 0..100 {
        let r = rng.next_range(-5.0, 5.0);
        assert!((-5.0..5.0).contains(&r));
    }
}

#[test]
fn test_hsl_rgb_conversions() {
    use crate::runner::{hsl_to_rgb, rgb_to_hsl};
    let (r, g, b) = hsl_to_rgb(0.0, 1.0, 0.5);
    assert_eq!((r, g, b), (255, 0, 0));
    let (h, s, l) = rgb_to_hsl(r, g, b);
    assert!((h - 0.0).abs() < 1.0);
    assert!((s - 1.0).abs() < 0.01);
    assert!((l - 0.5).abs() < 0.01);
}
