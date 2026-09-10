import { Hono } from 'hono';
import { fire } from 'hono/service-worker';

import * as Variables from '@spinframework/spin-variables';
import * as Kv from '@spinframework/spin-kv';

import indexHtml from './index.html';
import { Config, RequestTacking } from './models';
import { FetchEventLike } from 'hono/types';

const app = new Hono();

// Serve the demo UI that explains the sample and drives the API below.
app.get('/', (c) => c.html(indexHtml));

// Side-effect-free status endpoint used by the UI to report whether the
// time gate is currently open or closed. It never contacts the origin.
app.get('/status', (c) => {
  const config = readConfig();
  if (!config) {
    return c.json({ error: 'origin and block_until must be configured' }, 500);
  }
  const now = Date.now();
  const blockUntil = Date.parse(config.blockUntil);
  return c.json({
    blocked: now < blockUntil,
    now: new Date(now).toISOString(),
    blockUntil: new Date(blockUntil).toISOString(),
    blockStatusCode: config.blockStatusCode,
  });
});

// The access-limited endpoint: blocked until `block_until`, then proxied to
// `origin`. This preserves the original behaviour of the sample.

app.all('/access', (c) => guard(c.req.raw, c.event));
app.all('/access/*', (c) => guard(c.req.raw, c.event));

const readConfig = (): Config | null => {
  const origin = Variables.get('origin');
  const blockUntil = Variables.get('block_until');
  const trackBlockedRequests = Variables.get('track_blocked_requests');
  const blockLocation = Variables.get('block_location');
  let blockStatusCode = Variables.get('block_status_code');

  if (!origin || !blockUntil || !isValidDate(blockUntil)) {
    console.log('origin or block_until are not configured (or invalid)');
    return null;
  }
  if (!blockStatusCode) {
    blockStatusCode = '404';
  }
  return {
    origin,
    blockUntil,
    blockLocation,
    // variables are read as strings, convert status to number
    blockStatusCode: +blockStatusCode,
    // variables are read as strings, check if tracking is enabled
    trackBlockedRequests: trackBlockedRequests === 'true',
  } as Config;
};

const guard = async function (req: Request, event: FetchEventLike): Promise<Response> {
  const config = readConfig();
  if (!config) {
    return new Response(null, { status: 500 });
  }

  try {
    const now = new Date().getTime();
    const blockUntil = Date.parse(config.blockUntil);
    if (now < blockUntil) {
      console.log("Will block request...")
      if (config.trackBlockedRequests) {
        event.waitUntil(trackBlockedRequest(req));
      }
      let headers: any = {};
      if (!!config.blockLocation) {
        headers['location'] = config.blockLocation;
      }
      return new Response(null, {
        status: config.blockStatusCode,
        headers: headers,
      });
    }
  } catch (err) {
    console.log(`Error while checking block window: ${err}`);
    return new Response(null, { status: 500 });
  }

  const url = buildTargetUrl(req, config);
  const response = await fetch(url, {
    method: req.method,
    headers: req.headers,
    body: req.body as ReadableStream,
  });

  return new Response(response.body, {
    status: response.status,
    headers: response.headers,
  });
};

const isValidDate = (value: string): boolean => {
  const date = new Date(value);
  return !isNaN(date.getTime());
};

const buildTargetUrl = (req: Request, config: Config): string => {
  const incomingUrl = new URL(req.url);
  const origin = new URL(config.origin);

  // Strip the `/access` prefix so the origin sees a clean path.
  const incomingPath = incomingUrl.pathname.replace(/^\/access/, '');
  const originPath = origin.pathname;
  let destinationPath = originPath + incomingPath;
  destinationPath = destinationPath.replace(/\/+/g, '/');
  const destinationUrl = origin.origin + destinationPath + incomingUrl.search + incomingUrl.hash;
  return destinationUrl;
};

const trackBlockedRequest = async (req: Request) => {
  // Force this operation to be async so tracking happens in the background
  // (via event.waitUntil) without adding latency to the user response.
  await new Promise((resolve) => setTimeout(resolve));
  const key = `${req.method.toLowerCase()}_${req.url}`;
  const store = Kv.openDefault();
  let value = { count: 1 } as RequestTacking;
  if (store.exists(key)) {
    value = store.getJson(key);
    value.count += 1;
  }
  store.setJson(key, value);
};


fire(app, {fetch: undefined})
