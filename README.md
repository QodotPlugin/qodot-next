# Qodot Next

## A next-generation successor to Qodot, written in GDNative Rust

Qodot Next is a full rewrite of the original GDNative release of Qodot, and acts as the Godot engine integration for the Quarchitect library.

It's presently in alpha testing stage; the core is there, but needs some polishing and testing before it can be considered done.

This repository contains the GDNative code that exposes Quarchitect functionality to Godot.
It depends on the `quarchitect` crate, and will link against it statically to produce a library that can be loaded via the engine.

### New features

- Natively-parallelized parsing and geometry building processes
- Overhauled editor integration and resource system
- WAD3 support
- Support for taking .map files by godot resource or file path
- Support for searching textures by resource location, global file path or WAD
- Rebuild-on-change support for resource-based maps
