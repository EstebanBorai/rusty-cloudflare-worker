<div>
  <div align="center" style="display: block; text-align: center;">
    <img src="./assets/ferris_wasm.png" width="200" />
  </div>
  <h1 align="center">rusty-cloudflare-worker</h1>
  <h4 align="center">👷🏻‍♂️ Rust Cloudflare Worker to handle HTTP requests</h4>
</div>

<div align="center">

  [![Deploy to Cloudflare Workers](https://deploy.workers.cloudflare.com/button)](https://deploy.workers.cloudflare.com/?url=https://github.com/EstebanBorai/rusty-cloudflare-worker)

</div>

## Getting Started

Run Wrangler's development server using NPX as follows:

```sh
npx @cloudflare/wrangler dev
```

## Setup

1. Clone this project locally

```bash
git clone git@github.com:EstebanBorai/rusty-cloudflare-worker.git
```

2. Create a `wrangler.toml` file in the root directory of this repository
and paste the following contents:

```toml
name = "<worker name>"
type = "rust"

account_id = "<enter your account id>"
workers_dev = true
route = ""
zone_id = ""
```

3. Install `wrangler` CLI either with NPM or Cargo.

```bash
# npm
npm install -g @cloudflare/wrangler

# cargo
cargo install wrangler
```

4. Find your API Token and configure it

```bash
wrangler config
```

Paste your API token, and press _Enter_.

> Your API token should have access to edit workers

5. Publish your worker to your account

```bash
wrangler publish
```

6. Find your worker on the web via: `https://<worker name>.<your workers sub domain>.workers.dev`
