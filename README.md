# tree-sitter-bats

[![CI][ci]](https://github.com/nertzy/tree-sitter-bats/actions/workflows/ci.yml)

[Bats](https://github.com/bats-core/bats-core) (Bash Automated Testing System) grammar for [tree-sitter](https://github.com/tree-sitter/tree-sitter), extending [tree-sitter-bash](https://github.com/tree-sitter/tree-sitter-bash).

## Status

Early development. The grammar is the `tree-sitter init` placeholder; Bats syntax lands in upcoming changes. Node types will change without notice until the first release.

## Install

No packages are published to npm, crates.io, or PyPI yet. Build from source at a pinned commit:

```sh
git clone https://github.com/nertzy/tree-sitter-bats
cd tree-sitter-bats
tree-sitter build
```

## Parse Bats files

A usage example lands with the first grammar change.

## Scope and limitations

Bats test files are Bash scripts with extra syntax. This grammar parses them by extending tree-sitter-bash, so anything tree-sitter-bash misparses is misparsed here too unless this grammar works around it.

## Development

Requires the tree-sitter CLI 0.27.0 (pinned in `package.json` and CI), Node.js, and a C compiler. See [CONTRIBUTING.md](CONTRIBUTING.md).

## References

- [Bats documentation](https://bats-core.readthedocs.io/)
- [Writing tests in Bats](https://bats-core.readthedocs.io/en/stable/writing-tests.html)
- [Creating tree-sitter parsers](https://tree-sitter.github.io/tree-sitter/creating-parsers/)

## License

MIT. See [LICENSE](LICENSE).

[ci]: https://img.shields.io/github/actions/workflow/status/nertzy/tree-sitter-bats/ci.yml?logo=github&label=CI
