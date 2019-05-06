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
  const { linear_regression } = wasm_bindgen;
  await wasm_bindgen(wasm)
  try {
    // extract parameters
    const postData = await request.json()
    const x_input = postData.input.map(e => e.x)
    const y_input = postData.input.map(e => e.y)
    const x_predict = postData.predict

    // run training. The output is already a json object serialised via serde due to wasm not allowing us to return arrays
    const output = linear_regression(x_input, y_input, x_predict)
    
    // send response
    let res = new Response(JSON.stringify(output), {status: 200})
    res.headers.set("Content-type", "application/json")
    return res
  } catch (err) {
    return new Response(err.stack || err, {status: 404})
  }
}

