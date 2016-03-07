RUST_CHANNEL ?= nightly

CRATES = matches show text_writer triable return_if_ok string-wrapper

# FIXME: Make this unconditional when 1.8 hits the stable channel.
# ref_filter_map uses Ref::map which is stable since 1.8
ifneq "$(RUST_CHANNEL)" "stable"
    CRATES += ref_filter_map
endif

ifeq "$(RUST_CHANNEL)" "nightly"
    CRATES += zip_longest
endif

# Unmaintained: mod_path

.PHONY: default
default: test

define ALL

.PHONY: $(1)
$(1): $(addprefix $(1)-,$(CRATES))
$(foreach crate,$(CRATES),$(eval $(call ONE,$(1),$(crate))))

endef

define ONE

.PHONY: $(1)-$(2)
$(1)-$(2):
	cargo $(1) --manifest-path $(2)/Cargo.toml

endef

$(foreach command,test build clean publish,$(eval $(call ALL,$(command))))
