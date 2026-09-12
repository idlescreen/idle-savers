use super::*;
use crate::runner::Screensaver;
use crate::runner::TerminalCell;
use std::sync::Mutex;
use std::time::Duration;

/// Process-global env races under parallel tests.
/// Host reads `IDLE_SECONDARY_MONITOR` (see idle-runner sys_info).
static MONITOR_ENV_LOCK: Mutex<()> = Mutex::new(());

fn with_primary_monitor<R>(f: impl FnOnce() -> R) -> R {
    let _g = MONITOR_ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    unsafe {
        std::env::remove_var("IDLE_SECONDARY_MONITOR");
        std::env::remove_var("TRANCE_SECONDARY_MONITOR");
    }
    f()
}

fn with_secondary_monitor<R>(f: impl FnOnce() -> R) -> R {
    let _g = MONITOR_ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    unsafe {
        std::env::set_var("IDLE_SECONDARY_MONITOR", "1");
    }
    let out = f();
    unsafe {
        std::env::remove_var("IDLE_SECONDARY_MONITOR");
    }
    out
}

#[test]
fn test_beams_new() {
    let b = Beams::new();
    assert!(!b.spotlights.is_empty());
    assert_eq!(b.last_cols, 0);
    assert_eq!(b.last_rows, 0);
}

#[test]
fn test_beams_init() {
    let mut b = Beams::new();
    b.update(Duration::from_millis(16), 80, 24);
    assert_eq!(b.last_cols, 80);
    assert_eq!(b.last_rows, 24);
    assert!(!b.stars.is_empty());
    assert!(!b.particles.is_empty());
}

#[test]
fn test_beams_update_and_draw() {
    let mut b = Beams::new();
    b.init(80, 24);
    b.update(Duration::from_millis(16), 80, 24);
    let mut grid = vec![TerminalCell::default(); 80 * 24];
    b.draw(&mut grid, 80, 24);
    let drawn_count = grid.iter().filter(|c| c.ch != '\0').count();
    assert!(drawn_count > 0, "No beams/particles drawn in the grid");
}

#[test]
fn test_math_light_falloff() {
    with_primary_monitor(|| {
        let spot = Spotlight {
            origin_x_ratio: 0.5,
            color_r: 255.0,
            color_g: 255.0,
            color_b: 255.0,
            angle_center: std::f32::consts::FRAC_PI_2,
            angle_amplitude: 0.0,
            phase: 0.0,
            phase_offset: 0.0,
            speed: 1.0,
            spread: 0.5,
            speed_bias: 1.0,
            motion_blend: 1.0,
            motion_timer: 5.0,
            is_calm: false,
        };
        let spotlights = vec![spot];
        let current_angles = vec![std::f32::consts::FRAC_PI_2];
        let half = 0.5_f32;
        let a_min = std::f32::consts::FRAC_PI_2 - half;
        let a_max = std::f32::consts::FRAC_PI_2 + half;
        let spot_cots = vec![(
            a_min,
            a_max,
            a_min.cos() / a_min.sin(),
            a_max.cos() / a_max.sin(),
            1.0 / half,
        )];

        let ctx = super::light::LightContext::new(80, 24, &spotlights);
        let (_, _, _, near) = super::light::get_light_at(
            40.0,
            22.0,
            &ctx,
            &spotlights,
            &current_angles,
            &spot_cots,
            (255, 200, 120),
            false,
        );
        let (_, _, _, far) = super::light::get_light_at(
            40.0,
            4.0,
            &ctx,
            &spotlights,
            &current_angles,
            &spot_cots,
            (255, 200, 120),
            false,
        );

        assert!(
            near > far,
            "near origin intensity ({near}) should exceed far intensity ({far})"
        );
        assert!(
            near > 0.0,
            "expected non-zero intensity near origin, got {near}"
        );
    });
}

