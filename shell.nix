{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell {
  packages = with pkgs; [ git pkg-config ];

  RUST_BACKTRACE = 1;
}
