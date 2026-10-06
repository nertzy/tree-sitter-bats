/**
 * @file Bats (Bash Automated Testing System) grammar for tree-sitter, extending tree-sitter-bash.
 * @author Grant Hutchins <git@nertzy.com>
 * @license MIT
 */

/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

export default grammar({
  name: 'bats',

  rules: {
    // TODO: add the actual grammar rules
    source_file: $ => repeat('hello'),
  },
});
