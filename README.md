Mon
===
Mon is a `Rust` library for making games RPG games similar to Pokémon. Mon takes advantage of `Cargo`'s build script functionality to allow meta-data to be declared declaratively within `TOML` files. This allows generation of static data structures, making it easier and faster to define inter-related resources.

I originally created Mon to be used within GameMaker (GM). As such, it builds the GM bindings by default, although the base may be used as a Rust library itself and the GM bindings may also be used with C (althought I highly discourage that since GameMaker only supports doubles and C strings).

Contact me if you're interested in using my library directly or require bindings -- I may be willing to help set everything up.

Usage
-----
Use the `res` directory to place resource definitions. See the specifications document for all more details on resource files. Sample resources are provided in `samples/res` which emulate the resources from the official Pokémon games. See `SPECIFICATION.md` for the resource file specifications.

Building
--------
You must use `Rust` nightly to compile, since the build script uses `serde`'s compile plugins which are only available in the nightly.

If you want to try with the samples, make sure you copy the samples to the `res` directory first.
 - Windows:
   ```
   robocopy sample\res res
   ```
 - Linux/Unix/OS X:
   ```
   cp -r sample/res res
   ```

Then run `Cargo`, which effectively generates and compiles `Rust` code using resources in `res`.
```
cargo build --release
```

In addition to the `.dll` file created in `target`, there is also a generated `import/constants.txt` to be imported into GameMaker. To import in GameMaker, open `All configurations` under `Macros`. Then, load the generated `import/constants.txt` file. Finally, before exporting the GameMaker extension, the compiled library file must be copied to the extension location.
 - Windows:
   ```
   robocopy target\release\ gm\extensions\Mon\ mon.dll
   ```
 - Linux/Unix/OS X:
   ```
   cp target/release/mon.dll res
   ```

Notice
------
Pokémon and all respective names are copyright © Nintendo.
