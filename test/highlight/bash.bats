@test "bash constructs" {
  local count=$((BATS_TEST_NUMBER + 2))
  # <- keyword
  #     ^ variable
  #          ^ operator
  #           ^ punctuation.special
  #                                 ^ number
  export PATH="$PWD/bin:$PATH"
  # <- keyword
  for f in *.txt; do
  # <- keyword
  #     ^ keyword
  #             ^ punctuation.delimiter
  #               ^ keyword
    if [ -f "$f" ]; then
    # <- keyword
    #  ^ punctuation.bracket
    #    ^ operator
      grep -c x "$f" | wc -l 2>/dev/null
      # <- function
      #    ^ constant
      #              ^ operator
      #                      ^ number
      #                       ^ operator
    fi
    # <- keyword
  done
  # <- keyword
  case "$1" in
  # <- keyword
    -h|--help|v[0-9]*) ;;
    # <- string.regex
    # ^ operator
    #  ^ string.regex
    #         ^ string.regex
    a) echo $'tab\t' ;;
    #       ^ string
    #                ^ punctuation.delimiter
  esac
  out=$(date) && sleep 1 &
  #   ^ punctuation.special
  #     ^ function
  #              ^ function
  #                      ^ punctuation.delimiter
  cat <<EOF >"$out"
hello $out
EOF
# <- string.special
  cat <<< 'herestring' > out
  #   ^ operator
  #       ^ string
}
