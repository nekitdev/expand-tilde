# Changelog

<!-- changelogging: start -->

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
