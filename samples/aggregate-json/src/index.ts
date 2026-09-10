import { Hono } from 'hono';
import { fire } from 'hono/service-worker';

const RANDOM_DATA_API = 'https://random-data-api.fermyon.app';

const app = new Hono();

app.get('/api/facts', async (c) => {
  const animalsUrl = `${RANDOM_DATA_API}/animals/json`;
  const physicsUrl = `${RANDOM_DATA_API}/physics/json`;

  const responses = await Promise.all([fetch(animalsUrl), fetch(physicsUrl)]);
  const bodies = await Promise.all(
    responses.map((r) => r.json() as Promise<{ fact: string }>)
  );
  const facts = bodies.map((json) => json.fact);

  return c.json(facts);
});

fire(app)
