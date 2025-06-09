# Publishing Guide for Doksnet GitHub Action

This guide covers how to publish and maintain the Doksnet GitHub Action.

## Prerequisites

1. **Repository Setup**: Ensure the action files are in the root of your repository:
   - `action.yml` - Action configuration
   - `ACTION_README.md` - Action documentation

2. **Testing**: The action should be tested with the test workflow (`.github/workflows/test-action.yml`)

## Publishing Steps

### 1. Create Release Tags

GitHub Actions are distributed via Git tags. Create semantic version tags:

```bash
# Example: Publishing version 1.0.0
git tag -a v1.0.0 -m "Release v1.0.0"
git push origin v1.0.0

# Create major version tag (for users who want latest v1.x.x)
git tag -a v1 -m "Release v1"
git push origin v1 --force  # Use --force to move existing tag
```

### 2. GitHub Marketplace (Optional)

To publish on GitHub Marketplace:

1. Go to your repository on GitHub
2. Click "Releases" → "Create a new release"
3. Select your version tag (e.g., `v1.0.0`)
4. Fill in release notes
5. Check "Publish this Action to the GitHub Marketplace"
6. Select appropriate categories
7. Publish release

### 3. Usage After Publishing

Users can reference your action in their workflows:

```yaml
# Use specific version (recommended for production)
- uses: Pulko/doksnet@v1.0.0

# Use major version (gets latest v1.x.x)
- uses: Pulko/doksnet@v1

# Use latest (not recommended for production)
- uses: Pulko/doksnet@main
```

## Version Management

### Semantic Versioning

- **Major (v1.0.0 → v2.0.0)**: Breaking changes
- **Minor (v1.0.0 → v1.1.0)**: New features, backward compatible
- **Patch (v1.0.0 → v1.0.1)**: Bug fixes, backward compatible

### Tag Strategy

```bash
# For new minor/patch versions
git tag v1.1.0
git push origin v1.1.0

# Update major version pointer
git tag -f v1
git push origin v1 --force

# For major version changes
git tag v2.0.0
git push origin v2.0.0
git tag v2
git push origin v2
```

## Maintenance

### Updating Dependencies

The action installs doksnet from crates.io. No dependency updates needed in the action itself.

### Testing New Versions

Before publishing:

1. Test locally with `act` (if available)
2. Test on a fork with pull requests
3. Run the test workflow (`.github/workflows/test-action.yml`)

### Breaking Changes

If making breaking changes:
1. Update major version
2. Update documentation
3. Consider deprecation warnings for previous versions

## Action Structure

```
repository-root/
├── action.yml              # Action definition
├── ACTION_README.md        # Action documentation  
├── .github/
│   └── workflows/
│       └── test-action.yml # Action tests
└── ... (rest of doksnet project)
```

## Publishing Checklist

- [ ] `action.yml` is properly configured
- [ ] `ACTION_README.md` has usage examples
- [ ] Test workflow passes
- [ ] Version tag created and pushed
- [ ] GitHub release created (if using Marketplace)
- [ ] Major version tag updated
- [ ] Documentation updated with new version

## Troubleshooting

### Action Not Found

- Ensure tags are pushed: `git push origin --tags`
- Verify repository is public (for public actions)
- Check action.yml syntax

### Action Fails

- Check the action logs in GitHub Actions UI
- Verify inputs/outputs match documentation
- Test with minimal workflow first

### Marketplace Issues

- Ensure action.yml has proper `name` and `description`
- Add `branding` section for marketplace display
- Follow GitHub's marketplace guidelines 