# Doclink Checker

Simple application that checks that links in documentation
are still valid.

## Completes Features

- Recursively scan directories

## To do

- Support the passing of arguments
    - Directory to check
    - `--find-only`? Something that disables the url checking and simply lists all the urls available
    - A Go-like format flag similar to Docker would be nice
- Colour highlighting
- For urls that redirect, show the final redirect url.