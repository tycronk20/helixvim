# MacHelix Configuration

MacHelix provides a native macOS experience for the Helix editor. This document describes how to configure MacHelix.

## Configuration Files

MacHelix uses the following configuration files:

- **Helix Configuration**: `~/.config/helix/config.toml` - Standard Helix configuration
- **MacHelix Configuration**: `~/.config/machelix/config.toml` - MacHelix-specific settings

## Helix-Specific Options

All standard Helix configuration options are supported. See the [Helix documentation](https://docs.helix-editor.com/configuration.html) for details.

## MacHelix-Specific Options

### macOS Integration

| Option | Description | Default |
|--------|-------------|---------|
| `macos.native_menus` | Use macOS native menus | `true` |
| `macos.native_tabs` | Use macOS native tabs | `true` |
| `macos.native_fullscreen` | Use macOS native fullscreen | `true` |
| `macos.use_system_clipboard` | Integrate with system clipboard | `true` |
| `macos.services_menu` | Enable macOS Services menu | `true` |

### Window and UI

| Option | Description | Default |
|--------|-------------|---------|
| `window.remember_size` | Remember window size between sessions | `true` |
| `window.remember_position` | Remember window position | `true` |
| `window.default_width` | Default window width in pixels | `1024` |
| `window.default_height` | Default window height in pixels | `768` |
| `ui.tab_style` | Tab style ("native", "text", "hidden") | `"native"` |
| `ui.toolbar` | Show toolbar | `true` |
| `ui.toolbar_items` | Toolbar items to show | `["new", "open", "save"]` |

### Font and Rendering

| Option | Description | Default |
|--------|-------------|---------|
| `font.family` | Font family | `"SF Mono"` |
| `font.size` | Font size | `13` |
| `font.line_height` | Line height multiplier | `1.2` |
| `font.use_ligatures` | Use font ligatures | `true` |
| `rendering.use_gpu` | Use GPU for rendering | `true` |
| `rendering.vsync` | Enable vertical sync | `true` |
| `rendering.antialiasing` | Text antialiasing level (0-3) | `2` |

### Performance

| Option | Description | Default |
|--------|-------------|---------|
| `performance.idle_fps` | Frame rate when idle | `30` |
| `performance.active_fps` | Frame rate when active | `120` |
| `performance.background_jobs` | Maximum background jobs | `4` |

## Example Configuration

```toml
# ~/.config/helix/config.toml - Helix editor config
theme = "gruvbox"

[editor]
line-number = "relative"
mouse = true
auto-save = true

[editor.cursor-shape]
insert = "bar"
normal = "block"
select = "underline"

[keys]
normal = { ... }
```

```toml
# ~/.config/machelix/config.toml - MacHelix specific config
[macos]
native_menus = true
native_tabs = true
use_system_clipboard = true

[window]
remember_size = true
remember_position = true

[font]
family = "JetBrains Mono"
size = 14
use_ligatures = true

[rendering]
use_gpu = true
vsync = true

[ui]
tab_style = "native"
toolbar = true
toolbar_items = ["new", "open", "save", "undo", "redo"]
```

## Command Line Options

MacHelix supports all standard Helix command line options, plus the following:

```
USAGE:
    machelix [OPTIONS] [FILE]...

OPTIONS:
    -h, --help                     Print help information
    -V, --version                  Print version information
    --headless                     Run in headless mode (no GUI)
    --run-command <COMMAND>        Run command after startup
    --config-file <FILE>           Use specific config file
    --no-native-menus              Disable native menus
```