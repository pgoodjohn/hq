# H.Q.!

CLI Swiss Army Knife written in Rust.

## Manual Installation

Running `make release` will create a release build and move it to `~/.hq/bin`. Add the following to your `.zshconfig` to be able to run H.Q.! from anywhere:

```bash
if [ -f "/Users/$USERNAME/.hq/bin/hq" ]; then
        path+=("/Users/$USERNAME/.hq/bin")
        export PATH
fi
```