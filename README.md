# `cargo-quickfix`

`cargo-quickfix` is a `cargo` wrapper for use with Vim's [quickfix feature](http://vimdoc.sourceforge.net/htmldoc/quickfix.html).

The normal way to use the quickfix feature is to run build commands inside of Vim using `:make`, which automatically populates the quickfix buffer.

However, this can be inconvenient if you prefer to run `cargo` outside of Vim.

`cargo-quickfix` allows you to run `cargo` commands outside of `vim`, but still populate the quickfix list with error locations.

## Quick Start

- Install:

  ```
  cargo install cargo-quickfix
  ```

- Run:

  ```
  cargo quickfix test
  ```

- Open vim:

  ```
  vim
  ```

- Open the error file and jump to the first error:

  ```
  :cfile
  ```

- Jump to the next error:

  ```
  :cn
  ```

## Usage

Run `cargo quickfix <COMMAND>`, for example, `cargo quickfix test`.
You can also omit the command, in which case `cargo quickfix` will default to `check`.

Upon running, the output of the `cargo` command will be copied to `.errors.txt`.

Inside a Vim instance in the same directory, load `.errors.txt` into the quickfix buffer and jump to the first error, if any, with `:cfile`.

Useful commands include:
- `:cn` Go to the next error.
- `:cp` Go the previous error.
- `:cope` Open the quickfix list.

See `:help quickfix` for information.

Optionally, install the [vim-qf](https://github.com/romainl/vim-qf) plugin for additional mappings and functionality.

## Ignoring `.errors.txt`

To avoid committing `.errors.txt` or seeing it in `git diff` and `git status`, you can add it to your repositories `.gitignore`.

To avoid needing to do this for every repository, you can add it to a global ignore file:

```
git config --global core.excludesFile '~/.gitignore'
echo .errors.txt > ~/.gitignore
```
