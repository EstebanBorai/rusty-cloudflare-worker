const { http_handler } = wasm_bindgen;

addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})

/**
 * Fetch and log a request
 * @param {Request} request
 */
async function handleRequest(request) {
  try {
    await wasm_bindgen(wasm);

    let body = null;
    const headers = {};
  
    if (request.body) {
      body = await request.text();
    }
  
    for (const key of request.headers.keys()) {
      headers[key.toLowerCase()] = request.headers.get(key);
    }

    const parsedRequest = {
      method: request.method,
      url: request.url,
      headers,
      body: Array.from(new Uint8Array(100)),
    };

    // log request for debugging purposes
    // probably will be commented when the project is
    // finished
    console.log(JSON.stringify(parsedRequest));

    const response = await http_handler(parsedRequest);

    return new Response(response.body, {
      status: response.status,
      headers: response.headers
    });
  } catch (error) {
    return new Response(error, {
      status: 500
    });
  }
}
