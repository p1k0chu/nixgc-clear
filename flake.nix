{
  description = "Flake for nixgc-clear package";

  nixConfig = {
    extra-trusted-public-keys = "eigenvalue.cachix.org-1:ykerQDDa55PGxU25CETy9wF6uVDpadGGXYrFNJA3TUs=";
    extra-substituters = "https://eigenvalue.cachix.org";
  };
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    crate2nix.url = "github:nix-community/crate2nix";
    crate2nix.inputs = {
      nixpkgs.follows = "nixpkgs";
      cachix.follows = "";
      crate2nix_stable.follows = "";
      flake-compat.follows = "";
      nix-test-runner.follows = "";
      # as much as i'd love to disable those too, it wont work.
      # devshell.follows = "";
      # pre-commit-hooks.follows = "";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      crate2nix,
    }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      forAllSystems =
        func:
        nixpkgs.lib.genAttrs systems (
          system:
          let
            pkgs = import nixpkgs { inherit system; };
            cargoNix = crate2nix.tools."${system}".appliedCargoNix {
              name = "nixgc-clear";
              src = ./.;
            };
          in
          (func {
            inherit
              pkgs
              system
              cargoNix
              ;
          })
        );

    in
    {
      devShells = forAllSystems (
        { pkgs, ... }:
        {
          default = pkgs.mkShell {
            packages = with pkgs; [
              rustc
              clippy
              cargo
              rust-analyzer
              rustfmt
            ];
          };
        }
      );

      packages = forAllSystems (
        { cargoNix, ... }:
        rec {
          nixgc-clear = cargoNix.rootCrate.build;
          default = nixgc-clear;
        }
      );
    };
}
