# Target name for your design
TARGET = fpga-strater

# Toolchain paths
YOSYS = yosys
NEXTPNR = nextpnr-ice40
ICEPACK = icepack
ICEPROG = iceprog
CARGO = cargo

# Files and directories
# Directory where rust-hdl outputs Verilog
BUILD_DIR = build
PCF_FILE = go.pcf          # Pin constraint file
JSON_FILE = $(BUILD_DIR)/$(TARGET).json        # JSON netlist file from Yosys
ASC_FILE = $(BUILD_DIR)/$(TARGET).asc          # ASCII bitstream file from NextPNR
BIN_FILE = $(BUILD_DIR)/$(TARGET).bin          # Binary bitstream file for programming

# FPGA settings for iCE40 HX1K
FPGA_DEVICE = hx1k
# Package type, adjust based on your board
FPGA_PACKAGE = vq100

# Default target
all: $(BIN_FILE)

# Step 0: Create build dir
$(BUILD_DIR):
	@echo "Creating build dir"
	mkdir build

# Step 1: Generate Verilog from rust-hdl
$(BUILD_DIR)/$(TARGET).v: $(BUILD_DIR) src/main.rs
	@echo "Generating Verilog from rust-hdl..."
	$(CARGO) run --release > $(BUILD_DIR)/${TARGET}.v

# Step 2: Synthesize Verilog to JSON netlist with Yosys
$(JSON_FILE): $(BUILD_DIR)/$(TARGET).v
	@echo "Synthesizing Verilog with Yosys..."
	$(YOSYS) -p "synth_ice40 -json $(JSON_FILE)" $(BUILD_DIR)/*.v

# Step 3: Place and Route with NextPNR
$(ASC_FILE): $(JSON_FILE) $(PCF_FILE)
	@echo "Running Place and Route with NextPNR..."
	$(NEXTPNR) --$(FPGA_DEVICE) --package $(FPGA_PACKAGE) --json $(JSON_FILE) --pcf $(PCF_FILE) --asc $(ASC_FILE)

# Step 4: Generate binary bitstream with IcePack
$(BIN_FILE): $(ASC_FILE)
	@echo "Generating binary bitstream with IcePack..."
	$(ICEPACK) $(ASC_FILE) $(BIN_FILE)

# Step 5: Upload to FPGA with IceProg
upload: $(BIN_FILE)
	@echo "Uploading bitstream to FPGA with IceProg..."
	$(ICEPROG) $(BIN_FILE)

# Clean up generated files
clean:
	@echo "Cleaning up generated files..."
	rm -rf build/
# Phony targets
.PHONY: all upload clean

