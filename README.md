# ClearDevil

A third-party control implementation of YUU(Yes-YouYoung) Little Devil Fully Automatic Male Masturbator Base on Rust+Actix Web

## Features

- Auto connect your Little Devil Device
- Classic mode control (10 preset modes)
- Munual mode control
- 🌐 RESTful API 

## Requirements

- Rust 1.70+ 
- Linux/Windows/MacOS
- BlueZ Stack

### Install requirements

```bash
# Ubuntu/Debian
sudo apt-get install libbluetooth-dev pkg-config

# Fedora
sudo dnf install bluez-devel pkg-config

# Arch Linux
sudo pacman -S bluez pkgconf
```

## Build

```bash
# build this project
cargo build --release

# and run
cargo run
```

When the program starts, it will:
1. Automatically scan for devices with name "LD-1"(The Little Devil  Device)
2. connect to the device
3. sent init command `c3 6f 36 62 4f`
4. Start the server on `http://127.0.0.1:8080`

## API

### PING
```
GET /health
```

### Classic Mode
```
GET /classicmode?type=[1-10]
```

**Query:**
- `type`: Mode type query (1-10)

**Mode List:**
| Type | ModeName(Chinese Only) | BLE Command |
|------|---------|------|
| 1 | 发射基地 | `c3 7f 25 63 61` |
| 2 | 星战前夜 | `c3 7f 25 60 6e` |
| 3 | G计划 | `c3 7f 25 61 6f` |
| 4 | 赛博女郎 | `c3 7f 25 66 6c` |
| 5 | 霓虹艺女 | `c3 7f 25 67 6d` |
| 6 | 2077曼哈顿 | `c3 7f 25 64 6a` |
| 7 | 机械裸露 | `c3 7f 25 65 6b` |
| 8 | 超现实互动 | `c3 7f 25 6a 68` |
| 9 | 开火!开火! | `c3 7f 25 6b 69` |
| 10 | 爆发 | `c3 7c 25 76 5d` |

**Example:**
```bash
curl "http://127.0.0.1:8080/classicmode?type=1"
```

### Munual Mode
```
GET /manualmode?type=[A|B]&intensity=[0-20]
```

**Query:**
- `type`: Control type
  - `A`: Rotational
  - `B`: Suck
- `intensity`: Intensity level (0-20)

**Example:**
```bash
# Set rotation intensity A to 10.
curl "http://127.0.0.1:8080/manualmode?type=A&intensity=10"

# Suction intensity B is set to 15
curl "http://127.0.0.1:8080/manualmode?type=B&intensity=15"
```

### Stop
```
GET /stop
```

Send BLE command `c3 7e 25 62 63`

**Example:**
```bash
curl "http://127.0.0.1:8080/stop"
```

## 响应格式

Successful response:
```json
{
  "status": "success",
  "mode": 1,
  "command": "[c3, 7f, 25, 63, 61]"
}
```

Error response:
```json
{
  "error": "xxxx"
}
```
## Notes

Permissions: Running the program requires Bluetooth access permissions, which may require root or dialout group privileges.
sudo usermod -aG dialout $USER


Bluetooth Adapter: Ensure the system has an available Bluetooth adapter and that it is enabled.

Device Range: Ensure the LD-1 device is within the effective Bluetooth range (typically within 10 meters).

First Connection: The program will automatically scan and connect to the device upon startup. Please ensure the device is powered on.

Troubleshooting

Bluetooth adapter not found
Check if the Bluetooth service is running: systemctl status bluetooth
Start the Bluetooth service: sudo systemctl start bluetooth

Device LD-1 not found
Confirm the device is powered on and in discoverable mode.
Try restarting the device.

Connection failed
Ensure the device is not occupied by other applications.
Check system Bluetooth logs: sudo journalctl -u bluetooth -f

Tech Stack

Actix-web 4: Web framework
btleplug 0.11: Bluetooth LE library
Tokio: Async runtime
Serde: Serialization/Deserialization

## License

MIT
