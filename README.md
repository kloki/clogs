# clogs

A structured logs pretty printer that works for me.

# Install

## Cargo

```
cargo install clogs
```

## Binaries

Check [Releases](https://github.com/kloki/clogs/releases) and download the tar file.

For linux run:

```bash
tar -xvf clogs-x86_64-unknown-linux-gnu.tar.xz && cp clogs-x86_64-unknown-linux-gnu/clogs ~/.local/bin/
```

# Run

```bash
cat test.logs | clogs
```

Combine with other commands.

```bash
cat test.log | jq -c 'select(.fields.user_id=="12345")' | clogs | less -r
```
