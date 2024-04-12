{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell {
  packages = with pkgs; [ rustup git pkg-config rust-analyzer];

  RUST_BACKTRACE = 1;
}
