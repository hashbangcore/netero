/// Splits a command line into arguments, honoring quotes and backslash escapes.
pub fn split_args(input: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut quote: Option<char> = None;
    let mut escape = false;

    for ch in input.chars() {
        if escape {
            current.push(ch);
            escape = false;
            continue;
        }

        if ch == '\\' && quote != Some('\'') {
            escape = true;
            continue;
        }

        if let Some(q) = quote {
            if ch == q {
                quote = None;
            } else {
                current.push(ch);
            }
            continue;
        }

        if ch == '"' || ch == '\'' {
            quote = Some(ch);
            continue;
        }

        if ch.is_whitespace() {
            if !current.is_empty() {
                args.push(current.clone());
                current.clear();
            }
            continue;
        }

        current.push(ch);
    }

    if !current.is_empty() {
        args.push(current);
    }

    args
}

/// Returns true if the token looks like a file path.
pub fn looks_like_path(token: &str) -> bool {
    token.starts_with('/')
        || token.starts_with("./")
        || token.starts_with("../")
        || token.starts_with("~/")
}

/// Prefixes every line of a block, preserving its trailing newline.
pub fn indent_block(content: &str, prefix: &str) -> String {
    if content.is_empty() {
        return String::new();
    }
    let mut out = String::new();
    let mut lines = content.lines().peekable();
    while let Some(line) = lines.next() {
        out.push_str(prefix);
        out.push_str(line);
        if lines.peek().is_some() {
            out.push('\n');
        }
    }
    if content.ends_with('\n') {
        if !out.is_empty() {
            out.push('\n');
        }
        out.push_str(prefix);
    }
    out
}
