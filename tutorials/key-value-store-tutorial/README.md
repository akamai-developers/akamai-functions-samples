# Tutorial: Using the Key Value Store

This folder contains the sample application written as part of the [Use the Key Value Store](https://techdocs.akamai.com/akamai-functions/docs/use-the-key-value-store) tutorial.

## Prerequisites

You need the following tools on your machine, to build, run and deploy the application to _Akamai Functions_:

- The `spin` CLI
- Node.js (Version `24` or later)
- The `aka` plugin for `spin` CLI
- Access to _Akamai Functions_


## Building the Spin Application

Once you've cloned the repository, move into the tutorial folder ([./tutorials/key-value-store-tutorial](./tutorials/key-value-store-tutorial)), install the dependencies using `npm` and run `spin build`:

```bash
cd tutorials/key-value-store-tutorial
spin build
```

## Invoking the endpoints with `curl`

You can set a value using:

```bash
curl -iX POST -d '{ "firstName" :"John", "lastName": "Doe"}' localhost:3000/set/jd
```

and retrieve it again using:

```bash
curl localhost:3000/get/jd
```
