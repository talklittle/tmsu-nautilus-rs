# installation paths
INSTALL_DIR=$(DESTDIR)/usr/lib/nautilus/extensions-3.0

# other vars
VER=$(shell grep version Cargo.toml | head -1 | awk '{print $$3}' | tr -d '"' | tr -d "\n")
SHELL=/bin/sh
ARCH=$(shell uname -m)
DIST_NAME=tmsu-nautilus-$(ARCH)-$(VER)
DIST_DIR=$(DIST_NAME)
DIST_FILE=$(DIST_NAME).tgz
DIST_SO_FILENAME=libtmsu-nautilus.so
BUILD_SO_FILENAME=libtmsu_nautilus.so

all: clean test compile dist

clean:
	@echo
	@echo "CLEANING"
	@echo
	cargo clean
	rm -Rf $(DIST_DIR)
	rm -f $(DIST_FILE)

compile:
	@echo
	@echo "COMPILING"
	@echo
	cargo build --release
	chmod a-x target/release/$(BUILD_SO_FILENAME)
	strip target/release/$(BUILD_SO_FILENAME)

test: unit-test

unit-test:
	@echo
	@echo "RUNNING UNIT TESTS"
	@echo
	cargo test

dist: compile
	@echo
	@echo "PACKAGING DISTRIBUTABLE"
	@echo
	@mkdir -p $(DIST_DIR)
	cp target/release/$(BUILD_SO_FILENAME) -T $(DIST_DIR)/$(DIST_SO_FILENAME)
	cp README.md -t $(DIST_DIR)
	cp COPYING.txt -t $(DIST_DIR)
	tar czf $(DIST_FILE) $(DIST_DIR)

install:
	@echo
	@echo "INSTALLING"
	@echo
	mkdir -p $(INSTALL_DIR)
	cp target/release/$(BUILD_SO_FILENAME) -T $(INSTALL_DIR)/$(DIST_SO_FILENAME)

uninstall:
	@echo "UNINSTALLING"
	rm $(INSTALL_DIR)/$(DIST_SO_FILENAME)
