// SPDX-License-Identifier: MIT

//! Property stress: hostile dt/dims through the shared harness.

use crate::cosmos::Cosmos;
use idle_api::stress::stress_saver;

#[test]
fn stress_survives_hostile_frames() {
    let mut s = Cosmos::new();
    stress_saver(&mut s, 1000, 0xC0FFEE);
}
