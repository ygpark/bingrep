# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a C# .NET Framework 4.6.1 command-line utility called **bingrep** that performs binary pattern searching using regular expressions on files, physical drives, and EWF (E01) forensic image files. The project is written in Korean with Korean comments and documentation.

## Build Commands

```bash
# Build Debug configuration for x86
msbuild bingrep.sln /p:Configuration=Debug /p:Platform=x86

# Build Release configuration for x86
msbuild bingrep.sln /p:Configuration=Release /p:Platform=x86
```

## Architecture

The solution consists of two projects:

1. **bingrep** - Main console application (bingrep/bingrep.csproj)
   - Entry point: bingrep/Program.cs
   - Handles command-line arguments and orchestrates searches

2. **GhostYak** - Library project (GhostYak/GhostYak.csproj)
   - Contains all core functionality including:
     - Binary regex implementation using Re2.Net library
     - Physical disk access and raw I/O operations
     - Support for various disk image formats (DD, EWF/E01)
     - Low-level Windows device I/O control wrappers

## Key Components

- **BinaryRegex** (GhostYak/Text/RegularExpressions/BinaryRegex.cs) - Wrapper around Re2.Net for binary pattern matching
- **Storage Classes** (GhostYak/IO/RawDiskDrive/*) - Handle different storage types:
  - PhysicalStorage - Direct physical disk access
  - EWFStorage - Expert Witness Format (E01) files
  - DDImageStorage - Raw disk images
- **DeviceIOControl** (GhostYak/IO/DeviceIOControl/*) - Windows device control structures and wrappers

## Dependencies

- Re2.Net.dll - Regular expression library for binary pattern matching
- ewf.net.dll - Library for reading EWF (E01) forensic image files
- libewf.dll, zlib.dll - Native dependencies for EWF support (copied during post-build)

## Platform Requirements

- x86 architecture only (no x64 configuration)
- Administrator privileges required for physical disk access
- Windows-specific (uses Windows API for device I/O)