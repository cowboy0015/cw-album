# Building the Project
To build the project, follow these steps:
1. Create a `.cargo` folder and add a file named `config`. In the `config` file, include the following aliases:
```
[alias]
wasm = "build --release --target wasm32-unknown-unknown"
unit-test = "test --lib"
schema = "run --example schema"
```
2. Build the project using the following command:

```
RUSTFLAGS='-C link-arg=-s' cargo wasm
```
This command compiles the project for WebAssembly (Wasm) target.