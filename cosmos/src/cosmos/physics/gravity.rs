//! Softened inverse-square gravity and frame-dragging (first-principles, power-law).

/// Acceleration on a body of unit mass toward a source of mass `m` at offset (dx, dy).
/// a = G * m / (r² + ε²)^(3/2) * r⃗
pub fn newtonian_accel(m_source: f32, dx: f32, dy: f32, g: f32, softening: f32) -> (f32, f32) {
    let dist_sq = dx * dx + dy * dy;
    // x^1.5 == x * sqrt(x); powf(1.5) internally goes through exp2/log2
    // and costs ~10x a sqrt in this per-particle-per-source hot loop.
    let softened = dist_sq + softening * softening;
    let denom = softened * softened.sqrt();
    if denom < 1e-6 {
        return (0.0, 0.0);
    }
    let scale = g * m_source / denom;
    (dx * scale, dy * scale)
}

/// Tangential frame-drag from a spinning massive body (black hole accretion).
/// Falls off as 1/r² — faster infall with spiral at intermediate range.
pub fn frame_drag_accel(
    m_source: f32,
    dx: f32,
    dy: f32,
    spin_strength: f32,
    spin_dir: f32,
) -> (f32, f32) {
    let dist_sq = dx * dx + dy * dy;
    if dist_sq < 0.25 {
        return (0.0, 0.0);
    }
    let dist = dist_sq.sqrt();
    let tangent_scale = spin_strength * m_source / dist_sq;
    let tx = -dy / dist;
    let ty = dx / dist;
    (
        tx * tangent_scale * spin_dir,
        ty * tangent_scale * spin_dir * 0.45,
    )
}

/// Combined gravitational + optional spin acceleration toward source.
#[allow(clippy::too_many_arguments)]
pub fn gravity_with_spin(
    m_source: f32,
    dx: f32,
    dy: f32,
    g: f32,
    softening: f32,
    is_spinning: bool,
    spin_strength: f32,
    spin_dir: f32,
) -> (f32, f32) {
    let (mut ax, mut ay) = newtonian_accel(m_source, dx, dy, g, softening);
    if is_spinning {
        let (sx, sy) = frame_drag_accel(m_source, dx, dy, spin_strength, spin_dir);
        ax += sx;
        ay += sy;
    }
    (ax, ay)
}

pub fn mean_kinetic_energy(particles: &[crate::cosmos::types::Particle]) -> f32 {
    if particles.is_empty() {
        return 0.0;
    }
    particles
        .iter()
        .map(|p| p.vx * p.vx + p.vy * p.vy)
        .sum::<f32>()
        / particles.len() as f32
}

pub fn total_active_mass(eff: &crate::cosmos::Cosmos) -> f32 {
    let pm: f32 = eff.particles.iter().map(|p| p.mass).sum();
    let sm: f32 = eff.seeds.iter().filter(|s| s.active).map(|s| s.mass).sum();
    pm + sm
}
