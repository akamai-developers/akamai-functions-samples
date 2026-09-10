# Sending GraphlQL queries from a Rust Component

This sample illustrates how to send GraphQL queries using a popular Rust library.

The landing page (served at `/`) offers a small form to enter a repository owner and name. Submitting it redirects to `/stargazers/<owner>/<repo>`, where the component runs a simple query for the repository's number of stars and renders the result as an HTML page.

It needs a GH API token to work.

## Build and test the app locally

To run or deploy the app, you need to [generate a GitHub API token](https://github.com/settings/personal-access-tokens). Nothing beyond the default permissions is required, at least for reading the number of stars of public repositories.

Once you've cloned the repository and moved to the `./samples/graphql-stargazer` directory, install the dependencies, build and run the app:

```console
spin build
SPIN_VARIABLE_GH_API_TOKEN=[your API token] spin up
```

You can now open http://localhost:3000/ in a browser, enter an owner and repository name, and submit the form. You can also skip the form and navigate directly to a repository, e.g. http://localhost:3000/stargazers/spinframework/spin. If you're getting an error, make sure that you provided a valid GitHub API token.

## Deploy to FWF and Run the Spin App

To deploy the application, follow these steps:

```console
spin build
spin aka deploy --variable gh_api_token=[your API token]
```

The `spin aka deploy` command will print the application URL to `stdout`. The application is now ready, and you can visit it in your browser by opening the printed URL.
