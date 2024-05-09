# rofi-obsidian

This project currently has 2 main goals, integrating the opening of specific obsidian vaults directly into [rofi](https://github.com/davatorium/rofi), and circumventing a shortcoming obsidian currently has, which is being unable to select which vault to open at startup.

This programs makes use of x-scheme-handler to open the program that is currently assigned to handling `obsidian://*` URIs.

# Example installation

## Using cargo

Clone the repository and execute `cargo install`

## Manually

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
