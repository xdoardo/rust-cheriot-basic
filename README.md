# Building this crate 

Having `rustup` and [CHERIoT-enabled LLVM](https://github.com/CHERIoT-Platform/llvm-project) installed locally
should suffice. Note: your local build of LLVM should have enabled both the
`riscv32cheriot-unknown-cheriotrtos` target and the host target, because the
`rustc` fork calls some LLVM API that are different from upstream (in
particular, for `memcpy` and `memmove` -- see more
[here](https://github.com/rust-lang/rust/commit/f2f792ecd416a38ffc9bc9464123bea03f2de3c8)).

1. Clone the [rustc fork](https://github.com/xdoardo/rust) with the CHERIoT target added:
```sh 
$ git clone https://github.com/xdoardo/rust --branch=cheriot-on-1.88.0
```

2. Generate the `bootstrap.toml` file. Note: needs the `$CHERIOT_SYSROOT_DIR` env variable to be set and pointing to the `llvm-project/build` directory of your local LLVM build.
```sh
$ cd rust && ./gen_bootstrap.sh
```

3. Build the compiler and needed libraries: 
```sh
$ ./x build compiler core panic_abort --target=riscv32cheriot-unknown-cheriotrtos
```

4. Link the toolchain with rustup: 
```sh
$ rustup toolchain link 'cheriot' build/host/stage1
```

5. Build this crate!
```sh
$ cd rust-cheriot-basic && cargo +cheriot build --release  --target=riscv32cheriot-unknown-cheriotrtos
```

# What should work

The goal of this example is that of producing a working `rustc` compiler that
can compile simple programs to CHERIoT. We expect this crate to be compiled to
an `.s` library with the symbols for the defined functions. 

You should then be able to link the generated `.s` library with an RTOS program
that uses these functions. The test I ran looks like this: 
```cpp
extern "C" int                zero();
extern "C" int                add(int a, int b);
extern "C" long long unsigned div(long long unsigned a, long long unsigned b);

#include "cheri.hh"
#define TEST_NAME "RUST"
#include "tests.hh"

int test_rust()
{
	int zero_from_rust = zero();
	debug_log("Got zero from rust: {}", zero_from_rust);

	int add_from_rust = add(4, 2);
	debug_log("Got add from rust: {}", add_from_rust);

	long long unsigned div_from_rust = div(4, 2);
	debug_log("Got div from rust: {}", div_from_rust);

	return 0;
}
```

I got `xmake` to build it successfully by changing
```lua
batchcmds:vrunv(target:tool("ld"), table.join({"--script=" .. linkerscript, "--compartment", "--gc-sections", "--relax", "-o", target:targetfile()}, target:objectfiles()), opt)
```
to
```lua
batchcmds:vrunv(target:tool("ld"), table.join({"--script=" .. linkerscript, "-L<path_to_rust_cheriot_basic_dir>", "-lrust_cheriot_basic", "--compartment", "--gc-sections", "--relax", "-o", target:targetfile()}, target:objectfiles()), opt)
```
in `cheriot-rtos/sdk/xmake.lua`, and adding the relevant `test(...)` settings
in `cheriot-rtos/tests/xmake.lua`.