#[test]
fn test_math_light_angle_boundary() {
    with_primary_monitor(|| {
        let spot = Spotlight {
            origin_x_ratio: 0.5,
            color_r: 255.0,
            color_g: 255.0,
            color_b: 255.0,
            angle_center: std::f32::consts::FRAC_PI_2,
            angle_amplitude: 0.0,
            phase: 0.0,
            phase_offset: 0.0,
            speed: 1.0,
            spread: 0.1,
            speed_bias: 1.0,
            motion_blend: 1.0,
            motion_timer: 5.0,
            is_calm: false,
        };
        let spotlights = vec![spot];
        let current_angles = vec![std::f32::consts::FRAC_PI_2];
        let spot_cots = vec![(
            std::f32::consts::FRAC_PI_2 - 0.1,
            std::f32::consts::FRAC_PI_2 + 0.1,
            (std::f32::consts::FRAC_PI_2 - 0.1).cos() / (std::f32::consts::FRAC_PI_2 - 0.1).sin(),
            (std::f32::consts::FRAC_PI_2 + 0.1).cos() / (std::f32::consts::FRAC_PI_2 + 0.1).sin(),
            1.0 / 0.1,
        )];

        let ctx = super::light::LightContext::new(80, 24, &spotlights);
        let (_, _, _, i_center) = super::light::get_light_at(
            40.0,
            20.0,
            &ctx,
            &spotlights,
            &current_angles,
            &spot_cots,
            (255, 200, 120),
            false,
        );
        let (_, _, _, i_offside) = super::light::get_light_at(
            20.0,
            20.0,
            &ctx,
            &spotlights,
            &current_angles,
            &spot_cots,
            (255, 200, 120),
            false,
        );

        assert!(i_center > 0.0);
        assert_eq!(i_offside, 0.0);
    });
}

#[test]
fn test_secondary_monitor_rendering() {
    with_secondary_monitor(|| {
        let mut b = Beams::new();
        b.init(80, 24);
        for _ in 0..40 {
            b.update(Duration::from_millis(20), 80, 24);
        }
        let mut grid = vec![TerminalCell::default(); 80 * 24];
        b.draw(&mut grid, 80, 24);

        for (i, cell) in grid.iter().enumerate() {
            assert_eq!(
                cell.bg,
                (0, 0, 0),
                "Cell at index {} has non-black background {:?}",
                i,
                cell.bg
            );
        }

        let drawn_chars = grid.iter().filter(|c| c.ch != ' ' && c.ch != '\0').count();
        assert!(
            drawn_chars > 0,
            "Stars and dust should be drawn on secondary monitor"
        );
    });
}

#[test]
fn test_calm_never_freezes_all_beams() {
    let mut b = Beams::new();
    b.init(80, 24);
    for spot in &mut b.spotlights {
        spot.motion_timer = 0.0;
        spot.is_calm = false;
        spot.motion_blend = 1.0;
    }
    b.update(Duration::from_millis(16), 80, 24);
    let calm = b.spotlights.iter().filter(|s| s.is_calm).count();
    assert!(
        calm <= 2,
        "expected at most 2 calm beams after first wave, got {calm}"
    );
    assert!(
        calm >= 1 || b.spotlights.is_empty(),
        "with free slots, at least one beam should rest when timers fire"
    );

    for _ in 0..500 {
        if b.rng.next_f32() < 0.15 {
            for spot in &mut b.spotlights {
                if !spot.is_calm {
                    spot.motion_timer = 0.0;
                }
            }
        }
        b.update(Duration::from_millis(50), 80, 24);
        let calm = b.spotlights.iter().filter(|s| s.is_calm).count();
        assert!(calm <= 2, "calm cap broken: {calm} beams resting");
        if b.spotlights.len() > 2 {
            assert!(
                calm < b.spotlights.len(),
                "all beams froze at once — looks like a glitch"
            );
        }
    }
}

#[test]
fn update_clamps_huge_dt_after_resume() {
    // A suspend/resume (or long hitch) can deliver a multi-minute dt.
    // update() must clamp the simulation step (<=0.1s + first-frame init)
    // rather than teleporting particles or exhausting rockets/timers.
    let mut saver = Beams::new();
    saver.update(std::time::Duration::from_secs(300), 80, 24);
    assert!(
        saver.time_elapsed < 1.0,
        "time_elapsed advanced by {} on a 300s dt — clamp missing",
        saver.time_elapsed
    );
}
