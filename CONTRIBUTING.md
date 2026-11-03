# Contributing

Thanks for helping improve tree-sitter-bats.

## Development

You need the tree-sitter CLI 0.27.1, Node.js, and a C compiler. The CLI version is pinned exactly in `package.json` and in CI, so every contributor regenerates `src/` the same way. The npm `tree-sitter-cli` package provides that version through `npx tree-sitter`.

```sh
npm ci
npx tree-sitter generate
npx tree-sitter test
npm run lint
```

`npx eslint --fix grammar.js` fixes most lint findings.

Edit `grammar.js`, never the generated `src/` files, and commit the regenerated `src/` alongside the grammar change. CI fails when the committed parser differs from a fresh `tree-sitter generate`. Add or update corpus tests in `test/corpus/` for every change in parsing behavior.

### Highlight queries

`queries/highlights.scm` gets assertions in `test/highlight/`, which `tree-sitter test` runs. Keep every node to one capture: when two patterns share a node type, split it with complementary predicates (`#match?` and `#not-match?` on the same regex). `cargo test` fails on any node captured twice, in `test/highlight/` and, after `script/fetch-examples`, in the example suites.

### Neovim queries

`queries/neovim/` holds the queries nvim-treesitter installs. They use [nvim-treesitter's captures](https://github.com/nvim-treesitter/nvim-treesitter/blob/main/CONTRIBUTING.md) and predicates, and each starts with `; inherits: bash`, so Neovim applies nvim-treesitter's Bash queries first and these on top. To try a change, register the parser as the README shows, with `path = '/path/to/tree-sitter-bats'` in place of `url` and `revision`; nvim-treesitter then links the queries from your clone.

### Checking queries

CI checks all of `queries/` with [ts_query_ls](https://github.com/ribru17/ts_query_ls) 3.16.0, which rejects node types, fields, and predicates the parser doesn't have, and enforces its formatting. `script/check-queries` runs it after fetching nvim-treesitter's Bash queries at a pinned commit, so the check also covers every Bash pattern the Neovim queries inherit. Bump that commit to check against newer Bash queries. Build the parser into the repository root first, since `.tsqueryrc.json` looks for it there, and run from a clone named `tree-sitter-bats`, which is how ts_query_ls tells which language the queries are for:

```sh
npx tree-sitter build
script/check-queries
```

`ts_query_ls format queries/` fixes the formatting.

### Example suites

CI also parses the `.bats` files of the public Bats suites in `script/example-repos.txt`, each pinned to a commit, and accepts `ERROR` nodes only in the files listed in `script/known-failures.txt`. After a grammar change or a commit bump, regenerate the list and explain every added or removed file in the pull request:

```sh
script/parse-examples
```

### Updating tree-sitter-bash

`grammar.js` extends the exact tree-sitter-bash version in `package.json`, and `src/scanner.c` is a copy of that version's external scanner with the exported function names changed from `bash` to `bats`. Bump both together: update the dependency, copy the new `src/scanner.c` with the same rename and header, regenerate, and run the corpus tests and `script/parse-examples`.

Pull requests should explain the problem, the chosen behavior, and any user-visible documentation changes.

## Releasing

See [RELEASING.md](RELEASING.md).

## Reporting bugs and proposing changes

Use [GitHub issues](https://github.com/nertzy/tree-sitter-bats/issues) for reproducible bugs and focused proposals. Include the smallest Bats snippet that misparses, the `tree-sitter parse` output, and the tree you expected. For vulnerabilities, follow [SECURITY.md](SECURITY.md) instead of opening a public issue.
