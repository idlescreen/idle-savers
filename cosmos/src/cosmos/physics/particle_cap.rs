use crate::cosmos::Cosmos;

/// Soft particle budget scaled by adaptive quality and battery. Prevents unbounded
/// growth during long accretion phases (merge bursts can add 100+ particles per event).
pub fn particle_budget(eff: &Cosmos) -> usize {
    let bat = if eff.on_battery { 0.55 } else { 1.0 };
    let floor = if eff.on_battery { 120.0 } else { 200.0 };
    (580.0 * eff.quality_scale * bat).max(floor) as usize
}

pub fn can_spawn(eff: &Cosmos, count: usize) -> bool {
    eff.particles.len().saturating_add(count) <= particle_budget(eff)
}

/// Drop oldest, lowest-energy particles when over budget.
pub fn trim_particles(eff: &mut Cosmos) {
    let budget = particle_budget(eff);
    let excess = eff.particles.len().saturating_sub(budget);
    if excess == 0 {
        return;
    }
    eff.particles.sort_by(|a, b| {
        let ea = a.vx * a.vx + a.vy * a.vy;
        let eb = b.vx * b.vx + b.vy * b.vy;
        ea.partial_cmp(&eb).unwrap_or(std::cmp::Ordering::Equal)
    });
    eff.particles.truncate(budget);
}
