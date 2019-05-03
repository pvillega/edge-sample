# Edge Sample

A sample project to run a serverless application in the [Cloudflare Edge network](https://www.cloudflare.com/products/cloudflare-workers/) using [WebAssembly](https://rustwasm.github.io/book/what-is-webassembly.html) 

Based on a template for kick starting a Cloudflare worker project using
[`wasm-pack`](https://github.com/rustwasm/wasm-pack).

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
