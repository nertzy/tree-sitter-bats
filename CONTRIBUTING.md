# Contributing

Thanks for helping improve tree-sitter-bats.

## Development

You need the tree-sitter CLI 0.27.0, Node.js, and a C compiler. The CLI version is pinned exactly in `package.json` and in CI, so every contributor regenerates `src/` the same way. The npm `tree-sitter-cli` package provides that version through `npx tree-sitter`.

```sh
npm ci
npx tree-sitter generate
npx tree-sitter test
npm run lint
```

`npx eslint --fix grammar.js` fixes most lint findings.

Edit `grammar.js`, never the generated `src/` files, and commit the regenerated `src/` alongside the grammar change. CI fails when the committed parser differs from a fresh `tree-sitter generate`. Add or update corpus tests in `test/corpus/` for every change in parsing behavior.

Pull requests should explain the problem, the chosen behavior, and any user-visible documentation changes.

## Reporting bugs and proposing changes

Use [GitHub issues](https://github.com/nertzy/tree-sitter-bats/issues) for reproducible bugs and focused proposals. Include the smallest Bats snippet that misparses, the `tree-sitter parse` output, and the tree you expected. For vulnerabilities, follow [SECURITY.md](SECURITY.md) instead of opening a public issue.
