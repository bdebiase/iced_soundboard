{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    utils,
    naersk,
  }:
    utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {inherit system;};
      naersk-lib = pkgs.callPackage naersk {};
    in {
      defaultPackage = naersk-lib.buildPackage ./.;
      devShell = with pkgs;
        mkShell {
          nativeBuildInputs = [
            pkg-config
          ];
          buildInputs = [
            cargo
            rustc
            rustfmt
            pre-commit
            rustPackages.clippy
            freetype
            fontconfig
            glib
            cairo
            pango
            gdk-pixbuf
            atk
            gtk3
            alsa-lib
            jack2
          ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
          FONTCONFIG_FILE = makeFontsConf {
            fontDirectories = [freefont_ttf];
          };

          shellHook = ''
            export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${
              with pkgs;
              #lib.makeLibraryPath [libGL xorg.libX11 xorg.libXi xorg.libXcursor xorg.libXrandr]
                lib.makeLibraryPath [libGL wayland libxkbcommon fontconfig freetype]
            }"
          '';
        };
    });
}
