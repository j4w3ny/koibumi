# koibumi

> A Visual Novel Engine written in Rust.

**Notice:** koibumi is still in **early stage**(active development) and some features may not work yet. Please be patient and wait for new update.

## Goal

- Cross-Platform Support
  - Desktop (Windows, MacOS, Linux) (First-class Support)
  - Mobile (Android, iOS) (**In testing**, should be work anyway)
  - Web (WASM) (**In testing**, it should be work)
- Hot-Reload
- Multi-Script Language support (see [RenPy Compatibility](#renpy-compatibility) for more infomation)

## The Koibumi Script

Koibumi use [rhai](https://github.com/rhaiscript/rhai) as the script language in game. It's highly customizable!

For more infomation about the usage of koibumi script, please check out [Script Design](doc/Script-Design.md)

## RenPy Compatibility

Current RenPy script (known as `.rpy`) is partially supported. It's recommended to rewrite your script to `.rhai` for stability and better development experience.

Koibumi support the basic gammar of RenPy script, and most of API should be supported as well.

See [RenPy Support](doc/RenPy-Support.md) to check the supported API and gammar of RenPy script.

## Credits

## Guide To Build

## Author

AnErW
