//! Property tests for pure beams math (smoothstep + frame pacing).
// SPDX-License-Identifier: Apache-2.0

use super::pacing::{FramePacing, update_frame_time};
use super::types::smoothstep;
use proptest::prelude::*;
use std::time::Duration;

proptest! {
    /// smoothstep output is always in [0, 1] for finite inputs with edge1 > edge0.
    #[test]
    fn prop_smoothstep_in_unit_interval(
        edge0 in -1000.0f32..1000.0,
        width in 0.001f32..1000.0,
        x in -2000.0f32..2000.0,
    ) {
        let edge1 = edge0 + width;
        let y = smoothstep(edge0, edge1, x);
        prop_assert!(y.is_finite());
        prop_assert!((0.0..=1.0).contains(&y), "y={y}");
    }
}

proptest! {
    /// smoothstep is non-decreasing in x for fixed edges.
    #[test]
    fn prop_smoothstep_monotone_in_x(
        edge0 in -100.0f32..100.0,
        width in 0.01f32..50.0,
        x0 in -200.0f32..200.0,
        dx in 0.0f32..50.0,
    ) {
        let edge1 = edge0 + width;
        let y0 = smoothstep(edge0, edge1, x0);
        let y1 = smoothstep(edge0, edge1, x0 + dx);
        prop_assert!(y1 + 1e-5 >= y0, "y0={y0} y1={y1}");
    }
}

proptest! {
    /// Quality scale stays clamped to [0.20, 1.0] under any pacing inputs.
    #[test]
    fn prop_quality_scale_clamped(
        time_elapsed in 0.0f32..60.0,
        on_battery in any::<bool>(),
        dt_ms in 1u64..200,
        ema in 0.001f32..0.5,
        target in 0.004f32..0.05,
        quality in 0.20f32..1.0,
    ) {
        let mut pacing = FramePacing {
            frame_time_ema: ema,
            target_frame_time: target,
            quality_scale: quality,
        };
        update_frame_time(
            &mut pacing,
            time_elapsed,
            on_battery,
            Duration::from_millis(dt_ms),
        );
        prop_assert!(pacing.quality_scale.is_finite());
        prop_assert!(
            (0.20..=1.0).contains(&pacing.quality_scale),
            "quality_scale={}",
            pacing.quality_scale
        );
        prop_assert!(pacing.frame_time_ema.is_finite());
        prop_assert!(pacing.frame_time_ema >= 0.0);
        prop_assert!(pacing.target_frame_time > 0.0);
    }
}
