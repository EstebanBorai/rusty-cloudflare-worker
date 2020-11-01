<div>
  <!--
    <div align="center" style="display: block; text-align: center;">
      <img src="" height="120" width="120" />
    </div>
  -->
  <h1 align="center">image-resizer-worker</h1>
  <h4 align="center">👷🏻‍♂️ Cloudflare Worker to manipulate images files</h4>
</div>

## Setup

1. Clone this project locally

```bash
git clone git@github.com:EstebanBorai/image-resizer-worker.git
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

This command will print the following:

```
 ╭──────────────────────────────────────────────────────────────────────────────────────────────────────╮
 │                                                                                                      │
 │             To find your API Token, go to https://dash.cloudflare.com/profile/api-tokens             │
 │                     and create it using the "Edit Cloudflare Workers" template.                      │
 │                                                                                                      │
 │      Consider using `wrangler login` which only requires your Cloudflare username and password.      │
 │                                                                                                      │
 │                 If you are trying to use your Global API Key instead of an API Token                 │
 │                         (Not Recommended), run `wrangler config --api-key`.                          │
 │                                                                                                      │
 ╰──────────────────────────────────────────────────────────────────────────────────────────────────────╯

Enter API Token:
```

Paste your API token, and press _Enter_.

> Your API token must have access to edit workers

5. Publish your worker to your account

```bash
wrangler publish
```

6. Find your worker on the web via: `https://<worker name>.<your workers sub domain>.workers.dev`
