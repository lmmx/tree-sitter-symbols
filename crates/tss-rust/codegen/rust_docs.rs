pub fn get_doc_info(node_type: &str) -> Option<(&'static str, &'static str)> {
    let kw_base = "https://doc.rust-lang.org/book/appendix-01-keywords.html";
    let op_base = "https://doc.rust-lang.org/book/appendix-02-operators.html";

    // Keywords
    match node_type {
        "as" | "async" | "await" | "break" | "const" | "continue" | "crate" | "dyn" | "else"
        | "enum" | "extern" | "false" | "fn" | "for" | "if" | "impl" | "in" | "let" | "loop"
        | "match" | "mod" | "move" | "mut" | "pub" | "ref" | "return" | "Self" | "self"
        | "static" | "struct" | "super" | "trait" | "true" | "type" | "union" | "unsafe"
        | "use" | "where" | "while" => Some(("keywords", kw_base)),

        // Operators and symbols
        "!" | "!=" | "%" | "%=" | "&" | "&&" | "&=" | "*" | "*=" | "+" | "+=" | "," | "-"
        | "-=" | "->" | "." | ".." | "..=" | "..." | "/" | "/=" | ":" | ";" | "<<" | "<<="
        | "<" | "<=" | "=" | "==" | "=>" | ">" | ">=" | ">>" | ">>=" | "@" | "^" | "^=" | "|"
        | "|=" | "||" | "?" | "_" | "::" | "#" | "$" | "'" | "\"" | "(" | ")" | "[" | "]" | "{"
        | "}" | "//" | "/*" | "*/" => Some(("operators", op_base)),

        _ => None,
    }
}
