BUILD_DIR = build
INSTALL_DIR = $(HOME)/.local
MAIN_TARGET = tomorse

compileandrun:
	meson compile -C $(BUILD_DIR)
	cd $(BUILD_DIR);\
	./$(MAIN_TARGET)

compile:
	meson compile -C $(BUILD_DIR)

setup:
	meson setup $(BUILD_DIR) -Dprefix=$(INSTALL_DIR)

install:
	meson install -C $(BUILD_DIR)
