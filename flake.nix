{
  # Special thanks to paper-plane developers for original project.
  description = ''
    Chat over Telegram on a modern and elegant client.
  '';

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
    treefmt.url = "github:numtide/treefmt-nix";

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      crane,
      flake-utils,
      #advisory-db,
      treefmt,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        craneLib = crane.mkLib pkgs;
        src = craneLib.cleanCargoSource ./.;

        # Common arguments can be set here to avoid repeating them later
        commonArgs = {
          inherit src;
          strictDeps = true;

          buildInputs = [
            pkgs.gtk4
            pkgs.libadwaita
          ];
          nativeBuildInputs = [
            pkgs.pkg-config
            pkgs.wrapGAppsHook4
          ];
          cargoArtifacts = craneLib.buildDepsOnly commonArgs;

          # Additional environment variables
          CARGO_PROFILE = "dev";
        };

        # Build the actual crate itself, reusing the dependency
        # artifacts from above.
        adwaiGram = craneLib.buildPackage commonArgs;
      in
      {
        checks = {
          # Build the crate as part of `nix flake check` for convenience
          inherit adwaiGram;

          clippy = craneLib.cargoClippy (
            commonArgs
            // {
              cargoClippyExtraArgs = "--all-targets -- --deny warnings";
            }
          );

          doc = craneLib.cargoDoc commonArgs;

          # Check formatting
          rustFmt = craneLib.cargoFmt {
            inherit src;
          };

          tomlFmt = craneLib.taploFmt {
            src = pkgs.lib.sources.sourceFilesBySuffices src [ ".toml" ];
            # taploExtraArgs = "--config ./taplo.toml";
          };

          # Audit dependencies
          # audit = craneLib.cargoAudit {
          #   inherit src advisory-db;
          # };

          # Audit licenses
          # deny = craneLib.cargoDeny {
          #   inherit src;
          # };

          # Run tests with cargo-nextest
          # Consider setting `doCheck = false` on `my-crate` if you do not want
          # the tests to run twice
          nextest = craneLib.cargoNextest (
            commonArgs
            // {
              partitions = 1;
              partitionType = "count";
              cargoNextestPartitionsExtraArgs = "--no-tests=pass";
            }
          );
        };

        packages = {
          default = adwaiGram;
        };

        apps.default = flake-utils.lib.mkApp {
          drv = adwaiGram;
        };

        devShells.default = craneLib.devShell {
          # Inherit inputs from checks.
          checks = self.checks.${system};

          # Additional dev-shell environment variables can be set directly
          # MY_CUSTOM_DEVELOPMENT_VAR = "something else";

          # Extra inputs can be added here; cargo and rustc are provided by default.
          packages = [
            pkgs.nixd
            pkgs.nixfmt-rfc-style
            pkgs.taplo
          ]
          ++ commonArgs.buildInputs
          ++ commonArgs.nativeBuildInputs;
        };
        formatter = (treefmt.lib.evalModule pkgs ./treefmt.nix).config.build.wrapper;
      }
    );
}
