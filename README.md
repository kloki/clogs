# clogs

A structured logs pretty printer that works for me.

```
cargo install clogs
```

# Run

```bash
cat test.logs | clogs
```

Combine with other commands.

```bash
cat test.log | jq -c 'select(.fields.user_id=="12345")' | clogs | less -r
```
