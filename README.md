# `bevy_virtual_stick` (work in progress)

Virtual touch screen analog joysticks for the [Bevy](https://bevyengine.org/)
game engine.

Only two-axis sticks are supported.

## Goals

- [x] Support mouse and touch
- [x] Multiple joysticks on screen (for e.g. twin stick)
- [ ] Simple stupid implementation
- [ ] Modular rendering
- [ ] Integration with bevy (`Res<Input<TouchStick>>`)
- [ ] Integration with leafwing input manager
- [x] Minimal dependencies (including features)
- [ ] No asset dependencies

## Examples

- [`simple`](./examples/simple.rs)
- [`multiple`](./examples/multiple.rs)

## Usage

Check out the [examples](./examples).

## Bevy Version Support

The `main` branch targets the latest bevy release.

|bevy|bevy_touch_stick|
|----|----------------|
|0.11| main           |

## License

`bevy_touch_stick` is dual-licensed under either

- MIT License (./LICENSE-MIT or <http://opensource.org/licenses/MIT>)
- Apache License, Version 2.0 (./LICENSE-APACHE or <http://www.apache.org/licenses/LICENSE-2.0>)

at your option.

## Contributions

PRs welcome!

## Acknowledgements

`bevy_touch_stick` was forked from [`virtual_joystick`](https://github.com/SergioRibera/virtual_joystick)
