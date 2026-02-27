{
  description = "Flake for nixgc-clear package";

  nixConfig = {
    extra-trusted-public-keys = "eigenvalue.cachix.org-1:ykerQDDa55PGxU25CETy9wF6uVDpadGGXYrFNJA3TUs=";
    extra-substituters = "https://eigenvalue.cachix.org";
  };
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";

    crate2nix.url = "github:nix-community/crate2nix";
    crate2nix.inputs = {
      cachix.follows = "";
      crate2nix_stable.follows = "";
      flake-compat.follows = "";
      nix-test-runner.follows = "";
    };
  };

  outputs =
    inputs@{
      self,
      nixpkgs,
      crate2nix,
      flake-parts,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      perSystem =
        { pkgs, system, ... }:
        {
          devShells = {
            default = pkgs.mkShell {
              packages = with pkgs; [
                rustc
                clippy
                cargo
                rust-analyzer
                rustfmt
              ];
            };
          };

          packages =
            let
              cargoNix = crate2nix.tools."${system}".generatedCargoNix {
                name = "nixgc-clear";
                src = ./.;
              };
              cargoNix' = pkgs.callPackage "${cargoNix}/default.nix" { };
            in
            {
              cargoNix = cargoNix;

            }
            // rec {
              nixgc-clear = cargoNix'.rootCrate.build;
              default = nixgc-clear;
            };
        };

      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
    };
}
