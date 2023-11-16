# Public variables
DESTDIR ?=
OUTPUT_DIR ?= out

# Private variables
signaturs = classifier
functions = everything questions german trending
all: build

# Build
build: build/cli build/pwa

# Install
install: install/cli install/pwa

install/cli: $(addprefix install/cli/,$(clis))
$(addprefix install/cli/,$(clis)):
	install/cli -D -m 0755 $(OUTPUT_DIR)/$(subst install/cli/,,$@) $(DESTDIR)$(PREFIX)/bin/$(subst install/cli/,,$@)

install/pwa:
	mkdir -p $(DESTDIR)$(WWWROOT)
	cp -rf $(BUILD_DIR)/* $(DESTDIR)$(WWWROOT)$(WWWPREFIX)

# Uninstall
uninstall: uninstall/cli uninstall/pwa

uninstall/cli: $(addprefix uninstall/cli/,$(clis))
$(addprefix uninstall/cli/,$(clis)):
	rm $(DESTDIR)$(PREFIX)/bin/$(subst uninstall/cli/,,$@)

uninstall/pwa:
	rm -rf $(DESTDIR)$(WWWROOT)$(WWWPREFIX)

# Run
run: run/cli run/pwa

run/cli: $(addprefix run/cli/,$(clis))
$(addprefix run/cli/,$(clis)):
	$(subst run/cli/,,$@) $(ARGS)

run/pwa:
	cd frontend && bun run start

# Test
test: test/cli test/pwa

test/cli:
	go test -timeout 3600s -parallel $(shell nproc) ./...

test/pwa:
	cd frontend && bun run test

# Benchmark
benchmark: benchmark/cli benchmark/pwa

benchmark/cli:
	go test -timeout 3600s -bench=./... ./...

benchmark/pwa:
	cd frontend && bun run test

# Clean
clean: clean/cli clean/pwa

clean/cli:
	rm -rf out pkg/models pkg/signatures/go/* pkg/signatures/rust/*

clean/pwa:
	rm -rf frontend/node_modules frontend/.next frontend/out

# Dependencies
depend: depend/signature depend/classifier

depend/signature: $(addprefix depend/signature/,$(signatures))
$(addprefix depend/signature/,$(signatures)):
	scale --no-telemetry signature generate $(subst depend/signature/,,$@):latest -d ./pkg/signatures/$(subst depend/signature/,,$@)
	mkdir -p $(OUTPUT_DIR) pkg/signatures/$(subst depend/signature/,,$@)/go/guest pkg/signatures/$(subst depend/signature/,,$@)/go/host pkg/signatures/$(subst depend/signature/,,$@)/rust/guest
	scale --no-telemetry signature export local/$(subst depend/signature/,,$@):latest go guest pkg/signatures/$(subst depend/signature/,,$@)/go/guest
	scale --no-telemetry signature export local/$(subst depend/signature/,,$@):latest go host pkg/signatures/$(subst depend/signature/,,$@)/go/host
	scale --no-telemetry signature export local/$(subst depend/signature/,,$@):latest rust guest pkg/signatures/$(subst depend/signature/,,$@)/rust/guest

depend/classifier: $(addprefix depend/classifier/,$(functions))
$(addprefix depend/classifier/,$(functions)):
	mkdir -p $(OUTPUT_DIR)
	scale --no-telemetry function build --release -d ./functions/$(subst depend/classifier/,,$@)
	scale --no-telemetry function export local/$(subst depend/classifier/,,$@):latest $(OUTPUT_DIR)