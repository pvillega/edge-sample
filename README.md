# Edge Sample

A sample project to run a serverless application in the [Cloudflare Edge network](https://www.cloudflare.com/products/cloudflare-workers/) using [WebAssembly](https://rustwasm.github.io/book/what-is-webassembly.html) 

On receiving a POST request with an array of numeric pairs `(x, y)` (training data) and an array of values `x` to predict for, it calculates and returns the prediction using simple linear regression on that data.

Format of input body:

```
{
    "input":[  // Input data to use to calculate the linear regression coeficients
        {"x":1,"y":1},
        {"x":2,"y":2}
    ],
    "predict":[1,2]  // Array of values (x) for which we want to predict a result (y) using the linear regression coeficients calculated from the input 
}
```

Example of requests:

```
$  wrangler preview post '{"input":[{"x":1,"y":1},{"x":2,"y":2}],"predict":[1,2]}' 
Your worker responded with: {"coeficient":1,"intercept":0,"accuracy":0,"y_predictions":[1,2]}
```

See the bottom of this file for an example with more data.

Based on a template for kick starting a Cloudflare worker project using
[`wasm-pack`](https://github.com/rustwasm/wasm-pack).

Linear regression code based on [this blog post](https://cheesyprogrammer.com/2018/12/13/simple-linear-regression-from-scratch-in-rust/)

## Requirements

- Install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- Install [wrangler](https://developers.cloudflare.com/workers/webassembly/) via `cargo install wrangler`

## Useful Commands

- Build: `wrangler build`
- Preview in Chrome: `wrangler preview`
- Run tests: `wasm-pack test --node`
- Test in headless Safari: `wasm-pack test --headless --safari`

## Included dependencies

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
* [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.

## License

This code is open source software licensed under the Apache 2.0 License.

## Example for testing with more data

This is just a bigger sample to test performance. It runs under 5ms in Cloudflare workers so it can be used with the 'free' Cloudflare plan

Input:

wrangler preview post '{"input":[{"x":5.1,"y":3.5},{"x":4.9,"y":3.0},{"x":4.7,"y":3.2},{"x":4.6,"y":3.1},{"x":5.0,"y":3.6},{"x":5.4,"y":3.9},{"x":4.6,"y":3.4},{"x":5.0,"y":3.4},{"x":4.4,"y":2.9},{"x":4.9,"y":3.1},{"x":5.4,"y":3.7},{"x":4.8,"y":3.4},{"x":4.8,"y":3.0},{"x":4.3,"y":3.0},{"x":5.8,"y":4.0},{"x":5.7,"y":4.4},{"x":5.4,"y":3.9},{"x":5.1,"y":3.5},{"x":5.7,"y":3.8},{"x":5.1,"y":3.8},{"x":5.4,"y":3.4},{"x":5.1,"y":3.7},{"x":4.6,"y":3.6},{"x":5.1,"y":3.3},{"x":4.8,"y":3.4},{"x":5.0,"y":3.0},{"x":5.0,"y":3.4},{"x":5.2,"y":3.5},{"x":5.2,"y":3.4},{"x":4.7,"y":3.2},{"x":4.8,"y":3.1},{"x":5.4,"y":3.4},{"x":5.2,"y":4.1},{"x":5.5,"y":4.2},{"x":4.9,"y":3.1},{"x":5.0,"y":3.2},{"x":5.5,"y":3.5},{"x":4.9,"y":3.1},{"x":4.4,"y":3.0},{"x":5.1,"y":3.4},{"x":5.0,"y":3.5},{"x":4.5,"y":2.3},{"x":4.4,"y":3.2},{"x":5.0,"y":3.5},{"x":5.1,"y":3.8},{"x":4.8,"y":3.0},{"x":5.1,"y":3.8},{"x":4.6,"y":3.2},{"x":5.3,"y":3.7},{"x":5.0,"y":3.3},{"x":7.0,"y":3.2},{"x":6.4,"y":3.2},{"x":6.9,"y":3.1},{"x":5.5,"y":2.3},{"x":6.5,"y":2.8},{"x":5.7,"y":2.8},{"x":6.3,"y":3.3},{"x":4.9,"y":2.4},{"x":6.6,"y":2.9},{"x":5.2,"y":2.7},{"x":5.0,"y":2.0},{"x":5.9,"y":3.0},{"x":6.0,"y":2.2},{"x":6.1,"y":2.9},{"x":5.6,"y":2.9},{"x":6.7,"y":3.1},{"x":5.6,"y":3.0},{"x":5.8,"y":2.7},{"x":6.2,"y":2.2},{"x":5.6,"y":2.5},{"x":5.9,"y":3.2},{"x":6.1,"y":2.8},{"x":6.3,"y":2.5},{"x":6.1,"y":2.8},{"x":6.4,"y":2.9},{"x":6.6,"y":3.0},{"x":6.8,"y":2.8},{"x":6.7,"y":3.0},{"x":6.0,"y":2.9},{"x":5.7,"y":2.6},{"x":5.5,"y":2.4},{"x":5.5,"y":2.4},{"x":5.8,"y":2.7},{"x":6.0,"y":2.7},{"x":5.4,"y":3.0},{"x":6.0,"y":3.4},{"x":6.7,"y":3.1},{"x":6.3,"y":2.3},{"x":5.6,"y":3.0},{"x":5.5,"y":2.5},{"x":5.5,"y":2.6},{"x":6.1,"y":3.0},{"x":5.8,"y":2.6},{"x":5.0,"y":2.3},{"x":5.6,"y":2.7},{"x":5.7,"y":3.0},{"x":5.7,"y":2.9},{"x":6.2,"y":2.9},{"x":5.1,"y":2.5},{"x":5.7,"y":2.8},{"x":6.3,"y":3.3},{"x":5.8,"y":2.7},{"x":7.1,"y":3.0},{"x":6.3,"y":2.9},{"x":6.5,"y":3.0},{"x":7.6,"y":3.0},{"x":4.9,"y":2.5},{"x":7.3,"y":2.9},{"x":6.7,"y":2.5},{"x":7.2,"y":3.6},{"x":6.5,"y":3.2},{"x":6.4,"y":2.7},{"x":6.8,"y":3.0},{"x":5.7,"y":2.5},{"x":5.8,"y":2.8},{"x":6.4,"y":3.2},{"x":6.5,"y":3.0},{"x":7.7,"y":3.8},{"x":7.7,"y":2.6},{"x":6.0,"y":2.2},{"x":6.9,"y":3.2},{"x":5.6,"y":2.8},{"x":7.7,"y":2.8},{"x":6.3,"y":2.7},{"x":6.7,"y":3.3},{"x":7.2,"y":3.2},{"x":6.2,"y":2.8},{"x":6.1,"y":3.0},{"x":6.4,"y":2.8},{"x":7.2,"y":3.0},{"x":7.4,"y":2.8},{"x":7.9,"y":3.8},{"x":6.4,"y":2.8},{"x":6.3,"y":2.8},{"x":6.1,"y":2.6},{"x":7.7,"y":3.0},{"x":6.3,"y":3.4},{"x":6.4,"y":3.1},{"x":6.0,"y":3.0},{"x":6.9,"y":3.1},{"x":6.7,"y":3.1},{"x":6.9,"y":3.1},{"x":5.8,"y":2.7},{"x":6.8,"y":3.2},{"x":6.7,"y":3.3},{"x":6.7,"y":3.0},{"x":6.3,"y":2.5},{"x":6.5,"y":3.0},{"x":6.2,"y":3.4},{"x":5.9,"y":3.0}],"predict":[4.5,6.5]}'

Output:

{"coeficient":-0.057268277,"intercept":3.3886375,"accuracy":0.42955422,"y_predictions":[3.1309304,3.0163937]}