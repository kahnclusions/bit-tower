# 🗼 bit-tower

`bit-tower` is a self-hosted open-source web frontend for QBitTorrent, optimised for mobile devices, written in Rust. The web app is a Leptos full-stack app using `wasm-bindgen` in the browser and served with `axum`.

WARNING: Currently in early development, not ready for use. Only viewing torrent progress via sockets is implemented.

| light | dark |
| ----- | ---- |
| ![light mode](./public/preview-light.png) | ![dark mode](./public/preview-dark.png) |

## Building

Building with Nix is easiest:

```bash
nix build
```

But you can also build it directly. The environment variables need to be set so that we can embed the `hash.txt` into the binary.

```bash
LEPTOS_HASH_FILE_NAME="$(pwd)/target/site/hash.txt" LEPTOS_HASH_FILES=true cargo leptos build --release -vv -P
```

## Deploying

While experimental this project is only available as a Nix flake and can be deployed anywhere you use Nix.

All the assets (wasm, js, css) are bundled into the binary with their hashes. After building you can run like so:

```bash
LEPTOS_HASH_FILES=true ./result/bin/bittower
```

You can configure it to run as a systemd service like so:

```nix
{ config, pkgs, bittower, ... }:
{
  # ... 
  systemd.services.bittower = {
      enable = true;
      description = "bittower";
      unitConfig = {
        Type = "simple";
      };
      environment = {
        LEPTOS_SITE_ADDR = "127.0.0.1:3010";
        LEPTOS_ENV = "PROD";
        LEPTOS_HASH_FILES = "true"; # required in release mode
      };
      serviceConfig = {
        Restart="always";
        ExecStart = "${bittower.packages.${pkgs.system}.default}/bin/bittower";
        WorkingDirectory = "${bittower.packages.${pkgs.system}.default}/bin";
      };
      wantedBy = [ "multi-user.target" ];
  };
};
```
