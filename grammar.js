/**
 * @file Bats (Bash Automated Testing System) grammar for tree-sitter, extending tree-sitter-bash.
 * @author Grant Hutchins <git@nertzy.com>
 * @license MIT
 */

/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

import bash from 'tree-sitter-bash/grammar.js';

export default grammar(bash, {
  name: 'bats',

  rules: {
    _statement_not_subshell: ($, original) => choice(
      $.test_block,
      original,
    ),

    // `@test <name> { ... }`, which Bats rewrites into a test function.
    // Like Bats, the name may be quoted or a run of unquoted words.
    test_block: $ => seq(
      '@test',
      field('name', repeat1($._literal)),
      field('body', alias($._test_body, $.compound_statement)),
    ),

    _test_body: $ => seq(
      '{',
      optional($._terminated_statement),
      token(prec(-1, '}')),
    ),

    // tree-sitter-bash accepts any redirected statement between `[` and `]`
    // (`[ ! command -v go &>/dev/null ]`). Because newlines between words are
    // extras and a file redirect takes any number of destination words, that
    // branch can swallow `[ ... ]`, the commands after it, and a later
    // `[ ... ]` as one test_command, without an ERROR. Narrow the branch to a
    // command with single-destination file redirects closed by `]`, and
    // penalize it so separate statements win any remaining tie.
    // tree-sitter-bash only accepts a herestring before a statement's file
    // and heredoc redirects, so `cat > file <<< text` produces an ERROR
    // (tree-sitter-bash#232). Accept herestrings anywhere after the first
    // file or heredoc redirect; a herestring before them still belongs to
    // the command.
    redirected_statement: $ => prec.dynamic(-1, prec.right(-1, choice(
      seq(
        field('body', $._statement),
        field('redirect', choice($.file_redirect, $.heredoc_redirect)),
        field('redirect', repeat(choice(
          $.file_redirect,
          $.heredoc_redirect,
          $.herestring_redirect,
        ))),
      ),
      seq(
        field('body', choice($.if_statement, $.while_statement)),
        $.herestring_redirect,
      ),
      field('redirect', repeat1($._redirect)),
      $.herestring_redirect,
    ))),

    test_command: $ => choice(
      seq('[', optional($._expression), ']'),
      prec.dynamic(-10, seq(
        '[',
        alias($._test_redirected_statement, $.redirected_statement),
        ']',
      )),
      seq(
        '[[',
        choice(
          $._expression,
          alias($._test_command_binary_expression, $.binary_expression),
        ),
        ']]',
      ),
    ),

    _test_redirected_statement: $ => seq(
      field('body', choice($.command, $.negated_command)),
      field('redirect', repeat1(alias($._test_file_redirect, $.file_redirect))),
    ),

    _test_file_redirect: $ => seq(
      field('descriptor', optional($.file_descriptor)),
      choice('<', '>', '>>', '&>', '&>>', '<&', '>&', '>|'),
      field('destination', $._literal),
    ),
  },
});
