# If the first argument is "run"...
ifeq (run,$(firstword $(MAKECMDGOALS)))
  # use the rest as arguments for "run"
  RUN_ARGS := $(wordlist 2,$(words $(MAKECMDGOALS)),$(MAKECMDGOALS))
  # ...and turn them into do-nothing targets
  $(eval $(RUN_ARGS):;@:)
endif

.PHONY: run all

all: lib output

lib:
	xargo build --target x86_64-solrs

output:
	ld -o ukvm.elf -T solo5.lds target/x86_64-solrs/debug/libsolo5_hello.a  -nostdlib -z max-page-size=0x1000 -static

run: output
	-./ukvm-bin ukvm.elf $(RUN_ARGS)

