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

# Using lockdiff with git

The primary purpose of this tool is to hide "noise" related to package changes
in automatically generated files such as `Cargo.lock` when using with `git diff`
and related commands.

For this to work, you need to register the 'lockdiff' as a text converter
in your git config (usually `~/.config/git/config`)

```ini
[diff "lockdiff"]
textconv = lockdiff
```

and then tell `git` to use `lockdiff` when diffing files, in `~/.config/git/attributes`:

```
Cargo.lock diff=lockdiff
```


# Supported locks

* `Cargo.lock`
* `Gemfile.lock`
* `composer.lock`
* `go.sum` (beta)
* `poetry.lock` (fun fact, Poetry and Cargo have compatible formats 😎)
* `package-lock.json`
* `yarn.lock`
