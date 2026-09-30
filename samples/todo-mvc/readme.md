# TodoMVC (Vue + Spin + Key-Value Store)

This Spin application consists of two major components:

- `frontend`: Vue JS frontend for managing todos borrowed from TodoMVC
- `api`: HTTP API exposing different endpoints for managing todos (implemented using TinyGo)

The app uses the Key-Value store for persisting todos.

## Building the Application

The application is built using the `spin build` command. As the `frontend` is implemented using VueJS and the `api` is implemented with TinyGo, Node.JS and TinyGo must be installed on the system before invoking `spin build`

## Running locally

To run the application locally, execute the `spin up` command.


## Deploying to Akamai Functions

Once authenticated with `spin aka login`, the application can be deployed to Akamai Functions using the `spin aka deploy` command.
