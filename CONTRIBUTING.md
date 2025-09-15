# Contributing to RESL

Thanks for wanting to help! Here's how to get started.

## Getting Started

**Need Rust?** Install from [rustup.rs](https://rustup.rs/)

**Note:** This project uses stable Rust, but requires **Rust Nightly** for advanced `rustfmt` features only.

```bash
# Install nightly toolchain (for rustfmt only)
rustup install nightly

git clone https://github.com/decipher3114/resl.git
cd resl
cargo build
cargo test
```

Look for issues labeled `good first issue` if you're new.

## Making Changes

1. Fork the repo
2. Create a branch: `git checkout -b fix-something`
3. Make your changes
4. Run `cargo +nightly fmt` and `cargo clippy`
5. Make sure tests pass: `cargo test`
6. Open a pull request

## Code Style

- Use `cargo +nightly fmt` before committing (nightly required for advanced rustfmt features)
- Fix `cargo clippy` warnings (uses default stable toolchain)
- Write tests for new features
- Add examples to documentation

## Commit Messages

Keep it simple:

- `fix: handle empty strings properly`
- `docs: update CLI examples`
- `feat: add string interpolation`

## Need Help?

- Check existing issues first
- Open a new issue to discuss big changes
- Ask questions in GitHub Discussions

## Adding Language Bindings

Want RESL in your favorite language?

1. Use the existing C FFI (`resl-ffi` crate)
2. Write idiomatic wrappers for your language
3. Add tests and examples
4. Update the docs

## Bug Reports

Include:

- What you did
- What you expected
- What actually happened
- Minimal code example

## Feature Ideas

Tell us:

- What problem you're solving
- How you'd like it to work
- Why it's useful

That's it! Every contribution helps make RESL better.
