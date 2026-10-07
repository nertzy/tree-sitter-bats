# tree-sitter-bats

[![CI][ci]](https://github.com/nertzy/tree-sitter-bats/actions/workflows/ci.yml)

[Bats](https://github.com/bats-core/bats-core) (Bash Automated Testing System) grammar for [tree-sitter](https://github.com/tree-sitter/tree-sitter), extending [tree-sitter-bash](https://github.com/tree-sitter/tree-sitter-bash).

## Status

Early development. Node types may change before the first release.

## Install

No packages are published to npm, crates.io, or PyPI yet. Build from source at a pinned commit:

```sh
git clone https://github.com/nertzy/tree-sitter-bats
cd tree-sitter-bats
tree-sitter build --output bats.dylib # bats.so on Linux
```

## Parse Bats files

Every Bash construct parses as it does in tree-sitter-bash. Bats test definitions become `test_block` nodes with a `name` field (a `string`, a `raw_string`, or the unquoted words) and a `body` field (a `compound_statement`). This test

```bats
@test "addition using bc" {
  result="$(echo 2+2 | bc)"
  [ "$result" -eq 4 ]
}
```

parses as (abbreviated):

```text
(program
  (test_block
    name: (string (string_content))
    body: (compound_statement
      (variable_assignment ...)
      (test_command ...))))
```

`setup`, `teardown`, `setup_file`, and `teardown_file` are ordinary `function_definition` nodes, and `run`, `load`, and `bats_require_minimum_version` are ordinary `command` nodes.

### With ast-grep

Register the compiled library as an [ast-grep custom language](https://ast-grep.github.io/advanced/custom-language.html). Bash reads `$NAME` as an expansion, so set an `expandoChar` for metavariables:

```yaml
# sgconfig.yml
customLanguages:
  bats:
    libraryPath: path/to/bats.dylib
    extensions: [bats]
    expandoChar: _
```

```sh
ast-grep run --lang bats --pattern '@test $NAME { $$$BODY; }'
ast-grep scan --inline-rules '{ id: test-names, language: bats, rule: { kind: test_block } }'
```

## Highlighting

`queries/highlights.scm` covers the Bash constructs plus Bats syntax: `@test`, test names, the `setup` and `teardown` hooks, helpers such as `run`, `load`, and `skip`, and the variables Bats sets (`$status`, `$output`, `$lines`, `$stderr`, `$stderr_lines`, and `$BATS_*`). `tree-sitter.json` points editors and `tree-sitter highlight` at it.

No two patterns capture the same node, so editors that prefer the first matching pattern (tree-sitter-highlight) and those that prefer the last (Zed) highlight the same way. The Rust binding test enforces this on `test/highlight/` and, once `script/fetch-examples` has fetched them, on the example suites; CI runs it on both. `tree-sitter test` runs the assertions in `test/highlight/`.

## Scope and limitations

Bats test files are Bash scripts with extra syntax. This grammar parses them by extending tree-sitter-bash 0.25.1, so anything tree-sitter-bash misparses is misparsed here too unless this grammar works around it.

Worked around:

- A `[ ... ]` test, later commands with a redirect, and a following `[ ... ]` merge into one `test_command` without an `ERROR` node ([tree-sitter-bash#316](https://github.com/tree-sitter/tree-sitter-bash/issues/316)). Here a redirect inside single brackets, as in `[ ! command -v go &>/dev/null ]`, must be a command with single-word redirect targets followed by `]`, and the separate-statements parse wins any remaining ambiguity.
- A herestring after a file or heredoc redirect, as in `cat > file <<<"text"`, produces an `ERROR` ([tree-sitter-bash#232](https://github.com/tree-sitter/tree-sitter-bash/issues/232), [#282](https://github.com/tree-sitter/tree-sitter-bash/issues/282)). Here a herestring may follow the first file or heredoc redirect; a herestring before it still belongs to the command.

Not yet worked around:

- A line with a redirect after a pipeline of three or more commands merges into the pipeline's last command (same issue, [comment](https://github.com/tree-sitter/tree-sitter-bash/issues/316)).
- A heredoc whose line continues past the opener with `;`, `&`, or more than one further statement, as in `cat <<EOF; echo next` or `cat <<EOF | grep x; echo y`, produces an `ERROR`. Proposed fixes: [tree-sitter-bash#355](https://github.com/tree-sitter/tree-sitter-bash/pull/355) and [#356](https://github.com/tree-sitter/tree-sitter-bash/pull/356).
- More than one heredoc opened on a line, as in `cat <<A <<B` or `cat <<A; cat <<B`, produces an `ERROR`.
- An argument after a herestring that follows a file redirect, as in `cat > out <<< x -`, produces an `ERROR`.
- The read-write redirect `<>` produces an `ERROR` ([tree-sitter-bash#352](https://github.com/tree-sitter/tree-sitter-bash/issues/352)).
- A quoted string containing a space inside a glob after `=` in `[[ ... ]]`, as in `[[ $output = *"a b"* ]]`, produces an `ERROR` ([tree-sitter-bash#321](https://github.com/tree-sitter/tree-sitter-bash/issues/321)); `==` parses.
- A double-quoted string followed by a group in a `=~` regex, as in `[[ $output =~ "a "([^ ]+) ]]`, produces an `ERROR`.
- An escaped backtick inside a quoted glob in `[[ ... ]]`, as in `` [[ $output = *"\`cmd\`"* ]] ``, produces an `ERROR`.
- Bats also accepts a function marked as a test with a trailing comment (`name() { # @test`); it parses as an ordinary `function_definition`.

CI parses the `.bats` files of several public Bats suites at pinned commits. `script/known-failures.txt` lists the files that still produce `ERROR` nodes; each traces to one of the limitations above or to a deliberately malformed fixture.

## Development

Requires the tree-sitter CLI 0.27.0 (pinned in `package.json` and CI), Node.js, and a C compiler. See [CONTRIBUTING.md](CONTRIBUTING.md).

## References

- [Bats documentation](https://bats-core.readthedocs.io/)
- [Writing tests in Bats](https://bats-core.readthedocs.io/en/stable/writing-tests.html)
- [Creating tree-sitter parsers](https://tree-sitter.github.io/tree-sitter/creating-parsers/)

## License

MIT. See [LICENSE](LICENSE).

[ci]: https://img.shields.io/github/actions/workflow/status/nertzy/tree-sitter-bats/ci.yml?logo=github&label=CI
