Mon [![Build Status](https://travis-ci.org/TheSpiritXIII/Mon.svg?branch=master)](https://travis-ci.org/TheSpiritXIII/Mon)
===
Mon is a `Rust` library for making games RPG games similar to Pokémon. Mon takes advantage of `Cargo`'s build script functionality to allow meta-data to be declared declaratively within `TOML` files. This allows generation of static data structures, making it easier and faster to define resources.

Mon was originally created to be used within GameMaker (GM), which are simply bindings of the C bindings. Mon may be used as a Rust library itself.

Usage
-----
Use the `resources` directory to place resource definitions. Sample resources are provided in `samples` which emulate the resources from the official Pokémon games.

Building
--------
You must use `Rust` nightly to compile, since the build script uses `serde`'s compiler plugins which are only available in the nightly.

If you want to try this library with the samples, make sure you copy the samples to the `resources` directory first.
 - Windows:
   ```
   robocopy sample resources
   ```
 - Linux/Unix/OS X:
   ```
   cp -r sample resources
   ```

To build, run `Cargo`, which effectively generates and compiles `Rust` code from the resources in `resources`.
```
cd mon-gen
cargo build --release
```

There are several options you may pass as features to `Cargo`. One of them is `rebuild` which forces the `resources` to be recompiled. By default, they are only recompiled if the modification time does not match the previous modification time. Another is `c-api` which compiles the library with the C API.

Here is an example of using the `rebuild` feature.
```
cargo build --release --features 'rebuild'
```

Testing
-------
Mon includes a mock testing module under `gen_test`. This compiles all sample resources and uses those for testing. To test, you must specify the `test` feature in order for Cargo to rebuild the build script and base library to include the mock testing module. This can be done as so:
```
cargo test --features "test"
```

GameMaker
---------
To use with GM, run `Cargo` on `mon-gm`. You do not need to run `Cargo` on `mon-gen`. This generates a `.dll` file inside `target`. This `.dll` file must be copied to the GameMaker extension within the `gamemaker` directory. There is also a generated `constants.txt` to be imported into GameMaker as constants inside the `gen` folder created in `target`. To import constants into GameMaker, open `All configurations` under `Macros`. Then, load the generated `constants.txt` file.

Below are scripts for copying the `.dll` file to the extension directory.
 - Windows:
   ```
   mkdir gamemaker\extensions\Mon\
   robocopy mon-gm\target\release\ gamemaker\extensions\Mon\ mon.dll
   ```
 - Linux/Unix/OS X:
   ```bash
   mkdir gamemaker\extensions\Mon
   cp mon-gm/target/release/mon.dll gamemaker\extensions\Mon
   ```

Notice
------
Pokémon and all respective names are copyright © Nintendo.
