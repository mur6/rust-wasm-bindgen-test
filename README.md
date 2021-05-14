# rust-wasm-bindgen-test

## Prerequisite
You need to have emscripten, and OpenCV installed.

See here to install emscripten:
- https://emscripten.org/docs/getting_started/downloads.html

To build OpenCV with Emscripten, see here:
- https://docs.opencv.org/master/d4/da1/tutorial_js_setup.html

## Building the package
```
wasm-pack build --target web
```
So, it will generate `pkg/rust_wasm_bindgen_test.js`.

## Run
```
python3 -m http.server 8002
```
And if you access `http://localhost:8002/`, you will see this:

![result image](result.png)
