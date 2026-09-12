// SPDX-License-Identifier: MIT

//! Property stress: hostile dt/dims through the shared harness.

use crate::gnats::Gnats;
use idle_api::stress::stress_saver;

#[test]
fn stress_survives_hostile_frames() {
    let mut s = Gnats::new();
    stress_saver(&mut s, 1000, 0xC0FFEE);
}
