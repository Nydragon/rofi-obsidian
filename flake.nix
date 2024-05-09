{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };
  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustVersion = pkgs.rust-bin.stable.latest.default;

        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustVersion;
          rustc = rustVersion;
        };

        myRustBuild = rustPlatform.buildRustPackage {
          pname = manifest.name;
          version = manifest.version;
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;

          meta = with nixpkgs.lib; {
            description = manifest.description;
            license = licenses.unlicense;
            maintainers = [ ];
          };
        };
      in {
        packages = { rustPackage = myRustBuild; };
        defaultPackage = myRustBuild;
        devShell = pkgs.mkShell {
          buildInputs =
            [ (rustVersion.override { extensions = [ "rust-src" ]; }) ];
          shellHook = let
            cargo = "${pkgs.cargo}/bin/cargo";
            rofi = "${pkgs.rofi}/bin/rofi";
            bin = "rofi-obsidian";
          in ''
            alias test-rofi="${cargo} build && ${rofi} -show o -modes o:./target/debug/${bin}"
            alias release-rofi="${cargo} build && ${rofi} -show o -modes o:./target/profile/${bin}"

            echo "Use test-rofi to build and run the debuging version with rofi."
            echo "Use release-rofi to build and run the release version with rofi."
          '';
        };
      });

}
