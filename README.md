# Doclink Checker

Simple application that checks that links in documentation
are still valid.

## Completed Features

- Recursively scan directories
- Highlight URLs that result in a redirect
- Colour highlighting
- Pass in the directory you want to scan

## To do

- Ability to specify a file, rather than a directory
- Automatically exclude .git directories and binaries
- Support the passing of arguments
    - Directory to check
    - `--no-check`? Something that disables the url checking and simply lists all the urls available
    - A Go-like format flag similar to Docker would be nice
