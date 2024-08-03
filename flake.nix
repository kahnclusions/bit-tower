{
  description = "Build a cargo project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.rust-analyzer-src.follows = "";
    };

    flake-utils.url = "github:numtide/flake-utils";

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, crane, fenix, flake-utils, advisory-db, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        inherit (pkgs) lib;

        name = "spacetutor";

        rustToolchain = fenix.packages.${system}.fromToolchainFile {
          file = ./rust-toolchain.toml;
          sha256 = "sha256-MM2K43Kg+f83XQXT2lI7W/ZdQjLXhMUvA6eGtD+rqDY=";
        };

        craneLib = crane.lib.${system}.overrideToolchain rustToolchain;
        src = craneLib.cleanCargoSource (craneLib.path ./.);

        # Common arguments can be set here to avoid repeating them later
        commonArgs = {
          inherit src;
          strictDeps = true;

          buildInputs = with pkgs; [
            cargo-leptos
            binaryen
            tailwindcss
          ] ++ lib.optionals pkgs.stdenv.isDarwin [
            # Additional darwin specific inputs can be set here
            pkgs.libiconv
          ];
        };

        craneLibLLvmTools = craneLib.overrideToolchain
          (fenix.packages.${system}.complete.withComponents [
            "cargo"
            "llvm-tools"
            "rustc"
          ]);

        # Build *just* the cargo dependencies, so we can reuse
        # all of that work (e.g. via cachix) when running in CI
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        srcFilter = path: type:
          (lib.hasSuffix "tailwind.config.js" path) ||
          (lib.hasInfix "/content/" path) ||
          (lib.hasInfix "/public/" path) ||
          (lib.hasInfix "/style/" path) ||
          (craneLib.filterCargoSources path type)
        ;

        # Build the actual crate itself, reusing the dependency
        # artifacts from above.
        spacetutor = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
          src = lib.cleanSourceWith {
            src = craneLib.path ./.; # The original, unfiltered source
            filter = srcFilter;
          };

          nativeBuildInputs = with pkgs; [
            cargo-leptos
            tailwindcss
            openssl
            pkg-config
            cacert
            binaryen
            makeWrapper
          ];

          OPENSSL_DEV=pkgs.openssl.dev;

          buildPhaseCargoCommand = "cargo leptos build --release -vv";
          installPhaseCommand = ''
            mkdir -p $out/bin
            cp target/release/${name} $out/bin/
            cp target/release/hash.txt $out/bin/
            cp -r content $out/bin/content
            cp -r target/site $out/bin/
            wrapProgram $out/bin/${name} \
              --set LEPTOS_SITE_ROOT $out/bin/site \
              --set LEPTOS_HASH_FILES true
          '';
        });
      in
      {
        checks = {
          # Build the crate as part of `nix flake check` for convenience
          inherit spacetutor;

          # Run clippy (and deny all warnings) on the crate source,
          # again, reusing the dependency artifacts from above.
          #
          # Note that this is done as a separate derivation so that
          # we can block the CI if there are issues here, but not
          # prevent downstream consumers from building our crate by itself.
          spacetutor-clippy = craneLib.cargoClippy (commonArgs // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          });

          spacetutor-doc = craneLib.cargoDoc (commonArgs // {
            inherit cargoArtifacts;
          });

          # Check formatting
          spacetutor-fmt = craneLib.cargoFmt {
            inherit src;
          };

          # Audit dependencies
          spacetutor-audit = craneLib.cargoAudit {
            inherit src advisory-db;
          };

          # Audit licenses
          spacetutor-deny = craneLib.cargoDeny {
            inherit src;
          };

          # Run tests with cargo-nextest
          # Consider setting `doCheck = false` on `spacetutor` if you do not want
          # the tests to run twice
          spacetutor-nextest = craneLib.cargoNextest (commonArgs // {
            inherit cargoArtifacts;
            partitions = 1;
            partitionType = "count";
          });
        };

        packages = {
          default = spacetutor;
        } // lib.optionalAttrs (!pkgs.stdenv.isDarwin) {
          spacetutor-llvm-coverage = craneLibLLvmTools.cargoLlvmCov (commonArgs // {
            inherit cargoArtifacts;
          });
        };

        devShells.default = craneLib.devShell {
          # Inherit inputs from checks.
          checks = self.checks.${system};
          
          buildInputs =  with pkgs; [
            openssl
            pkg-config
            cacert
            cargo-make
            trunk
            (rust-bin.selectLatestNightlyWith( toolchain: toolchain.default.override {
              extensions= [ "rust-src" "rust-analyzer" ];
              targets = [ "wasm32-unknown-unknown" ];
            }))
          ] ++ pkgs.lib.optionals pkg.stdenv.isDarwin [
            darwin.apple_sdk.frameworks.SystemConfiguration
          ];

          # Additional dev-shell environment variables can be set directly
          # MY_CUSTOM_DEVELOPMENT_VAR = "something else";
          OPENSSL_DEV=pkgs.openssl.dev;

          # Extra inputs can be added here; cargo and rustc are provided by default.
          packages = with pkgs; [
            cargo-leptos
            binaryen
            tailwindcss
            just
            cargo-watch
            treefmt
            nixpkgs-fmt
            rustfmt
          ];
        };
      });
}


# {
#   description = "A basic Rust devshell for NixOS users developing Leptos";
#
#   inputs = {
#     nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
#     rust-overlay.url = "github:oxalica/rust-overlay";
#     flake-utils.url  = "github:numtide/flake-utils";
#   };
#
#   outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
#     flake-utils.lib.eachDefaultSystem (system:
#       let
#         overlays = [ (import rust-overlay) ];
#         pkgs = import nixpkgs {
#           inherit system overlays;
#         };
#       in
#       with pkgs;
#       {
#         devShells.default = mkShell {
#           buildInputs = [
#             openssl
#             pkg-config
#             cacert
#             cargo-make
#             trunk
#             (rust-bin.selectLatestNightlyWith( toolchain: toolchain.default.override {
#               extensions= [ "rust-src" "rust-analyzer" ];
#               targets = [ "wasm32-unknown-unknown" ];
#             }))
#           ] ++ pkgs.lib.optionals pkg.stdenv.isDarwin [
#             darwin.apple_sdk.frameworks.SystemConfiguration
#           ];
#
#           shellHook = ''
#             '';
#         };
#       }
#     );
# }
