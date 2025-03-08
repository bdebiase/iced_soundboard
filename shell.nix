let
  flake = builtins.getFlake (toString ./nix);
in
  flake.devShells.${builtins.currentSystem}.default