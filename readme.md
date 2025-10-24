0-Shell: Lexer & Parser Deep Dive
Project Overview

0-Shell is a minimalist Unix-like shell implemented in Rust, featuring a complete from-scratch implementation of lexical analysis and parsing without external dependencies.
Architecture Overview
text

Input String → Lexer → Tokens → Parser → AST → Executor

1. Lexer (Lexical Analysis)
1.1 Core Data Structures
TokenKind (src/lexer/types.rs)
rust

pub enum TokenKind {
    Word,           // Commands, arguments, filenames
    Pipe,           // | 
    RedirectOut,    // >
    RedirectIn,     // <
    RedirectAppend, // >>
    And,            // &&
    Or,             // ||
    Semicolon,      // ;
    Eof,            // End of input
}

Purpose: Categorizes what each token represents in the shell grammar.
Span (src/lexer/types.rs)
rust

pub struct Span {
    pub start_line: usize,
    pub start_col: usize, 
    pub end_line: usize,
    pub end_col: usize,
}

Purpose: Tracks exact position in source for precise error reporting.
Token (src/lexer/types.rs)
rust

pub struct Token {
    pub kind: TokenKind,
    pub value: String,
    pub span: Span,
}

Purpose: Combines type, content, and location information.
1.2 Lexer State Machine
LexerState (src/lexer/lexer.rs)
rust

enum LexerState {
    Normal,        // Processing regular input
    InQuote(char), // Inside quotes (single or double)
    Escaped,       // After backslash (escape sequence)
}

Purpose: Manages context for different parsing rules.
Lexer Struct (src/lexer/lexer.rs)
rust

pub struct Lexer {
    chars: Vec<char>, // Input characters
    pos: usize,       // Current position
    line: usize,      // Current line number  
    col: usize,       // Current column number
}

Purpose: Maintains scanning state and position tracking.
1.3 Core Lexer Methods
new() - Initialization
rust

pub fn new(input: &str) -> Self {
    Lexer {
        chars: input.chars().collect(),
        pos: 0,
        line: 1,
        col: 1,
    }
}

Purpose: Convert input string to character array for processing.
advance() - Position Management
rust

fn advance(&mut self) {
    if self.pos < self.chars.len() {
        let c = self.chars[self.pos];
        self.pos += 1;
        if c == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
    }
}

Purpose: Move to next character and update line/column tracking.
peek() - Lookahead
rust

fn peek(&self) -> Option<char> {
    if self.pos < self.chars.len() {
        Some(self.chars[self.pos])
    } else {
        None
    }
}

Purpose: Examine current character without consuming it.
peek_next() - Double Lookahead
rust

fn peek_next(&self) -> Option<char> {
    if self.pos + 1 < self.chars.len() {
        Some(self.chars[self.pos + 1])
    } else {
        None
    }
}

Purpose: Look two characters ahead for multi-character operators.
1.4 Main Lexing Algorithm
lex() - Primary Lexing Function
rust

pub fn lex(mut self) -> Result<Vec<Token>, LexerError> {
    let mut tokens = Vec::new();
    let mut state = LexerState::Normal;
    let mut current_value = String::new();
    let mut start_line = 1;
    let mut start_col = 1;
    let mut quote_char = '\0';

    while let Some(c) = self.peek() {
        match &state {
            LexerState::Normal => {
                if self.handle_special_char(c, &mut tokens, &mut current_value, 
                                           &mut start_line, &mut start_col) {
                    continue;
                }
                // ... character processing
            }
            // ... other states
        }
    }
    // ... cleanup and error checking
}

Algorithm:

    Initialize state machine in Normal state

    Process each character based on current state

    Handle special characters and state transitions

    Accumulate tokens and manage positions

    Final validation and EOF token addition

handle_special_char() - Operator Detection
rust

