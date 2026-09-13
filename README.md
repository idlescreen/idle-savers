# savers

All eleven official IdleScreen screensaver plugins in one workspace, plus
the `idle-savers` bundle (`meta/`). Part of
[IdleScreen](https://idlescreen.github.io) — modular Wayland screensavers
for Linux.

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

Each crate builds a `libscreensaver_<name>.so` cdylib plus an
`.idleplugin.toml` manifest, installed to
`/usr/libexec/idle/screensavers/` by the signed deb/rpm packages.

## Use

```sh
idlescreen savers              # list installed plugins
idlescreen saver set aurora    # pick one
idlescreen preview storm       # fullscreen preview
```

## Develop

Path dependency: a `runtime/` checkout inside this repo (or a symlink to a
sibling clone) provides `idle-api`.

```sh
git clone https://github.com/idlescreen/savers.git && cd savers
git clone https://github.com/idlescreen/runtime runtime    # path dep
cargo test --workspace    # all savers
cargo test -p storm       # one saver
```

## License

Apache-2.0 · © 2026 IdleScreen
