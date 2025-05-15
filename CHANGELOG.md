# Changelog

<!-- changelogging: start -->

## [0.6.1](https://github.com/nekitdev/expand-tilde/tree/v0.6.1) (2025-05-15)

### Changes

- Added `?Sized` bound in `ExpandTilde` implementation.

## [0.6.0](https://github.com/nekitdev/expand-tilde/tree/v0.6.0) (2025-03-31)

### Features

- Added `expand_tilde_owned` method to the `ExpandTilde` trait.

## [0.5.0](https://github.com/nekitdev/expand-tilde/tree/v0.5.0) (2025-03-21)

### Features

- Added `diagnostics` feature that enables `miette` for error reporting.

## [0.4.0](https://github.com/nekitdev/expand-tilde/tree/v0.4.0) (2025-01-22)

### Features

- Added `expand_tilde_owned` function, which accepts the path by value and returns owned paths.
  ([#4](https://github.com/nekitdev/expand-tilde/pull/4))

## [0.3.0](https://github.com/nekitdev/expand-tilde/tree/v0.3.0) (2025-01-11)

### Removals

- `expand_tilde_path` was moved into `expand_tilde_inner` function inside `expand_tilde`.
  ([#3](https://github.com/nekitdev/expand-tilde/pull/3))

## [0.2.0](https://github.com/nekitdev/expand-tilde/tree/v0.2.0) (2025-01-03)

### Features

- Added `home_dir` function which improves diagnostic messages.
  ([#2](https://github.com/nekitdev/expand-tilde/pull/2))

### Changes

- Expansion functions now use `Cow<'_, Path>` to avoid unnecessary allocations.
  ([#2](https://github.com/nekitdev/expand-tilde/pull/2))

### Internal

- The actual expansion was moved to a separate function, `expand_tilde_path`.
  ([#2](https://github.com/nekitdev/expand-tilde/pull/2))

## [0.1.0](https://github.com/nekitdev/expand-tilde/tree/v0.1.0) (2024-10-15)

Initial release.
