# Slow Gifs on rust example

## Cargo Release

```bash
cargo run --release
```

Output

```bash
C:\Users\Arnav Jindal\Documents\test-gi>cargo run --release
   Compiling test-gi v0.1.0 (C:\Users\Arnav Jindal\Documents\test-gi)
    Finished release [optimized] target(s) in 2.09s
     Running `target\release\test-gi.exe`
9734ms
```

This means that a basic read-write op for a gif thats included took almost 10s!

## my Machine

I closed everything except the Vscode window I had open. Nothing else was running.

i7 8550u with 16Gb of ram.

Try this on your machine as well and lmk!.