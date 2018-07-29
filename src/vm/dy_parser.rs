use std::rc::Weak;
use std::rc::Rc;
use std::cell::RefCell;

//pub enum BlockState {
//	None,
//	CommentBlock,
//	StringBlock,
//}
//
//pub enum RegionKind {
//	None,
//	Region,
//	If,
//	Elif,
//	Else,
//	LastActive,
//	InactiveRegion,
//	InactiveIf,
//	InactiveElif,
//	InactiveElse,
//}
//
//pub struct RegionTree {
//	kind: RegionKind,
//	line: FormatedLine,
//    parent: Option<Weak<RefCell<Box<RegionTree>>>>,
//	children: Option<Vec<Rc<RefCell<Box<RegionTree>>>>>,
//}
//
//
//pub struct FormatedLine {
//	block_state: BlockState,
//    region_tree: Option<Rc<RegionTree>>,
//    index: i32,
//    tokens: Option<Vec<SyntaxToken>>,
//}
//
//pub enum TokenKind {
//	Missing,
//	Whitespace,
//	Comment,
//	Preprocessor,
//	PreprocessorArguments,
//	PreprocessorSymbol,
//	PreprocessorDirectiveExpected,
//	PreprocessorCommentExpected,
//	PreprocessorUnexpectedDirective,
//	VerbatimStringLiteral,
//
//	LastWSToken, // Marker only
//
//	VerbatimStringBegin,
//	BuiltInLiteral,
//	CharLiteral,
//	StringLiteral,
//	IntegerLiteral,
//	RealLiteral,
//	Punctuator,
//	Keyword,
//	Identifier,
//	ContextualKeyword,
//	EOF,
//}
//
//pub struct SyntaxToken {
//	pub kind: TokenKind,
//    pub text: str,
//
//}
//
//fn scan_char_literal(line: str, start: & mut i32) -> SyntaxToken {
//
//}
//
//fn scan_whitespace(line: str, start: & mut i32) -> SyntaxToken {
//
//}
//
//const KEY_WORDS: [&'static str; 58] = ["abstract", "as", "base", "break", "case", "catch", "checked", "class", "const", "continue",
//		"default", "delegate", "do", "else", "enum", "event", "explicit", "extern", "finally",
//		"fixed", "for", "foreach", "goto", "if", "implicit", "in", "interface", "internal", "is",
//		"lock", "namespace", "new", "operator", "out", "override", "params", "private",
//		"protected", "public", "readonly", "ref", "return", "sealed", "sizeof", "stackalloc", "static",
//		"struct", "switch", "this", "throw", "try", "typeof", "unchecked", "unsafe", "using", "virtual",
//		"volatile", "while"];
//
//const OPERATORS: [&'static str; 44] = ["++", "--", "->", "+", "-", "!", "~", "++", "--", "&", "*", "/", "%", "+", "-", "<<", ">>", "<", ">",
//		"<=", ">=", "==", "!=", "&", "^", "|", "&&", "||", "??", "?", "::", ":",
//		"=", "+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=", "<<=", ">>=", "=>"];
//
//const BUILT_TYPES: [&'static str; 16] = ["bool", "byte", "char", "decimal", "double", "float", "int", "long", "object", "sbyte", "short",
//		"string", "uint", "ulong", "ushort", "void"];
//
//const PREPROCESSOR_KEY_WORLDS: [&'static str; 12] = ["define", "elif", "else", "endif", "endregion", "error", "if", "line", "pragma", "region", "undef", "warning"];
//
//pub struct DyParser<'a> {
//	pub lines: Vec<&'a str>,
////	pub root_region: RegionTree<'a>,
//}
//
//impl<'a> DyParser<'a> {
//	pub fn parser(&self, code_content : & String) {
//		self.lines = code_content.replace("\r\n", "\n").replace("\r", "\n").split("\n").collect();
//	}
//
//	pub fn parse_line() {
//
//	}
//
//	pub fn lex_line() {
//
//	}
//
//	pub fn tokenize() {
//
//	}
//}

#[derive(Debug)]
enum TokenKind {
    Missing,
    Whitespace,
    Comment,
    Preprocessor,
    PreprocessorArguments,
    PreprocessorSymbol,
    PreprocessorDirectiveExpected,
    PreprocessorCommentExpected,
    PreprocessorUnexpectedDirective,
    VerbatimStringLiteral,

    LastWSToken, // Marker only

    VerbatimStringBegin,
    BuiltInLiteral,
    CharLiteral,
    StringLiteral,
    IntegerLiteral,
    RealLiteral,
    Punctuator,
    Keyword,
    Identifier,
    ContextualKeyword,
    EOF,

}

#[derive(Debug)]
struct SyntaxToken {
    kind: TokenKind,


}

#[derive(Debug)]
pub struct DyParser {
    source: Vec<char>,
    formatted_lines: Vec<FormatedLine>,

}

impl DyParser {
    pub fn new(code : String) -> DyParser  {
        let source = code.chars().collect();
        DyParser {
            source,
            formatted_lines: Vec::new(),
        }
    }

    pub fn format(&mut self) {
        let mut has_line_char = false;
        let mut pre_line_begin= 0;
        let mut formated_line_count = 0;
        for (index, ch) in self.source.iter().enumerate() {
            if *ch == '\r' || *ch == '\n' {
                has_line_char = true;
            }
            else {
                if has_line_char {
                    let pre_line_end: i32 = (index as i32) - 1;
                    let formated_line = FormatedLine::new(formated_line_count, pre_line_begin, pre_line_end);
                    self.formatted_lines.push(formated_line);
                    formated_line_count += 1;
                    pre_line_begin = index as i32;
                }
                has_line_char = false;
            }
        }
    }
}

#[derive(Debug)]
struct FormatedLine {
    index: i32,
    begin: i32,
    end: i32,
    tokens: Vec<SyntaxToken>,
}

impl FormatedLine {
    fn new(index: i32, begin: i32, end: i32) -> FormatedLine {
        FormatedLine{
            index,
            begin,
            end,
            tokens: Vec::new(),
        }
    }
}