; inherits: bash

; `name() { # @test` marks a function that Bats runs as a test
(test_marker_comment) @comment

"@test" @keyword.function

(test_name
  (word) @string @spell)

(test_name
  [
    (string)
    (raw_string)
  ] @spell)

(function_definition
  name: (word) @function.builtin
  (#any-of? @function.builtin
    "setup" "teardown" "setup_file" "teardown_file" "setup_suite" "teardown_suite"))

(command_name
  (word) @function.builtin
  (#any-of? @function.builtin
    "run" "load" "skip" "bats_load_library" "bats_require_minimum_version" "bats_pipe"))

((variable_name) @variable.builtin
  (#any-of? @variable.builtin "status" "output" "lines" "stderr" "stderr_lines"))

((variable_name) @variable.builtin
  (#lua-match? @variable.builtin "^BATS_"))
