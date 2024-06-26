{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };
  outputs =
    {
      nixpkgs,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustVersion = pkgs.rust-bin.stable.latest.default;

        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustVersion;
          rustc = rustVersion;
        };

        rofi-obsidian-pkg = rustPlatform.buildRustPackage {
          pname = manifest.name;
          version = manifest.version;
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;

          meta = {
            description = manifest.description;
            license = nixpkgs.lib.licenses.unlicense;
            maintainers = [ ];
          };
        };
      in
      {
        packages = rec {
          rofi-obsidian = rofi-obsidian-pkg;
          default = rofi-obsidian;
        };

        devShells.default = pkgs.mkShell {
          buildInputs = [
            (rustVersion.override { extensions = [ "rust-src" ]; })
            pkgs.git-cliff
            pkgs.committed
            pkgs.pre-commit
            pkgs.typos
          ];
          shellHook =
            let
              cargo = "${pkgs.cargo}/bin/cargo";
              rofi = "${pkgs.rofi}/bin/rofi";
              bin = "rofi-obsidian";
              rofiDebug = "rofi-obsidian-debug";
              rofiRelease = "rofi-obsidian-release";
              RED = "\\033[0;31m";
              NC = "\\033[0m";
            in
            ''
              alias ${rofiDebug}="${cargo} build && ${rofi} -show o -modes o:./target/debug/${bin}"
              alias ${rofiRelease}="${cargo} build --profile release && ${rofi} -show o -modes o:./target/release/${bin}"
              ${pkgs.pre-commit}/bin/pre-commit install -f

              printf "\n${RED}INFO -----------${NC}\n\n";
              echo "Use ${rofiDebug} to build and run the debugging version with rofi."
              echo "Use ${rofiRelease} to build and run the release version with rofi."
              printf "\n${RED}INFO -----------${NC}\n\n";
            '';
        };
      }
    );
}
