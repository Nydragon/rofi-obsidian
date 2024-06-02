# rofi-obsidian

<div align="center">

[![Crates.io Version](https://img.shields.io/crates/v/rofi-obsidian?style=flat-square&logo=rust)](https://crates.io/crates/rofi-obsidian)
[![Nixpkgs-unstable Version](https://img.shields.io/badge/dynamic/json?url=https%3A%2F%2Fwww.nixhub.io%2Fpackages%2Frofi-obsidian%3F_data%3Droutes%252F_nixhub.packages.%2524pkg._index&query=%24.releases.0.version&prefix=v&style=flat-square&logo=nixos&logoColor=fff&label=nixpkgs-version)](https://search.nixos.org/packages?channel=unstable&show=rofi-obsidian)
![GitHub Release Date](https://img.shields.io/github/release-date/Nydragon/rofi-obsidian?style=flat-square&logo=github)
![GitHub commits since latest release (branch)](https://img.shields.io/github/commits-since/Nydragon/rofi-obsidian/latest?style=flat-square&logo=github)

</div>

This project currently has 2 main goals, integrating the opening of specific obsidian vaults directly into [rofi](https://github.com/davatorium/rofi), and circumventing a shortcoming obsidian currently has, which is being unable to select which vault to open at startup.

This programs makes use of x-scheme-handler to open the program that is currently assigned to handling `obsidian://*` URIs.

# Example installation

### Using cargo

#### From crates.io

Execute:

```sh
cargo install rofi-obsidian
```

#### From source

Clone the repository and execute:

```sh
cargo install
```

### Using Nix

#### Nixpkgs

```sh
nix profile install nixpkgs#rofi-obsidian
```

#### Latest changes

```sh
nix profile install github:Nydragon/rofi-obsidian
```

### Manually

Either add the binary to your $PATH environment variable or move it into the $XDG_CONFIG_HOME/rofi/scripts folder.

## Final Step

Don't forget to modify your config.rasi in the following way:
Add `"obsidian:rofi-obsidian"` to the `modes` array and `obsidian` to `combi-modes`.
You may circumvent step 1 and specify the entire path to the binary instead: "obsidian:/home/nico/.config/rofi/rofi-obsidian".

A minimal config example could look like this:

```rasi
configuration {
    combi-modes: ["obsidian"];
    modes: ["obsidian:rofi-obsidian"]
}
```

# Troubleshooting

## Obsidian doesn't start

Make sure that one of the following programs is installed:

- xdg-open
- gio
- gnome-open
- kde-open

## A different program starts when using the plugin

Execute the following command (or an equivalent command to inspect mime type associations):

```bash
xdg-mime query default x-scheme-handler/obsidian
```

It should return the executable being used to start Obsidian.
