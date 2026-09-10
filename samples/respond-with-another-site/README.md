# Respond with another site

This sample shows a Fermyon Wasm Function that responds to a request with the response from another site (in this case a 'random animal facts' sample).

The app is fronted by a small landing page:

- `GET /` serves a UI that explains the proxy behavior.
- `GET /site` performs the server-side `fetch` to the upstream site and returns its response.

## Run it

```console
spin build
spin up
```

Then open http://localhost:3000/ in a browser and press **Fetch from the other site**, or hit the proxy endpoint directly:

```console
curl http://localhost:3000/site
```

## Allowed outbound hosts

Note that the `spin.toml` file lists the upstream site as an allowed outbound request destination for the component:

```toml
allowed_outbound_hosts = ["https://random-data-api.fermyon.app"]
```

By default, components are sandboxed, so you must enable the outbound site, or you'll get an "access denied" error. (Constraining the hosts in this way prevents the component from sending data to unwanted sites, for example in the event of a NPM package being compromised.)
