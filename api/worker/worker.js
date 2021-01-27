addEventListener("fetch", (event) => {
  event.respondWith(handleRequest(event.request));
});

/**
 * Fetch and log a request
 * @param {Request} request
 */
async function handleRequest(request) {
  const { get_icons, get_repo_icons, set_token } = wasm_bindgen;
  await wasm_bindgen(wasm);

  const url = new URL(request.url);

  let token = url.searchParams.get("token");
  if (token) set_token(token);

  let site_url = url.searchParams.get("url");
  if (site_url) {
    const greeting = await get_icons(site_url);
    return new Response(greeting, { status: 200 });
  }

  let user = url.searchParams.get("user");
  let repo = url.searchParams.get("repo");
  if (user && repo) {
    const greeting = await get_repo_icons(user, repo);
    return new Response(greeting, { status: 200 });
  }
}
