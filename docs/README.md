# milk Documentation

Welcome to the milk documentation. This directory contains all technical documentation for building, testing, and maintaining the milk audio player.

## 📚 Documentation Map

### Getting Started

- **[Quick Build Guide](BUILDING.md)** - Fast-track guide to building milk
- **[Technical Specification](milk_tech_spec.md)** - Original technical specification and architecture

### Build & Release

- **[Build Guide](BUILD.md)** - Comprehensive build and packaging documentation
- **[Build Configuration](BUILD_CONFIGURATION.md)** - Detailed configuration reference
- **[Release Checklist](RELEASE_CHECKLIST.md)** - Step-by-step release process
- **[Task 20 Summary](TASK_20_SUMMARY.md)** - Build and packaging implementation summary

### Testing

- **[Installation Testing](INSTALLATION_TESTING.md)** - Complete installation and functional testing guide

### Development

- **[Error Handling](ERROR_HANDLING.md)** - Error handling patterns and guidelines
- **[Performance Optimizations](PERFORMANCE_OPTIMIZATIONS.md)** - Performance tuning and optimization strategies

## 🚀 Quick Links

### For Developers

1. Start here: [Quick Build Guide](BUILDING.md)
2. Understand the architecture: [Technical Specification](milk_tech_spec.md)
3. Learn error handling: [Error Handling](ERROR_HANDLING.md)
4. Optimize performance: [Performance Optimizations](PERFORMANCE_OPTIMIZATIONS.md)

### For Release Managers

1. Build the application: [Build Guide](BUILD.md)
2. Verify configuration: [Build Configuration](BUILD_CONFIGURATION.md)
3. Test installation: [Installation Testing](INSTALLATION_TESTING.md)
4. Follow release process: [Release Checklist](RELEASE_CHECKLIST.md)

## 📖 Document Descriptions

### BUILDING.md

Quick reference for building milk. Includes prerequisites, build commands, and basic troubleshooting.

**Use when:** You need to quickly build the application.

### BUILD.md

Comprehensive guide covering the entire build and packaging process, including MSI/NSIS installers, portable distributions, and verification steps.

**Use when:** You need detailed information about the build system, installers, or distribution.

### BUILD_CONFIGURATION.md

Technical reference for all build configuration files, including Tauri config, Cargo settings, and file associations.

**Use when:** You need to understand or modify build configurations.

### RELEASE_CHECKLIST.md

Step-by-step checklist for creating a new release, from version updates to GitHub release creation.

**Use when:** Preparing to release a new version of milk.

### INSTALLATION_TESTING.md

Comprehensive testing guide covering MSI installer, NSIS installer, portable version, performance testing, and functional testing.

**Use when:** Testing installation packages or verifying a release.

### ERROR_HANDLING.md

Guidelines for error handling patterns, error types, and best practices in the milk codebase.

**Use when:** Implementing error handling or debugging issues.

### PERFORMANCE_OPTIMIZATIONS.md

Performance optimization strategies, profiling techniques, and performance targets.

**Use when:** Optimizing performance or investigating performance issues.

### milk_tech_spec.md

Original technical specification document describing the architecture, components, and design decisions.

**Use when:** Understanding the overall system architecture and design.

### TASK_20_SUMMARY.md

Summary of the build and packaging implementation (Task 20), including all configuration changes and created files.

**Use when:** Understanding what was implemented for build and packaging.

## 🔧 Related Resources

### Scripts

- `scripts/verify-build.ps1` - Automated build verification
- `scripts/create-portable.ps1` - Portable distribution creation
- `scripts/create-portable.sh` - Portable distribution (Unix)

### Configuration Files

- `src-tauri/tauri.conf.json` - Tauri application configuration
- `src-tauri/Cargo.toml` - Rust package configuration
- `src-tauri/wix/file-associations.wxs` - WiX file associations
- `package.json` - Node.js package configuration

### Specifications

- `.kiro/specs/milk-player/requirements.md` - Feature requirements
- `.kiro/specs/milk-player/design.md` - Feature design
- `.kiro/specs/milk-player/tasks.md` - Implementation tasks

## 🗺️ Documentation Structure

See **[DOCUMENTATION_STRUCTURE.md](DOCUMENTATION_STRUCTURE.md)** for a visual overview of how all documentation files relate to each other, including:

- Documentation hierarchy
- Document relationships
- Cross-reference matrix
- Navigation tips for different roles

## 📝 Contributing to Documentation

When adding new documentation:

1. Place it in the `docs/` directory
2. Add an entry to this README with a clear description
3. Link to it from relevant documents
4. Update the root README.md if it's a primary document
5. Update [DOCUMENTATION_STRUCTURE.md](DOCUMENTATION_STRUCTURE.md) with new relationships

## 🎯 Requirements Reference

Key requirements addressed by the build system:

- **Requirement 8.1**: Executable size <15MB
- **Requirement 8.2**: RAM usage <100MB (idle)
- **Requirement 8.3**: Startup time <2 seconds

See [Technical Specification](milk_tech_spec.md) for complete requirements.
