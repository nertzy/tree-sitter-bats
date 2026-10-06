import XCTest
import SwiftTreeSitter
import TreeSitterBats

final class TreeSitterBatsTests: XCTestCase {
    func testCanLoadGrammar() throws {
        let parser = Parser()
        let language = Language(language: tree_sitter_bats())
        XCTAssertNoThrow(try parser.setLanguage(language),
                         "Error loading Bats grammar")
    }
}
