# nvctl

CLI application to manage the discrete Nvidia GPU in Optimus laptops.

## Background

I wanted a way to switch the GPU in use without needing to enter a
password.

## Usage

Build (`cargo build --release`), then install with `sudo make install`.

The intended usecase for nvctl is for it to be installed as a SUID
binary and owned by root so that normal users can use it to control the
GPU (e.g. with [nv-applet][]).

[nv-applet]: https://github.com/sersorrel/nv-applet

## Bugs

Hopefully not; nvctl is by and large a wrapper around other tools,
namely bbswitch and prime-select. (It would have been a shell script,
but for security reasons, the SUID bit is typically ignored on
interpreted scripts.)

Nevertheless, if you find any bugs, please open an issue.

## Contributing

Please do, if you can think of anything to add or improve.

## Copyright

Copyright © 2019 Ash Holland. Licensed under the EUPL (1.2 or later).
