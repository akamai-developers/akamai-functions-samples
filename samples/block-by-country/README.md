# Block by Client Country

This sample illustrates how you could block clients from accessing a particular resource by maintaining a country blocklist.

Open the app root (`/`) in a browser for a small UI that explains the sample and lets you check your access, block your own country, view, or clear the blocklist. Under the hood the UI calls the endpoints below.

- `GET /` — the demo UI.
- `GET /check` — the protected resource. Returns `200` with `{ allowed, country, message }` when your country is not blocked, and `403` with the same shape when it is.
- `GET /admin/blocked-countries` — list the blocked countries.
- `POST /admin/blocked-countries` — add the caller's own country to the blocklist.
- `DELETE /admin/blocked-countries` — clear the blocklist.

The blocklist is stored in the default key-value store. Behind the covers, [ip-api.com](https://ip-api.com) is used to look up the country from the client's IP address (taken from the `spin-client-addr`/`true-client-ip` headers).

> **Caution**: The country lookup does not work for a client IP of `127.0.0.1` because `ip-api.com` does not resolve the loopback address. When testing locally, pass a public IP via the `true-client-ip` header (see below).

## Deploy to FWF and Run the Spin App

Once you've cloned the repository and moved to `./samples/block-by-country`, install the dependencies, build and run the app:

```console
spin build
spin aka deploy
```

The `spin aka deploy` command will print the application URL to `stdout`. Store the URL in a variable called `APP_URL`:

```console
export APP_URL=<YOUR_APP_URL>
```

### Accessing the protected route

Send a `GET` request to `/check`, which returns the outcome as JSON. Pass a public IP via `true-client-ip` to simulate a client location:

```console
curl -iX GET -H 'true-client-ip: 8.8.8.8' $APP_URL/check

HTTP/1.1 200 OK
content-type: application/json

{"allowed":true,"country":"United States","message":"If you can read this, you've successfully passed the blocking mechanism."}
```

### Block your own country

To block the country resolved from your (simulated) IP, send a `POST` request to `/admin/blocked-countries`:

```console
curl -iX POST -H 'true-client-ip: 8.8.8.8' $APP_URL/admin/blocked-countries

HTTP/1.1 200 OK
content-type: application/json

{"message":"Your country (United States) has been added to the blocklist.","blocklist":["United States"]}
```

### Try to access the protected route again

Send another `GET` to `/check` with the same IP — this time it is blocked with a `403`:

```console
curl -iX GET -H 'true-client-ip: 8.8.8.8' $APP_URL/check

HTTP/1.1 403 Forbidden
content-type: application/json

{"allowed":false,"country":"United States","message":"Sorry, your country (United States) is blocked."}
```

### Retrieve the country blocklist

```console
curl -iX GET $APP_URL/admin/blocked-countries

HTTP/1.1 200 OK
content-type: application/json

["United States"]
```

### Clear the country blocklist

```console
curl -iX DELETE $APP_URL/admin/blocked-countries

HTTP/1.1 204 No Content
```
