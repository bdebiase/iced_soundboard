{
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    nixpkgs,
    flake-utils,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];

        pkgs = import nixpkgs {
          inherit system overlays;
        };

        buildInputs = with pkgs; [
          alsa-lib
          jack2
          glib
          atk
          pango
          gtk3

          expat
          fontconfig
          freetype
          freetype.dev
          libGL
          pkg-config
          xorg.libX11
          xorg.libXcursor
          xorg.libXi
          xorg.libXrandr
          wayland
          libxkbcommon

          rust-bin.nightly.latest.default
          rust-analyzer
        ];
      in {
        devShells.default = pkgs.mkShell {
          inherit buildInputs;

          LD_LIBRARY_PATH =
            builtins.foldl' (a: b: "${a}:${b}/lib") "${pkgs.vulkan-loader}/lib" buildInputs;
          RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
          #CARGO_ENCODED_RUSTFLAGS = "-Clink-arg=-fuse-ld=${pkgs.mold}/bin/mold";

          shellHook = ''
            export TMPDIR=/mnt/LinuxExpansion/Places/tmp
            mkdir -p $TMPDIR
          '';
        };
      }
    );
}
