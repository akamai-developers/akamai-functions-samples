// For Hono documentation refer to https://hono.dev
import { Hono } from 'hono';
import { fire } from 'hono/service-worker';
import indexHtml from './index.html';
import { loadBlocklist, storeBlocklist } from './blocking';
import { getClientAddressFromRequest, cleanupIpAddress } from './helpers';

const app = new Hono();

// Route ordering matters, the first route that matches will be used.
// Any unmatched route will return a 404.

// Serve the demo UI that explains the sample and drives the API below.
app.get('/', (c) => c.html(indexHtml));

// Protected resource: apply the IP blocklist and report the outcome.
// Returns 200 when the caller is allowed, 401 when their IP is blocked.
app.get('/check', (c) => {
  const clientAddress = getClientAddressFromRequest(c.req.raw);
  if (!clientAddress) {
    return c.json(
      { allowed: false, ip: null, message: "Could not determine client IP address. Request blocked." },
      401
    );
  }

  const ip = cleanupIpAddress(clientAddress);
  const blocklist = loadBlocklist();

  if (blocklist.indexOf(ip) > -1) {
    return c.json({ allowed: false, ip, message: "Sorry, your IP is blocked." }, 401);
  }

  return c.json({
    allowed: true,
    ip,
    message: "If you can read this, you've successfully passed the blocking mechanism.",
  });
});

// Admin: list the currently blocked IP addresses.
app.get('/admin/blocked-ips', (c) => c.json(loadBlocklist()));

// Admin: add the caller's own IP address to the blocklist.
app.post('/admin/blocked-ips', (c) => {
  const clientAddress = getClientAddressFromRequest(c.req.raw);
  if (!clientAddress) {
    return c.json({ message: "Could not determine client IP address" }, 500);
  }

  const ip = cleanupIpAddress(clientAddress);
  const blocklist = loadBlocklist();
  if (blocklist.indexOf(ip) > -1) {
    return c.json({ ip, message: "Your IP address was already on the blocklist" });
  }

  blocklist.push(ip);
  storeBlocklist(blocklist);
  return c.json({ ip, message: "Your IP address has been added to the blocklist" });
});

// Admin: clear the entire blocklist.
app.delete('/admin/blocked-ips', (c) => {
  storeBlocklist([]);
  return c.body(null, 204);
});

fire(app, { fetch: undefined })
