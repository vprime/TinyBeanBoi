# TinyBeanBoi

Tiny Games Challenge Entry
https://hackaday.io/contest/196871-tiny-games-challenge

Tiny qualifications: 
- Tiny pocket-size device (Esp32)
Honerable Mentions: 
- "The Classics" We are recreating a Tamagachi with our own twist for modern hardware.
- "Pocket Arcade" Just like a Tamagachi, it should be able to hook to a belt loop, and be carried around.


# Development
## Sources
espup: 
https://docs.esp-rs.org/book/installation/riscv-and-xtensa.html

RS Display code options:
https://github.com/ri-char/rp2040-st7789
https://github.com/x931890193/st7789v
https://github.com/Gussy/st7789v

OG Source:
https://github.com/Xinyuan-LilyGO/TTGO-T-Display/tree/master



## Target Device
Cheap ESP32 devices with displays and at least 2 buttons.
https://www.lilygo.cc/products/lilygo%C2%AE-ttgo-t-display-1-14-inch-lcd-esp32-control-board
https://www.amazon.com/HiLetgo-Display-Bluetooth-Internet-Development/dp/B07X1W16QS/
Device Source
https://github.com/Xinyuan-LilyGO/TTGO-T-Display

Batteries
https://www.amazon.com/MakerFocus-Rechargable-Protection-Insulated-Development/dp/B07CXNQ3ZR/

bottom buttons:
GPIO0 GPIO35

Audio:
Probably 

### Hardware Specifications:
- Chipset: ESPRESSIF-ESP32 240MHz Xtensa single-/dual-core 32-bit LX6 microprocessor
- FLASH: QSPI flash 4MB
- SRAM: 520 kB SRAM
- Button: Reset
- USB to TTL: CH9102
- Modular interface: UART, SPI, SDIO, I2C, LED PWM, TV PWM, I2S, IRGPIO, ADC, capacitor touch sensor, DACLNA pre-amplifier
- Display: IPS ST7789V 1.14 Inch 135 x 240 (260ppi) 
- Working voltage: 2.7V-4.2V
- Working current: About 60MA
- Sleep current: About 120uA
- Working temperature range: -40°C ~ +85°C
- Size Weight: Approx. 51.52*25.04*8.54mm (7.81g)
- Power Supply Specifications:
- Power Supply: USB 5V/1A
- Charging current: 500mA
- Battery: 3.7V lithium battery
- JST Connector: 2Pin 1.25mm
- USB: Type-C
### Wi-Fi:
- Standard: FCC/CE-RED/IC/TELEC/KCC/SRRC/NCC
- Protocol 802.11 b/g/n (802.11n, speed up to 150Mbps) A-MPDU and A-MSDU polymerization, support 0.4μS Protection interval
- Frequency range: 2.4GHz~2.5GHz (2400M~2483.5M)
- Transmit Power: 22dBm
- Communication distance: 300m
### Bluetooth:
- Protocol Meet Bluetooth v4.2BR/EDR and BLE standard
- Radio frequency With -97dBm sensitivity NZIF receiver Class-1,Class-2&Class-3 emitter AFH
- Audio frequency CVSD&SBC audio frequency
### Software specification:
- Wi-Fi Mode: Station/SoftAP/SoftAP+Station/P2P
- Security mechanism: WPA/WPA2/WPA2-Enterprise/WPS
- Encryption Type: AES/RSA/ECC/SHA
- Firmware upgrade UART download/OTA(Through network/host to download and write firmware)
- Software Development: Support cloud server development /SDK for user firmware development
- Networking protocol: IPv4, IPv6, SSL, TCP/UDP/HTTP/FTP/MQTT
- User Configuration: AT + Instruction set, cloud server, android/iOSapp
- OS: FreeRTOS