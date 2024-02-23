# `bevy_touch_stick`

Virtual touch screen analog joysticks for the [Bevy](https://bevyengine.org/)
game engine.

## Goals

- [x] Support mouse and touch
- [x] Multiple joysticks on screen (for e.g. twin stick)
- [x] Emulate a regular bevy gamepad
- [x] Minimal dependencies (including features)
- [ ] Simple stupid implementation
- [ ] Modular rendering
- [ ] No asset dependencies

Only two-axis sticks are supported.

If you need single-axis sticks, use [SergioRibera/virtual_joystick](https://github.com/SergioRibera/virtual_joystick) instead.

## Examples

- [`leafwing`](./examples/leafwing.rs) (recommended): Shows usage with [`leafwing-input-manager`](https://github.com/Leafwing-Studios/leafwing-input-manager)
- [`simple`](./examples/simple.rs)
- [`multiple`](./examples/multiple.rs)

## Usage

Check out the [examples](./examples).

## Bevy Version Support

The `main` branch targets the latest bevy release.

|bevy|bevy_touch_stick|
|----|----------------|
|0.13| main           |

## License

`bevy_touch_stick` is dual-licensed under either

- MIT License (./LICENSE-MIT or <http://opensource.org/licenses/MIT>)
- Apache License, Version 2.0 (./LICENSE-APACHE or <http://www.apache.org/licenses/LICENSE-2.0>)

at your option.

## Contributions

PRs welcome!

## Acknowledgements

`bevy_touch_stick` was forked from [`virtual_joystick`](https://github.com/SergioRibera/virtual_joystick)
