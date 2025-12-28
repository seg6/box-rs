RUSTC     = rustc
TARGET    = x86_64-unknown-linux-gnu
OUT_DIR   = bin
BINARY    = $(OUT_DIR)/box
SRC       = src/main.rs

LINK_ARGS = -C link-arg=-nostartfiles \
            -C link-arg=-nodefaultlibs \
            -C link-arg=-static

RUSTFLAGS = --target $(TARGET) \
 			--edition 2024 \
			-C opt-level=z \
			-C lto=fat \
            -C codegen-units=1 \
			-C panic=abort \
			-C lto=fat \
			-C strip=symbols \
			$(LINK_ARGS)

all: $(BINARY)

$(BINARY): src/**/*.rs
	@mkdir -p $(OUT_DIR)
	$(RUSTC) $(RUSTFLAGS) $(SRC) -o $(BINARY)

clean:
	rm -rf $(OUT_DIR)

run: $(BINARY)
	./$(BINARY)

.PHONY: all clean run dump
