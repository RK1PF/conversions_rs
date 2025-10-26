# Release Checklist

This document outlines the steps to follow when creating a new release of Conversions RS.

## Pre-Release Checklist

### Code Quality
- [ ] All tests pass locally (`cargo test`)
- [ ] Code is properly formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Documentation is up to date (`cargo doc`)
- [ ] Examples work correctly

### Version Management
- [ ] Update version number in `Cargo.toml`
- [ ] Update version number in README.md installation section
- [ ] Update CHANGELOG.md with new version and changes
- [ ] Commit version bump changes

### Documentation
- [ ] README.md is accurate and up to date
- [ ] CHANGELOG.md includes all changes
- [ ] Code examples work with new version
- [ ] API documentation is complete

### Testing
- [ ] All unit tests pass
- [ ] Integration tests pass
- [ ] Manual testing of CLI functionality
- [ ] Test on multiple platforms if possible
- [ ] Test installation from crates.io (after publishing)

## Release Process

### 1. Prepare the Release
```bash
# Ensure you're on the main branch
git checkout main
git pull origin main

# Run final checks
cargo test
cargo clippy
cargo fmt --check

# Build release version
cargo build --release
```

### 2. Version Bump
```bash
# Update Cargo.toml version
# Update README.md version references
# Update CHANGELOG.md

# Commit changes
git add .
git commit -m "chore: bump version to vX.Y.Z"
git push origin main
```

### 3. Create and Push Tag
```bash
# Create annotated tag
git tag -a vX.Y.Z -m "Release vX.Y.Z"

# Push tag to trigger release workflow
git push origin vX.Y.Z
```

### 4. Monitor Automated Release
- [ ] GitHub Actions CI workflow completes successfully
- [ ] GitHub Actions Release workflow completes successfully
- [ ] Binaries are attached to GitHub release
- [ ] Crate is published to crates.io (if configured)

### 5. Post-Release
- [ ] Verify GitHub release page looks correct
- [ ] Test installation from crates.io: `cargo install conversions_rs`
- [ ] Update any external documentation
- [ ] Announce release (if applicable)

## GitHub Secrets Setup

For automated publishing to work, ensure these secrets are configured in your GitHub repository:

### Required Secrets
- `CRATES_IO_TOKEN`: Your crates.io API token for publishing
  - Get from https://crates.io/me
  - Should have publish scope

### Optional Secrets
- `CODECOV_TOKEN`: For code coverage reports (if using Codecov)

## Troubleshooting

### Common Issues

**Release workflow fails:**
- Check that the tag follows the format `vX.Y.Z`
- Ensure all tests pass in CI
- Verify GitHub secrets are properly configured

**Crates.io publishing fails:**
- Ensure `CRATES_IO_TOKEN` secret is set correctly
- Check that the version doesn't already exist on crates.io
- Verify Cargo.toml metadata is complete and valid

**Binary uploads fail:**
- Check the asset paths in the release workflow
- Ensure the binary names match across platforms

## Version Number Guidelines

Follow [Semantic Versioning](https://semver.org/):

- **MAJOR** (X.0.0): Incompatible API changes
- **MINOR** (X.Y.0): New functionality, backward compatible
- **PATCH** (X.Y.Z): Bug fixes, backward compatible

### Examples
- `1.0.0` → `1.0.1`: Bug fix
- `1.0.0` → `1.1.0`: New feature added
- `1.0.0` → `2.0.0`: Breaking change to public API