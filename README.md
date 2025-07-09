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
$ ./x build compiler core alloc --target=riscv32cheriot-unknown-cheriotrtos
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
a static (`.s`) library with the symbols for the defined functions. 

You should then be able to link the generated `.s` library with an RTOS program
that uses these functions. The file `rust-test.cc` shows an example of how to
use this library from RTOS: you should be able to add it as a test in
`cheriot-rtos/tests` and run it. Does it work? :) 

## Note 
I got `xmake` to build it successfully by changing
```diff
--- a/sdk/xmake.lua
+++ b/sdk/xmake.lua
@@ -202,7 +202,7 @@ rule("cheriot.component")
 		-- Link using the compartment's linker script.
 		batchcmds:show_progress(opt.progress, "linking " .. target:get("cheriot.type") .. ' ' .. target:filename())
 		batchcmds:mkdir(target:targetdir())
-		batchcmds:vrunv(target:tool("ld"), table.join({"--script=" .. linkerscript, "--compartment", "--gc-sections", "--relax", "-o", target:targetfile()}, target:objectfiles()), opt)
+		batchcmds:vrunv(target:tool("ld"), table.join({"--script=" .. linkerscript, "-L<path_to_rust_lib>", "-lrust_cheriot_basic", "--compartment", "--gc-sections", "--relax", "-o", target:targetfile()}, target:objectfiles()), opt)
 		-- This depends on all of the object files and the linker script.
 		batchcmds:add_depfiles(linkerscript)
 		batchcmds:add_depfiles(target:objectfiles())
```
in `cheriot-rtos/sdk/xmake.lua`.

You will also have to add the relevant `test(...)` settings in
`cheriot-rtos/tests/xmake.lua`, and add a matching call to `test_rust` in
`test-runner.cc`. Another important addition is that of giving more stack size to the tests: 
```diff
@@ -142,7 +146,7 @@ firmware("test-suite")
                 compartment = "test_runner",
                 priority = 3,
                 entry_point = "run_tests",
-                stack_size = 0x800,
+                stack_size = 0x1F00,
                 -- This must be an odd number for the trusted stack exhaustion
                 -- test to fail in the right compartment.
                 trusted_stack_frames = 9
```


Ideally, when this is a bit more tested, we can have this
crate as part of the RTOS tests, and make all of this easier.
