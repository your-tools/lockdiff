# Lockdiff

Convert a lock with lots of info to something more readable

For instance:

```toml
[[package]]
name = "foo"
version = "1.4"
```

becomes:

```
foo@1.4
```

Note that this works both with `poetry.lock` and `Cargo.lock` - both use `toml`
with the same syntax :)

# Using lockdiff with git

The primary purpose of this tool is to hide "noise" related to package changes
in automatically generated files when using with `git`

For this to work, you need to register the 'lockdiff' in .gitattributes:

```
Cargo.lock diff=lockdiff
```

And have the following section in your git config:

```ini
[diff "lockdiff"]
textconv = lockdiff
```
