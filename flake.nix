{
  description = "Build bit-tower";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    crane = {
      url = "github:ipetkov/crane";
    };
    rust-overlay = {
      url = "https://flakehub.com/f/oxalica/rust-overlay/0.1.tar.gz";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nix-filter.url = "github:numtide/nix-filter";
  };

  outputs = { self, nixpkgs, crane, nix-filter, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = (import nixpkgs) {
          inherit system overlays;
        };
        inherit (pkgs) lib;
        frameworks = pkgs.darwin.apple_sdk.frameworks;

        # filter the source to reduce cache misses
        # add a path here if you need other files, e.g. bc of `include_str!()`
        src = nix-filter {
          root = ./.;
          include = [
            (nix-filter.lib.matchExt "toml")
            ./Cargo.toml
            ./Cargo.lock
            ./public
            ./content
            ./src
            ./style
            ./tailwind.config.js
          ];
        };

        toolchain = pkgs.rust-bin.nightly."2024-09-01".minimal.override {
          targets = [ "wasm32-unknown-unknown" ];
        };
        dev-toolchain = pkgs.rust-bin.nightly."2024-09-01".default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
          targets = [ "wasm32-unknown-unknown" ];
        };

        # read leptos options from `Cargo.toml`
        leptos-options = (builtins.fromTOML (
          builtins.readFile ./Cargo.toml
        )).package.metadata.leptos;
        
        # configure crane to use our toolchain
        craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;

        # crane build configuration used by multiple builds
        common-args = {
          inherit src;

          # use the name defined in the `Cargo.toml` leptos options
          pname = leptos-options.output-name;
          version = "0.1.0";

          doCheck = false;

          nativeBuildInputs = [
            pkgs.binaryen # provides wasm-opt
            pkgs.cargo-leptos
            pkgs.tailwindcss
            pkgs.makeWrapper
          ] ++ pkgs.lib.optionals (system == "x86_64-linux") [
            pkgs.nasm # wasm compiler only for x86_64-linux
          ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
            # Additional darwin specific inputs can be set here
            pkgs.libiconv # character encoding lib needed by darwin
            frameworks.Security
            frameworks.CoreFoundation
            frameworks.CoreServices
            frameworks.SystemConfiguration
            frameworks.Accelerate
          ];

          buildInputs = [
            pkgs.pkg-config # used by many crates for finding system packages
            pkgs.openssl # needed for many http libraries
          ];

        };

        cargoArtifacts = craneLib.buildDepsOnly common-args;

        bittower-frontend-deps = craneLib.mkCargoDerivation (common-args // {
          pname = "bittower-frontend-deps";
          src = craneLib.mkDummySrc common-args;
          cargoArtifacts = null;
          doInstallCargoArtifacts = true;

          buildPhaseCargoCommand = ''
            cargo build \
              --package=${leptos-options.output-name} \
              --lib \
              --target-dir=target/front \
              --target=wasm32-unknown-unknown \
              --no-default-features \
              --profile=${leptos-options.lib-profile-release}
          '';
        });

        bittower-server-deps = craneLib.mkCargoDerivation (common-args // {
          pname = "bittower-server-deps";
          src = craneLib.mkDummySrc common-args;
          cargoArtifacts = bittower-frontend-deps;
          doInstallCargoArtifacts = true;

          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [ pkgs.openssl ];

          buildPhaseCargoCommand = ''
            cargo build \
              --package=${leptos-options.output-name} \
              --no-default-features \
              --release
          '';
        });


        # build the binary and bundle using cargo leptos
        bittower = craneLib.buildPackage (common-args // {
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [ pkgs.openssl ];
          OPENSSL_DEV=pkgs.openssl.dev;

          # add inputs needed for leptos build
          nativeBuildInputs = common-args.nativeBuildInputs;

          buildPhaseCargoCommand = ''
            LEPTOS_HASH_FILE_NAME="$(pwd)/target/site/hash.txt" LEPTOS_HASH_FILES=true cargo leptos build --release -vvv -P
          '';

          installPhaseCommand = ''
            mkdir -p $out/bin
            cp target/release/${leptos-options.output-name} $out/bin/
          '';

          cargoArtifacts = bittower-server-deps;
        });

      in {
        checks = {
          # lint packages
          # app-hydrate-clippy = craneLib.cargoClippy (common-args // {
          #   cargoArtifacts = bittower-server-deps;
          #   cargoClippyExtraArgs = "-p bittower-app --features hydrate -- --deny warnings";
          # });
          # app-ssr-clippy = craneLib.cargoClippy (common-args // {
          #   cargoArtifacts = bittower-server-deps;
          #   cargoClippyExtraArgs = "-p bittower-app --features ssr -- --deny warnings";
          # });
          bittower-server-clippy = craneLib.cargoClippy (common-args // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "-p bittower-server -- --deny warnings";
          });
          bittower-frontend-clippy = craneLib.cargoClippy (common-args // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "-p bittower-frontend -- --deny warnings";
          });

          # make sure the docs build
          # bittower-server-doc = craneLib.cargoDoc (common-args // {
          #   cargoArtifacts = bittower-server-deps;
          # });

          # check formatting
          # bittower-server-fmt = craneLib.cargoFmt {
          #   pname = common-args.pname;
          #   version = common-args.version;
          #   
          #   inherit src;
          # };

          # # audit licenses
          # bittower-server-deny = craneLib.cargoDeny {
          #   pname = common_args.pname;
          #   version = common_args.version;
          #   inherit src;
          # };

          # run tests
          # bittower-server-nextest = craneLib.cargoNextest (common-args // {
          #   cargoArtifacts = bittower-server-deps;
          #   partitions = 1;
          #   partitionType = "count";
          # });
        };

        packages = {
          default = bittower;
          bittower = bittower;
        };
        
        devShells.default = pkgs.mkShell {
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [ pkgs.openssl ];
          OPENSSL_DEV=pkgs.openssl.dev;
          nativeBuildInputs = (with pkgs; [
            dev-toolchain # rust toolchain
            just # command recipes
            dive # for inspecting docker images
            cargo-leptos # main leptos build tool
            bacon # cargo check w/ hot reload
            cargo-deny # license checking
            tailwindcss
          ])
            ++ common-args.buildInputs
            ++ common-args.nativeBuildInputs
            ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
              pkgs.darwin.Security
              frameworks.Security
              frameworks.CoreFoundation
              frameworks.CoreServices
              frameworks.SystemConfiguration
              frameworks.Accelerate
            ];
        };
      }
    );
}
