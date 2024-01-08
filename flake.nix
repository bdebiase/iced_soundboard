{
  description = "Build a cargo project while also compiling the standard library";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };

    fenix.url = "github:nix-community/fenix/monthly";
  };

  outputs = { self, nixpkgs, crane, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        rustToolchain = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
          extensions = [ "rust-src" ];
          targets = [ "x86_64-unknown-linux-gnu" ];
        });

        # rustToolchain = with fenix.packages.${system};
        #   fromToolchainFile {
        #     file = ./rust-toolchain.toml;
        #     sha256 = "sha256-U2yfueFohJHjif7anmJB5vZbpP7G6bICH4ZsjtufRoU=";
        #   };

        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        my-crate = craneLib.buildPackage {
          src = craneLib.cleanCargoSource (craneLib.path ./.);
          strictDeps = true;

          # cargoVendorDir = craneLib.vendorMultipleCargoDeps {
          #   inherit (craneLib.findCargoFiles src) cargoConfigs;
          #   cargoLockList = [
          #     ./Cargo.lock

          #     "${rustToolchain.passthru.availableComponents.rust-src}/lib/rustlib/src/rust/Cargo.lock"
          #   ];
          # };

          # cargoExtraArgs = "-Z build-std --target x86_64-unknown-linux-gnu";

          nativeBuildInputs = with pkgs; [
            pkg-config
          ];

          buildInputs = with pkgs; [
            alsa-lib
            jack2
            glib
            atk
            pango
            gtk3
          ];
        };
      in
      {
        packages.default = my-crate;

        apps.default = flake-utils.lib.mkApp {
          drv = my-crate;
        };

        devShells.default = craneLib.devShell {
          packages = [
            my-crate.nativeBuildInputs
            my-crate.buildInputs
          ];
        };
      });
}
