# setup

```sh
. ~/export-esp.sh
cargo run
```

# board

```sh
$ cargo espflash board-info
[2024-01-19T22:50:12Z INFO ] Detected 4 serial ports
[2024-01-19T22:50:12Z INFO ] Ports which match a known common dev board are highlighted
[2024-01-19T22:50:12Z INFO ] Please select a port
[2024-01-19T22:50:15Z INFO ] Serial port: '/dev/cu.usbserial-210'
[2024-01-19T22:50:15Z INFO ] Connecting...
[2024-01-19T22:50:15Z INFO ] Using flash stub
Chip type:         esp32 (revision v3.1)
Crystal frequency: 40MHz
Flash size:        4MB
Features:          WiFi, BT, Dual Core, 240MHz, Coding Scheme None
MAC address:       e8:6b:ea:c3:0c:04
```

# reference

- https://docs.rs/ssd1306/latest/ssd1306/
