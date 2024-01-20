# what is this

Demo of writing text to the OLED built into one of the ESP32 boards like this: https://www.amazon.com/HiLetgo-ESP-WROOM-32-Bluetooth-Development-Display/dp/B072HBW53G

# setup

```sh
. ~/export-esp.sh
cargo run
```

# board

```sh
$ cargo espflash board-info
Chip type:         esp32 (revision v3.1)
Crystal frequency: 40MHz
Flash size:        4MB
Features:          WiFi, BT, Dual Core, 240MHz, Coding Scheme None
MAC address:       e8:6b:ea:c3:0c:04
```

# reference

- https://docs.rs/ssd1306/latest/ssd1306/
