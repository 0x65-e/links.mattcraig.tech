# About

This website is written for Cloudflare Workers using Rust and the [workers-rs](https://github.com/cloudflare/workers-rs) crate. Cloudflare workers is a serverless, "function as a service" (FaaS) platform that runs across distributed data centers.

It redirects from shortened links to their full counterparts. Links are defined in Workers KV, a serverless key-value store on the edge.

## Usage

With `wrangler` CLI, you can build, test, and deploy to Workers with the following commands: 

```bash
# compiles project to WebAssembly and will warn of any issues
wrangler build 

# runs Worker in an ideal development workflow (with a local server, file watcher & more)
wrangler dev

# deploys Worker globally to Cloudflare
wrangler publish
```

You will need to generate your own KV namespace and replace the values in [wrangler.toml](wrangler.toml).

```bash
# creates a preview namespace
wrangler kv:namespace create "LINKS" --preview

# creates a production namespace
wrangler kv:namespace create "LINKS"
```

You can choose a name other than `LINKS` for your namespace, but be sure to update the KV access in [libs.rs](src/lib.rs).

You may also want to change the name of your worker in [wrangler.toml](wrangler.toml).

### Continuous Deployment

This repository includes a [Github Workflow](.github/workflows/workers_deploy.yml) that automatically publishes the latest version of the `master` branch on every push.

For CD to work properly with the wrangler CLI, you must set up repository secrets named `CF_ACCOUNT_ID` and `CF_API_TOKEN` with your Cloudflare account ID and API token as described in the [wrangler system environment variables guide](https://developers.cloudflare.com/workers/wrangler/system-environment-variables/).
