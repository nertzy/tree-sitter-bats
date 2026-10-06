package tree_sitter_bats_test

import (
	"testing"

	tree_sitter "github.com/tree-sitter/go-tree-sitter"
	tree_sitter_bats "github.com/nertzy/tree-sitter-bats/bindings/go"
)

func TestCanLoadGrammar(t *testing.T) {
	language := tree_sitter.NewLanguage(tree_sitter_bats.Language())
	if language == nil {
		t.Errorf("Error loading Bats grammar")
	}
}
