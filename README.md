# Doclink Checker

[![Build](https://github.com/labooner/doclink-checker/workflows/Build/badge.svg)](https://github.com/labooner/doclink-checker/actions?query=workflow%3ABuild)
[![Security Audit](https://github.com/labooner/doclink-checker/workflows/Security%20audit/badge.svg)](https://github.com/labooner/doclink-checker/actions?query=workflow%3A%22Security+audit%22)
![GitHub release (latest SemVer)](https://img.shields.io/github/v/release/labooner/doclink-checker)

Simple application that checks that links in documentation
are still valid.

**This is a poor mans attempt at what [Lychee](https://github.com/lycheeverse/lychee) now does** - I'd recommend you check out their project!

## Completed Features

- Scan files or recursively scan directories
- Highlight URLs that result in a redirect
- Colour highlighting

## To do

- Automatically exclude .git directories and binaries
    - Should probably specify a whitelist of file extensions
- `--no-check`? Something that disables the url checking and simply lists all the urls available
- `--format` flag similar to Docker would be nice
