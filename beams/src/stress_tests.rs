// SPDX-License-Identifier: MIT

//! Property stress: hostile dt/dims through the shared harness.

use crate::beams::Beams;
use idle_api::stress::stress_saver;

#[test]
fn stress_survives_hostile_frames() {
    let mut s = Beams::new();
    stress_saver(&mut s, 1000, 0xC0FFEE);
}
