# Public variables
DESTDIR ?=
OUTPUT_DIR ?= out

# Private variables
signatures = $(notdir $(wildcard signatures/*))
functions = $(notdir $(wildcard functions/*))
all: build

# Build
build: build/function

build/function: $(addprefix build/function/,$(functions))
$(addprefix build/function/,$(functions)):
	mkdir -p $(OUTPUT_DIR)
	scale --no-telemetry function build --release -d ./functions/$(subst build/function/,,$@)
	scale --no-telemetry function export local/$(subst build/function/,,$@):latest $(OUTPUT_DIR)

# Install
install:
	true

# Uninstall
uninstall:
	true

# Run
run:
	true

# Test
test:
	true

# Benchmark
benchmark:
	true

# Clean
clean: clean/signature clean/function

clean/signature:
	rm -rf signatures/*/go/* signatures/*/rust/*

clean/function:
	rm -rf out

# Dependencies
depend: depend/signature

depend/signature: $(addprefix depend/signature/,$(signatures))
$(addprefix depend/signature/,$(signatures)):
	scale --no-telemetry signature generate $(subst depend/signature/,,$@):latest -d ./signatures/$(subst depend/signature/,,$@)
	mkdir -p signatures/$(subst depend/signature/,,$@)/go/guest signatures/$(subst depend/signature/,,$@)/rust/guest
	scale --no-telemetry signature export local/$(subst depend/signature/,,$@):latest go guest signatures/$(subst depend/signature/,,$@)/go/guest
	scale --no-telemetry signature export local/$(subst depend/signature/,,$@):latest rust guest signatures/$(subst depend/signature/,,$@)/rust/guest
