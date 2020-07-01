# Try node with rust lib

This project created to test about NodeJS and Rust by comparing their performance.

## Getting started

Build rust lib

```bash
cd fizzbuzz && cargo build --release # lighter C binary file
```

Testing performance

```bash
cd fizzbuzz-app && npm install && node app.js
```

Or easier steps

```bash
npm run build_lib
npm start
```

## Performance test

```bash
$ node app.js
node_run: 0.566ms
rust_run: 0.178ms

Same result: true
```

## Build rust lib

```bash
cd fizzbuzz && cargo build --release
```

On linux will got `libfizzbuzz.so`, macos will got `libfizzbuzz.dylib`. and lib should store at `/usr/lib` or `/usr/local/lib`

## Reference links

- [retrieve-raw-value-CString](https://doc.rust-lang.org/std/ffi/struct.CString.html#method.from_raw)
- [allocated-string](http://jakegoulding.com/rust-ffi-omnibus/string_return/)
- [nodeffi-tutorial](https://github.com/node-ffi/node-ffi/wiki/Node-FFI-Tutorial)
