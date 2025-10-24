# clogs

A structured logs pretty printer that works for me.

# Install

## Cargo

```
cargo install clogs
```

## Binaries

Check [Releases](https://github.com/kloki/clogs/releases) for binaries and installers

# Run

```bash
cat test.logs | clogs
```

Combine with other commands.

```bash
cat test.log | jq -c 'select(.fields.user_id=="12345")' | clogs | less -r
```
