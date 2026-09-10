# Aggregating JSON sample

This sample shows how to make concurrent outgoing HTTP requests from an edge
Function and combine the results into a single response.

The Function fans out two upstream requests **in parallel** with `Promise.all`,
extracts one field from each JSON document, and returns the combined values as a
single array. In a real-world situation these could be requests to different data
services, aggregated in whatever shape your client needs.

## Building and running

1. Build the application:
   ```bash
   spin build
   ```

2. Run it:
   ```bash
   spin up
   ```

The service is available at `http://localhost:3000`.

## Demo UI

Open `http://localhost:3000` in a browser for an interactive demo that explains the
sample and lets you trigger the aggregation with a button. The combined facts are
listed along with the round-trip time and the number of aggregated sources.

## API endpoints

| Method | Path         | Description                                                        |
|--------|--------------|--------------------------------------------------------------------|
| `GET`  | `/`          | The demo UI (this page).                                           |
| `GET`  | `/api/facts` | Runs the aggregation and returns a JSON array of facts.            |

### `GET /api/facts`

Fetch the aggregated result directly:

```bash
curl http://localhost:3000/api/facts
```

Example response:

```json
[
  "Cats have 32 muscles in each ear.",
  "The speed of light is about 299,792 kilometres per second."
]
```
