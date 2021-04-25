addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})

async function parseRequestBody(request, headers) {
  const contentType = headers['content-type'];

  if (!request.clone().body) {
    return null;
  }

  if (contentType.includes('json')) {
    return request.clone().json();
  }

  if (contentType.includes('text')) {
    return request.clone().text();
  }

  return request.body;
}

async function handleRequest(req) {
  try {
    const { bootstrap, handle_request } = wasm_bindgen;

    // Initialize WASM Bindgen
    await wasm_bindgen(wasm);

    bootstrap();

    const headers = {};
  
    for (const key of req.headers.keys()) {
      headers[key.toLowerCase()] = req.headers.get(key);
    }

    const request = {
      method: req.method,
      url: req.url,
      headers,
      body: await parseRequestBody(req.clone(), headers),
    }

    const response = await handle_request(request);

    return new Response(response.body, {
      status: response.status,
      headers: response.headers,
    });
  } catch (error) {
    console.error('WorkerError', JSON.stringify(error));

    if ('status_code' in error && 'headers' in error) {
      return new Response(JSON.stringify(error.body), {
        status: error.status_code,
        headers: error.headers,
      });
    }

    return new Response(JSON.stringify(error), {
      status: 500
    });
  }
}
