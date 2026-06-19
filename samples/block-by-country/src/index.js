// https://itty.dev/itty-router/routers/autorouter
import { AutoRouter } from 'itty-router';
import { blockByCountry, loadBlocklist, storeBlocklist } from "./blocking";
import { getClientAddressFromRequest, cleanupIpAddress } from "./helpers";

let router = AutoRouter();

// Route ordering matters, the first route that matches will be used
// Any route that does not return will be treated as a middleware
// Any unmatched route will return a 404
router
  .get("/", blockByCountry, () => handleGetData())
  .get("/index.html", () => redirectToRoot())
  .get("/admin/blocked-countries", () => handleGetBlockList())
  .post("/admin/blocked-countries", (req) => handleAddCountryToBlocklist(req))
  .delete("/admin/blocked-countries", () => handleClearBlocklist())

//@ts-ignore
addEventListener('fetch', async (event) => {
  event.respondWith(router.fetch(event.request));
});

function redirectToRoot() {
  return new Response(null, {
    status: 301, headers: { "location": "/" }
  })
}

function handleGetData() {
  const payload = `<html>
<head><title>Block by Country Example</title>
</head>
<body>
<h1>Block by IP Country</h1>
<p>If you can read this, you've succesfully passed the guard!</p>
<ul>
<li>You can send a <code>GET</code> request to <code>/admin/blocked-countries</code> to retrieve the list of all blocked countries.</li>
<li>Send a <code>POST</code> request to <code>/admin/blocked-countries</code> to add the current country (determined by the client IP) to the block list</li>
<li>Send a <code>DELETE</code> request to <code>/admin/blocked-countries</code> to remove the current country (determined by the client IP) from the block list</li>
</ul>
</body>
  </html>`

  return new Response(payload, { status: 200, headers: { "content-type": "text/html" } });

}

function handleGetBlockList() {
  const blocklist = loadBlocklist();
  return new Response(JSON.stringify(blocklist || []), { status: 200, headers: { "content-type": "application/json" } });
}

async function handleAddCountryToBlocklist(req) {
  const clientAddress = getClientAddressFromRequest(req);
  if (!clientAddress) {
    return new Response("Could not determine client ip address", { status: 500 });
  }
  const ip = cleanupIpAddress(clientAddress);
  const response = await fetch(`http://ip-api.com/json/${ip}`);
  const details = await response.json();

  let blocklist = loadBlocklist();
  if (blocklist.indexOf(details.country) > -1) {
    return new Response(JSON.stringify({
      message: `Your country (${details.country}) is already on the blocklist`
    }), { status: 200, headers: { "content-type": "application/json" } });
  }

  blocklist.push(details.country);
  storeBlocklist(blocklist);
  return new Response(JSON.stringify({
    message: `Your country (${details.country}) has been added to the blocklist`
  }), { status: 200, headers: { "content-type": "application/json" } });

}

function handleClearBlocklist() {
  storeBlocklist([]);
  return new Response(null, { status: 204 });
}
