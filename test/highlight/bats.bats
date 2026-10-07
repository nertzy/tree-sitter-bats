#!/usr/bin/env bats
# <- keyword.directive
# bats file_tags=slow
# <- comment
#! not an interpreter line
# <- comment

setup() {
# <- function.builtin
  load test_helper
  # <- function.builtin
  bats_load_library bats-assert
  # <- function.builtin
}

teardown_file() { :; }
# <- function.builtin

assert_ok() { [ "$status" -eq 0 ]; }
# <- function
#                 ^ variable.special
#                         ^ operator

@test "runs ${thing}" {
# <- keyword
#     ^ string
#           ^ punctuation.special
#             ^ variable
  run -1 my_cmd --flag "$1"
  # <- function.builtin
  #             ^ constant
  #                      ^ variable.special
  [[ $output =~ ^err ]] || skip "nope"
  #     ^ variable.special
  #          ^ operator
  #             ^ string.regex
  #                     ^ operator
  #                        ^ function.builtin
  echo "${lines[0]}" "$stderr" "$BATS_TEST_TMPDIR" "$HOME" "$?"
  # <- function.builtin
  #        ^ variable.special
  #                     ^ variable.special
  #                                ^ variable.special
  #                                                   ^ variable
  #                                                          ^ variable.special
}

@test unquoted multi-word name { true; }
# <- keyword
#     ^ string
#              ^ string
#                         ^ string
#                                ^ function.builtin
