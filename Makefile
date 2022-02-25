.PHONY: test
test:
	cargo test -- --test-threads=1

header:
	cp $(OPAM_SWITCH_PREFIX)/lib/libirmin/include/irmin.h ./docs/irmin.h
