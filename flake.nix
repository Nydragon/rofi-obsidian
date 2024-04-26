{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";

  };
  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustVersion = pkgs.rust-bin.stable.latest.default;

        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustVersion;
          rustc = rustVersion;
        };

        myRustBuild = rustPlatform.buildRustPackage {
          pname =
            "rofi-obsidian"; # make this what ever your cargo.toml package.name is
          version = "0.1.0";
          src = ./.; # the folder with the cargo.toml

          cargoLock.lockFile = ./Cargo.lock;
        };
      in {
        packages = { rustPackage = myRustBuild; };
        defaultPackage = myRustBuild;
        devShell = pkgs.mkShell {
          buildInputs =
            [ (rustVersion.override { extensions = [ "rust-src" ]; }) ];
        };
      });

}
