# DualSense Haptics Tool

A Rust-based application for controlling haptics, LED colors, and adaptive trigger effects on a PlayStation 5 DualSense controller via USB. Supports command-line, config file, and GUI interaction.

---

## 🔧 Dependencies

### System

* Linux
* `libudev-dev`
* `libx11-dev`, `libxcb-render0-dev`, `libxcb-shape0-dev`, `libxcb-xfixes0-dev`

Install with:

```bash
sudo apt update
sudo apt install libudev-dev libx11-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev
```

### Rust

Install [Rust](https://rustup.rs/) and build the project:

```bash
cargo build --release
```

---

## 🔐 Non-root USB Access

To use the controller as a regular user:

```bash
sudo nano /etc/udev/rules.d/99-dualsense.rules
```

Paste this:

```udev
SUBSYSTEM=="hidraw", ATTRS{idVendor}=="054c", ATTRS{idProduct}=="0ce6", MODE="0666"
```

Then apply:

```bash
sudo udevadm control --reload-rules
sudo udevadm trigger
```

Unplug and reconnect the DualSense controller.

---

## 🚀 Usage

### Launch CLI

```bash
./target/release/dualsense_haptics_tool rigid --start 64 --force 255
```

### Launch GUI

```bash
./target/release/dualsense_haptics_tool --ui
```

---

## ⚙️ Config File Format

You can define a configuration using TOML:

```toml
mode = "slope"
start = 32
end = 128
force = 255
```

Run with:

```bash
./dualsense_haptics_tool --config my_effect.toml
```

You can override values from the command line:

```bash
./dualsense_haptics_tool --config my_effect.toml --force 200
```

---

## 🧾 CLI Parameters

| Parameter  | Description                    |
| ---------- | ------------------------------ |
| `--ui`     | Launches the graphical UI      |
| `--config` | Load settings from a TOML file |
| `--mode`   | off / rigid / pulse / slope    |
| `--start`  | Start position (0-255)         |
| `--force`  | Force value (0-255)            |
| `--end`    | End position for slope (0-255) |

---

## 🧪 Example Commands

```bash
# Rigid effect from halfway
./dualsense_haptics_tool rigid --start 128 --force 255

# Pulse near the top
./dualsense_haptics_tool pulse --start 32 --force 200

# Slope from low to high resistance
./dualsense_haptics_tool slope --start 16 --end 240

# Load from config
./dualsense_haptics_tool --config configs/bow_snap.toml

# Launch GUI
./dualsense_haptics_tool --ui
```

---

## ✅ Features

* Adaptive trigger programming (L2/R2)
* RGB LED control
* CLI, config file, and GUI support
* Dual trigger mirroring

---

## ❌ Limitations

* USB only (no Bluetooth)
* Single-mode triggers only (no combo effects)
* No Windows/macOS support yet

---

## 🧑‍💻 License

MIT or Apache-2.0 — your choice.
