addEventListener("fetch", (event) => {
  event.respondWith(handleRequest(event.request));
});

/**
 * Fetch and log a request
 * @param {Request} request
 */
async function handleRequest(request) {
  const url = new URL(request.url).searchParams.get("url");

  if (url) {
    const { get_icons } = wasm_bindgen;
    await wasm_bindgen(wasm);

    const greeting = await get_icons(url);
    return new Response(greeting, { status: 200 });
  }
}
