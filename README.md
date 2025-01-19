# FWLEDMatrix
A collection of firmware and interfaces for the Framework LED Matrix input module.

## Usage
The firmware in [rp2040_firmware](/sig_rp2040_firmware/rp2040_firmware) is designed specifically for the Framework LED Matrix input module for the Framework 16 laptop. The firmware must be loaded onto the input module for the interface software to communicate with the LED Matrix. Firmware uploading is currently performed with Arduino IDE. The uploading process can be found in [this](https://jschroedl.com/rgb-start/) blog post by Joe Schroedl, creator of the [RGB LED Matrix](https://github.com/corndog2000/RGB-LED-Matrix-Rust). This firmware is capable of writing both PWM brightness and scale to each LED by their x and y coordinates and as a matrix. It allows the LED Matrix's PWM to be updated by the host computer at a rate of over 100 frames per second.

The interface library in [sig_rp2040_interface](/sig_rp2040_interface) holds both a struct definition and methods for connecting with the LED Matrix input module. The struct can communicate with one LED Matrix. It can send PWM and scale information either separately or together to each LED. Both PWM and scale are stored as matrices which can be updated by a containing program. The communication methods can discover the LED Matrix port automatically or with a given port. The methods perform a check to ensure the discovered LED Matrix is loaded with the correct firmware.

The board program (WIP) in [sig_rp2040_board](/sig_rp2040_board) divides the LED Matrix into three 9x10 boxes for other programs to use as displays. These boxes are intended to be communicated with through the TCP protocol.
