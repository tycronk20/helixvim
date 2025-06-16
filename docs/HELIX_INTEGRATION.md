# Helix Integration Roadmap

This document outlines the high-level steps for replacing Vim with Helix in MacHelix.

## Steps

1. **Add Helix dependencies** in `Cargo.toml`.
2. **Create wrapper modules** under `src/editor/` to expose Helix functionality.
3. **Implement editor state management** in `src/editor/state.rs`.
4. **Hook up macOS bridge** in `src/app` for native integration.
5. **Build the UI layer** using `wgpu` and `winit`.
6. **Run tests** regularly with `just test` and keep code formatted with `just fmt`.

This roadmap will evolve as development progresses.
