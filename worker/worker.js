addEventListener('fetch', event => {
  // we don't allow passthrough on errors as there is no underlying service that can answer the request
  // event.passThroughOnException()

  event.respondWith(handleRequest(event.request))
})

/**
 * Fetch and log a request
 * @param {Request} request
 */
async function handleRequest(request) {
    const { greet, parse } = wasm_bindgen;
    await wasm_bindgen(wasm)
    const greeting = greet()
    const output = parse()
    let res = new Response(greeting + "\n" + output, {status: 200})
    res.headers.set("Content-type", "text/html")
    return res
}
