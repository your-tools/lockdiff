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
foo (1.4)
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

Note: in case of `requirements.txt`, you may want to put the config in `.git/info/attributes`
because sometimes the `requirements.txt` is maintained by hand and not generated by a tool like `pip-compile`.


# Supported locks

* `Cargo.lock`
* `Gemfile.lock`
* `composer.lock`
* `go.sum`
* `package-lock.json`
* `poetry.lock`
* `pubspec.lock`
* `shard.lock`
* `yarn.lock`
* `requirements.txt` and similar (the file name must contain `requirements` and end with `.txt`, like in `dev-requirements.txt`)