fn handle_special_char(&mut self, c: char, tokens: &mut Vec<Token>, 
                      current_value: &mut String, start_line: &mut usize, 
                      start_col: &mut usize) -> bool {
    match c {
        '|' => {
            self.push_word_if_exists(tokens, current_value, *start_line, *start_col);
            if let Some('|') = self.peek_next() {
                tokens.push(Token::new(TokenKind::Or, "||".to_string(), ...));
                self.advance(); // consume second |
            } else {
                tokens.push(Token::new(TokenKind::Pipe, "|".to_string(), ...));
            }
            self.advance();
            *start_line = self.line;
            *start_col = self.col;
            true
        }
        // ... other operators
    }
}

Purpose: Detect and handle shell operators, including multi-character ones.
State Transition Logic

Normal State:

    Whitespace: End current word, reset position tracking

    Quotes (", '): Push current word, enter InQuote state

    Escape (\): Enter Escaped state

    Operators: Handle via handle_special_char()

    Regular chars: Accumulate in current_value

InQuote State:

    Matching quote: Push quoted content, return to Normal

    Escape character: Enter Escaped state

    Other characters: Accumulate in current_value (spaces preserved!)

Escaped State:

    Any character: Add literally to current_value, return to previous state

1.5 Key Lexer Features

    Complete Quote Handling: Supports both single and double quotes with escape sequences

    Precise Error Locations: Span tracking enables exact error reporting

    Multi-character Operators: Handles ||, &&, >> correctly

    Whitespace Sensitivity: Different rules inside/outside quotes

    Robust Error Handling: Clear error types for different failure modes

2. Parser (Syntax Analysis)
2.1 Abstract Syntax Tree (AST)
Core AST Types (src/parser/types.rs)
rust

pub enum Command {
    Simple(SimpleCommand),     // echo hello world
    Pipe(PipeCommand),         // cmd1 | cmd2
    Redirect(RedirectCommand), // cmd > file
    And(AndCommand),           // cmd1 && cmd2  
    Or(OrCommand),             // cmd1 || cmd2
    Sequence(SequenceCommand), // cmd1; cmd2; cmd3
}

Purpose: Represents all possible shell command structures.
Command Variants

SimpleCommand:
rust

pub struct SimpleCommand {
    pub args: Vec<String>, // [command, arg1, arg2, ...]
}

Examples: ls, echo hello, cat file.txt

PipeCommand:
rust

pub struct PipeCommand {
    pub left: Box<Command>,  // Left side of pipe
    pub right: Box<Command>, // Right side of pipe
}

Examples: ls | grep rs, cat file | sort | uniq

RedirectCommand:
rust

pub struct RedirectCommand {
    pub command: Box<Command>,
    pub operator: RedirectOp, // >, <, >>
    pub filename: String,     // Target file
}

Examples: echo test > file.txt, sort < input.txt

Logical Commands:
rust

pub struct AndCommand {
    pub left: Box<Command>,  // First command
    pub right: Box<Command>, // Second command (runs if first succeeds)
}

pub struct OrCommand {
    pub left: Box<Command>,  // First command  
    pub right: Box<Command>, // Second command (runs if first fails)
}

Examples: cd /tmp && pwd, false || echo "failed"

SequenceCommand:
rust

pub struct SequenceCommand {
    pub commands: Vec<Command>, // [cmd1, cmd2, cmd3, ...]
}

Examples: echo one; echo two; echo three
2.2 Parser Implementation
Parser Struct (src/parser/parser.rs)
rust

pub struct Parser {
    tokens: Vec<Token>, // Input tokens from lexer
    pos: usize,         // Current token position
}

Parser Helper Methods

check() - Token Lookahead:
rust

fn check(&self, kind: TokenKind) -> bool {
    !self.is_at_end() && self.tokens[self.pos].kind == kind
}

Purpose: Examine current token without consuming it.

advance() - Token Consumption:
rust

fn advance(&mut self) -> &Token {
    if !self.is_at_end() { self.pos += 1; }
    &self.tokens[self.pos - 1]
}

Purpose: Consume current token and move to next.

match_token() - Conditional Consumption:
rust

fn match_token(&mut self, kind: TokenKind) -> bool {
    if self.check(kind) {
        self.advance();
        true
    } else {
        false
    }
}

Purpose: Consume token only if it matches expected kind.
2.3 Recursive Descent Parsing

The parser implements a top-down recursive descent algorithm where operator precedence is encoded in the call hierarchy:
text

parse_sequence()     // ; (lowest precedence)
  parse_logical()    // &&, ||
    parse_pipe()     // | 
      parse_redirect() // >, <, >>
        parse_simple()  // WORD (highest precedence)

parse() - Entry Point
rust

pub fn parse(&mut self) -> Result<Command, ParseError> {
    let command = self.parse_sequence()?;
    
    // Validate entire input was consumed
    if !self.is_at_end() && !self.check(TokenKind::Eof) {
        return Err(ParseError::UnexpectedToken(...));
    }
    
    Ok(command)
}

parse_sequence() - Command Sequences
rust

fn parse_sequence(&mut self) -> Result<Command, ParseError> {
    let command = self.parse_logical()?;
    
    if self.match_token(TokenKind::Semicolon) {
        let mut commands = vec![command];
        
        while !self.is_at_end() && !self.check(TokenKind::Eof) {
            commands.push(self.parse_logical()?);
            if !self.match_token(TokenKind::Semicolon) {
                break;
            }
        }
        
        Ok(Command::Sequence(SequenceCommand { commands }))
    } else {
        Ok(command)
    }
}

Grammar: sequence : and_or ( ';' and_or )*
Purpose: Handle command sequences separated by semicolons.
parse_logical() - Boolean Operators
rust

fn parse_logical(&mut self) -> Result<Command, ParseError> {
    let mut command = self.parse_pipe()?;
    
    while self.check(TokenKind::And) || self.check(TokenKind::Or) {
        let operator_kind = self.current().kind.clone();
        self.advance();
        
        let right = self.parse_pipe()?;
        
        command = match operator_kind {
            TokenKind::And => Command::And(AndCommand {
                left: Box::new(command),
                right: Box::new(right),
            }),
            TokenKind::Or => Command::Or(OrCommand {
                left: Box::new(command),
                right: Box::new(right),
            }),
            _ => unreachable!(),
        };
    }
    
    Ok(command)
}

Grammar: and_or : pipe ( ('&&' | '||') pipe )*
Purpose: Handle logical AND/OR operators with left-associativity.
parse_pipe() - Pipeline Commands
rust

fn parse_pipe(&mut self) -> Result<Command, ParseError> {
    let mut command = self.parse_redirect()?;
    
    while self.match_token(TokenKind::Pipe) {
        let right = self.parse_redirect()?;
        
        command = Command::Pipe(PipeCommand {
            left: Box::new(command),
            right: Box::new(right),
        });
    }
    
    Ok(command)
}

Grammar: pipe : redirect ( '|' redirect )*
Purpose: Handle command pipelines with left-associativity.
parse_redirect() - I/O Redirection
rust

fn parse_redirect(&mut self) -> Result<Command, ParseError> {
    let mut command = self.parse_simple()?;
    
    while self.check(TokenKind::RedirectOut) || 
          self.check(TokenKind::RedirectIn) || 
          self.check(TokenKind::RedirectAppend) {
        
        let operator_token = self.advance();
        let operator = RedirectOp::from_token(&operator_token)?;
        
        if !self.check(TokenKind::Word) {
            return Err(ParseError::UnexpectedToken(..., Expected::Filename));
        }
        
        let filename = self.advance().value.clone();
        
        command = Command::Redirect(RedirectCommand {
            command: Box::new(command),
            operator,
            filename,
        });
    }
    
    Ok(command)
}

Grammar: redirect : simple ( redirect_op WORD )*
Purpose: Handle input/output redirection operators.
parse_simple() - Basic Commands
rust

fn parse_simple(&mut self) -> Result<Command, ParseError> {
    let start_pos = self.pos;
    let (command, consumed) = simple::parse_simple(&self.tokens[start_pos..])?;
    self.pos += consumed;
    Ok(command)
}

Grammar: simple : WORD+
Purpose: Parse simple commands consisting of words and arguments.
2.4 Simple Command Parser (src/parser/simple.rs)
rust

pub fn parse_simple(tokens: &[Token]) -> Result<(Command, usize), ParseError> {
    let mut args = Vec::new();
    let mut pos = 0;

    while pos < tokens.len() {
        match &tokens[pos].kind {
            TokenKind::Word => {
                args.push(tokens[pos].value.clone());
                pos += 1;
            }
            TokenKind::Eof => break,
            // Stop at operators for higher-level parsers
            TokenKind::Pipe | TokenKind::RedirectOut | TokenKind::RedirectIn |
            TokenKind::RedirectAppend | TokenKind::And | TokenKind::Or | TokenKind::Semicolon => {
                break;
            }
        }
    }

    if args.is_empty() {
        return Err(ParseError::EmptyCommand);
    }

    Ok((Command::Simple(SimpleCommand { args }), pos))
}

Purpose: Collect consecutive word tokens into a simple command.
2.5 Error Handling
ParseError Types
rust

pub enum ParseError {
    UnexpectedToken(Token, Expected), // Wrong token encountered
    UnexpectedEof,                    // Premature end of input
    UnmatchedPipe,                    // Pipe with missing command
    InvalidRedirect,                  // Malformed redirection
    EmptyCommand,                     // No command specified
}

Expected Context
rust

pub enum Expected {
    Command,
    Argument, 
    Filename,
    Operator(String), // Specific operator expected
}

3. Key Technical Achievements
3.1 Lexer Innovations

    State Machine Design: Clean separation between Normal, InQuote, and Escaped states

    Precise Position Tracking: Line/column tracking enables excellent error messages

    Quote Handling: Proper handling of nested quotes and escape sequences

    Operator Recognition: Support for multi-character operators (||, &&, >>)

3.2 Parser Innovations

    Recursive Descent Architecture: Clear mapping from grammar to code

    Operator Precedence: Natural encoding through call hierarchy

    Left-Associative Parsing: Correct handling of chained operators

    Comprehensive AST: Support for all major shell constructs

    Robust Error Recovery: Fail-fast with precise error locations

3.3 Code Quality Features

    Complete Test Coverage: Unit tests for all parsing edge cases

    Clean Error Messages: User-friendly syntax error reporting

    Modular Design: Separate modules for different parser components

    Type Safety: Rust's type system prevents many common parsing errors

4. Example Parsing Traces
Example 1: echo "hello world" | grep hello

Lexer Output:
text

Word("echo")
Word("hello world") 
Pipe("|")
Word("grep")
Word("hello")
Eof

Parser Output:
rust

PipeCommand {
    left: SimpleCommand { args: ["echo", "hello world"] },
    right: SimpleCommand { args: ["grep", "hello"] }
}

Example 2: ls -l > output.txt && cat output.txt

Parser Output:
rust

AndCommand {
    left: RedirectCommand {
        command: SimpleCommand { args: ["ls", "-l"] },
        operator: RedirectOp::Output,
        filename: "output.txt"
    },
    right: SimpleCommand { args: ["cat", "output.txt"] }
}

5. Contribution Highlights

This implementation represents a complete from-scratch implementation of:

    Full lexical analysis with state machine

    Recursive descent parser with proper operator precedence

    Comprehensive AST supporting all major shell features

    Professional error handling with precise location tracking

    Modular, testable architecture following Rust best practices

The lexer and parser together form a solid foundation for a production-quality shell, demonstrating deep understanding of parsing theory and systems programming concepts.