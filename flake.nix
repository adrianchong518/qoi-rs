{
  inputs = {
    cargo2nix.url = "github:cargo2nix/cargo2nix/release-0.11.0";
    flake-utils.follows = "cargo2nix/flake-utils";
    nixpkgs.follows = "cargo2nix/nixpkgs";

    flake-compat = {
      url = github:edolstra/flake-compat;
      flake = false;
    };
  };

  outputs = { self, nixpkgs, flake-utils, cargo2nix, ... }:

  flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ cargo2nix.overlays.default ];
      };

      rustPkgs = pkgs.rustBuilder.makePackageSet {
        rustVersion = "1.61.0";
        packageFun = import ./Cargo.nix;
      };
    in
    rec {
      packages.qoi-rs = (rustPkgs.workspace.qoi-rs {}).bin;
      packages.default = packages.qoi-rs;

      devShell = rustPkgs.workspaceShell {
        nativeBuildInputs = with pkgs; [
          cargo2nix.packages.${system}.default
        ];
      };
    }
  );
}