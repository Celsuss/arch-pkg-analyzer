default: lint build test

build:
  @echo Building…
  cargo build

test:
  @echo Testing…
  cargo nextest run

lint:
  @echo Linting…
  cargo clippy --all-targets -- -D warnings
  cargo +nightly fmt --all -- --check
  taplo format --check
  cargo deny check
  cargo machete

check:
    @cargo check --all

# Run the analyzer against the local system (requires sudo for some DB access)
run *args:
    cargo run -- {{args}}

# Package the project using a clean chroot (requires devtools)
chroot-build:
    extra-x86_64-build

setup-git:
    @echo "Setting up Git hooks..."
    # (commands to copy pre-commit hooks)
