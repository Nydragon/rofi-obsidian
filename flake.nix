{
  inputs = { nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable"; };
  outputs = inputs: {
    packages."x86_64-linux".default = derivation {
      name = "rofi-obsidian";
      builder =
        "${inputs.nixpkgs.legacyPackages."x86_64-linux".rustup}/bin/cargo";
      src = ./.;
      system = "x86_64-linux";
    };
  };
}
