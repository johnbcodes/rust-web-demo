# Usage:
#
# Do any modifications to `diesel.toml` and/or run migrations and then re-make the schema:
#
#    $ make src/schema.rs
#
# Do any modifications to `src/schema.rs` and then re-make the patch:
#
#    $ make src/schema.rs.patch

SHELL = /bin/bash

.PHONY: src/schema.rs
src/schema.rs:
	# patch file must exist if it's defined in diesel.toml
	touch src/schema.rs.patch
	# print the schema
	diesel print-schema > src/schema.rs
	# make unpached version
	make src/schema.rs.unpatched
	# make the new patch file
	make src/schema.rs.patch

.PHONY: src/schema.rs.patch
src/schema.rs.patch:
	diff -U6 src/schema.rs.unpatched src/schema.rs > src/schema.rs.patch || true

.PHONY: src/schema.rs.unpatched
src/schema.rs.unpatched:
	# if patch isn't empty, create schema.rs.unpatched from the existing patch
	[ ! -s src/schema.rs.patch ] || \
		patch -p0 -R -o src/schema.rs.unpatched < src/schema.rs.patch
	# otherwise create schema.rs.unpatched by copying schema.rs
	[ -f src/schema.rs.unpatched ] || \
		cp -a src/schema.rs src/schema.rs.unpatched