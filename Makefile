# =====================================
# TigerΔ Automation Tool
# XDP / eBPF Core + Rust Loader
# =====================================

# Network interface to attach XDP
INTERFACE ?= eth0

# Paths
BPF_SRC := src/kernel/tiger_delta_xdp.c
BPF_OBJ := src/kernel/tiger_delta_xdp.o

LOADER_BIN := target/release/tiger_loader

# Toolchain
CLANG ?= clang
LLC   ?= llc

# Compiler flags for eBPF
BPF_CFLAGS := -O2 -Wall -Werror \
              -D__BPF_TRACING__ \
              -target bpf

.PHONY: all build-ebpf build-loader run clean info

# Default target
all: build-ebpf build-loader

# Show config
info:
	@echo "🐅 TigerΔ Build Info"
	@echo " Interface : $(INTERFACE)"
	@echo " BPF Src   : $(BPF_SRC)"
	@echo " BPF Obj   : $(BPF_OBJ)"
	@echo " Loader    : $(LOADER_BIN)"

# =====================================
# Build eBPF / XDP kernel core
# =====================================
build-ebpf:
	@echo "🔧 Compiling XDP eBPF core..."
	$(CLANG) -S $(BPF_CFLAGS) -emit-llvm -c $(BPF_SRC) -o - | \
	$(LLC) -march=bpf -filetype=obj -o $(BPF_OBJ)
	@echo "✅ eBPF bytecode ready: $(BPF_OBJ)"

# =====================================
# Build Rust loader
# =====================================
build-loader:
	@echo "🦀 Building Rust loader..."
	cargo build --release --bin tiger_loader
	@echo "✅ Rust loader built: $(LOADER_BIN)"

# =====================================
# Attach XDP (requires sudo)
# =====================================
run: build-ebpf build-loader
	@echo "🚀 Attaching TigerΔ XDP to interface: $(INTERFACE)"
	sudo ./$(LOADER_BIN) $(INTERFACE)

# =====================================
# Cleanup
# =====================================
clean:
	@echo "🧹 Cleaning build artifacts..."
	rm -f $(BPF_OBJ)
	cargo clean
