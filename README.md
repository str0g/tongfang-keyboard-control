# tongfang-keyboard-control

## Getting started
- Clone repostiory with `git clone --depth=1`.
- Check with `lsub` if device identification matches your machine if not adjust `examples/99-keyboard.rules`

## tongfang-keyboard-control
Is application to use advance capabilities of laptop keyboard.

## Supported devices
- XMG Neo 17 EA21
- Tuxedo Stellaris 17 AMD Gen3
- ID 048d:ce00 Integrated Technology Express, Inc. ITE Device(8291)

## Description
Application lets user change color/brightness/light pattern, additionally can be integrated into systemd so desire keyboard behavior is going to be setup during system start.

## Installation
Arch Linux users `makepkg -e` other distribution users follow what is inside `package/PKGBUILD` and `package/hook.install`.

## Usage
`tongfang-keyboard-control --help`

## Contributing
Capture usb communication with `wiresharek` (require `usbmon` kernel module) and put data in request or implement missing stuff your self.

## Whats missing
- Save to bios (VirtualBox with xmg controle center on Windows will do the job)
- Custom color per key
- gamemode

## License
Mozilla Public License Version 2.0

