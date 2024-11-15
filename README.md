# FPGA Learning Project with Rust-HDL

This repository is designed for learning FPGA development using Rust-HDL on the Nandland Go Board (iCE40HX1K VQ100). It includes Rust source files, a Makefile for automation, and a [go.pcf](go.pcf) constraint file to facilitate the design and programming process.

## Prerequisites

To set up the environment (using nix), run the following command:

```sh
nix-shell -p nextpnr yosys icestorm gnumake
```

This command will provide all the necessary dependencies to execute the Makefile, including:

- **yosys**: for synthesis
- **nextpnr**: for place and route
- **icestorm**: for generating the bitstream
- **make**: for executing build commands

## Makefile Targets

- **make**: Builds the project, including synthesis, placement, routing, and bitstream generation.
- **make upload**: Writes the generated binary to the FPGA.

## License

This project is released under the MIT License.

