# idle-savers

All eleven official IdleScreen screensaver plugins, in one workspace.

| Saver | Package | Description |
|---|---|---|
| `aurora/` | `idle-saver-aurora` | Aurora borealis curtains over a starfield (`[saver] aurora.*` params) |
| `beams/` | `idle-saver-beams` | Spotlight cones sweeping a rising dust starfield |
| `bursts/` | `idle-saver-bursts` | Firework rockets and particle bursts |
| `chaos/` | `idle-saver-chaos` | Strange-attractor particle chaos |
| `cosmos/` | `idle-saver-cosmos` | Accretion, ignition, and collapse of a tiny universe |
| `glyphs/` | `idle-saver-glyphs` | Falling luminous glyphs |
| `gnats/` | `idle-saver-gnats` | Swarming midges around a light |
| `hearth/` | `idle-saver-hearth` | Fireplace embers (`[saver] hearth.*` params) |
| `radar/` | `idle-saver-radar` | Radar sweep with drifting contacts |
| `ripple/` | `idle-saver-ripple` | Rain ripples on dark water |
| `storm/` | `idle-saver-storm` | Forest rain, lightning, wildlife silhouettes |

Each crate builds a `libscreensaver_<name>.so` cdylib plus a sibling
`.idleplugin.toml` manifest, installed to
`/usr/libexec/idle/screensavers/` by the signed deb/rpm packages. The
`idle-savers` meta-package (in `idlescreen/idle`) depends on all eleven.

## Development

```bash
./bootstrap.sh            # installs deps + symlinks the idle engine checkout
cargo test --workspace    # all savers
cargo test -p storm       # one saver
```

The workspace depends on `idle-api` via a path dependency on the
`idlescreen/idle` engine repo. CI checks it out into `idle/`; locally
`bootstrap.sh` clones or symlinks a sibling `../idle` checkout there.

## Layout

- `<saver>/src` — saver implementation (`cdylib` + tests)
- `<saver>/assets` — pixmap + Windows icon
- `<saver>/libscreensaver_<saver>.idleplugin.toml` — signed-manifest input
- `build-support/` — shared build-script crate (Windows resource embed)
- One `rust-toolchain.toml`, one `Cargo.lock`, one CI — bumps land once.

## Releasing

Tagging `vX.Y.Z` builds all ten savers, packages each as
`idle-saver-<name>` deb+rpm, cosign-signs every artifact, publishes the
GitHub release, and dispatches the `idlescreen/packages` pool import.
All savers share the workspace version (`workspace.package.version`).

History: these crates were imported from the ten `idle-saver-*`
repositories, which are now archived.
