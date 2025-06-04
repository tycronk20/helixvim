# HelixVim

HelixVim is a native macOS application that provides the [Helix](https://helix-editor.com/) editing experience with deep macOS integration. Built entirely in Rust, it combines Helix's modern editing capabilities with a native macOS user interface.

## Features

- Modern Helix editing experience in a native macOS application
- Built entirely in Rust for performance and reliability
- Full macOS integration:
  - Native menus and keyboard shortcuts
  - Native tabs and window management
  - Services menu integration
  - Drag and drop support
  - Touch Bar support
- GPU-accelerated rendering for smooth scrolling and editing
- Configurable UI with support for Helix themes
- Language server protocol support via Helix's core

## Installation

### Homebrew

```bash
brew tap tycronk20/helixvim
brew install helixvim
```

### Manual Installation

1. Download the latest release from the [releases page](https://github.com/tycronk20/helixvim/releases)
2. Open the DMG file
3. Drag HelixVim.app to your Applications folder

## Building from Source

### Prerequisites

- Rust toolchain (1.70.0 or later)
- Homebrew (for dependencies)

### Build Steps

1. Clone the repository:
   ```bash
   git clone https://github.com/tycronk20/helixvim.git
   cd helixvim
   ```

2. Install dependencies:
   ```bash
   brew bundle
   ```

3. Run the bootstrap script:
   ```bash
   ./scripts/bootstrap.sh
   ```

4. Build HelixVim:
   ```bash
   just build
   ```

5. The app bundle will be available at `./HelixVim.app`

## Development Workflow

HelixVim uses [just](https://github.com/casey/just) as a command runner. Common tasks:

```bash
just run        # Run the application in development mode
just test       # Run all tests
just fmt        # Format code
just clippy     # Run linter
just bundle     # Create macOS app bundle
just dmg        # Create distributable DMG
```

## Configuration

HelixVim uses Helix's configuration format with additional macOS-specific options. See [CONFIG.md](docs/CONFIG.md) for details.

## Architecture

HelixVim is built as a native Rust application with the following components:

- **Helix Core**: Provides the editing engine and text manipulation
- **macOS Bridge**: Native Cocoa integration via Rust bindings
- **Rendering Engine**: GPU-accelerated text rendering
- **Configuration**: Extended Helix configuration with macOS options

## Contributing

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## License

HelixVim is licensed under the same terms as Helix. See [LICENSE](LICENSE) for details.