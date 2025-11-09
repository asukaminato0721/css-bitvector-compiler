use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut args = env::args().skip(1);
    let input = args
        .next()
        .ok_or_else(|| "usage: filter_selectors <input.css> [output.css]".to_string())?;
    let output = args.next();

    if args.next().is_some() {
        return Err("too many arguments".into());
    }

    let css = fs::read_to_string(&input)?;
    let filtered = filter_single_selectors(&css);

    if let Some(path) = output {
        fs::write(path, filtered)?;
    } else {
        print!("{filtered}");
    }

    Ok(())
}

fn filter_single_selectors(css: &str) -> String {
    let mut filtered = String::with_capacity(css.len());
    let mut cursor = 0;
    let bytes = css.as_bytes();
    let mut i = 0;
    let mut depth = 0;

    while i < bytes.len() {
        match bytes[i] {
            b'"' | b'\'' => {
                i = skip_string(bytes, i);
            }
            b'/' if i + 1 < bytes.len() && bytes[i + 1] == b'*' => {
                i = skip_comment(bytes, i);
            }
            b'{' => {
                if depth == 0 {
                    if let Some(block_end) = find_block_end(css, i) {
                        let selector_start = find_selector_start(css, i);
                        let selector_text = cleaned_selector(&css[selector_start..i]);
                        let is_basic = is_basic_selector(&selector_text);

                        let prefix = &css[cursor..selector_start];
                        if is_basic {
                            if prefix.chars().any(|c| !c.is_whitespace()) {
                                filtered.push_str(prefix);
                            }
                        } else {
                            filtered.push_str(&css[cursor..=block_end]);
                        }

                        cursor = block_end + 1;
                        i = cursor;
                        continue;
                    } else {
                        filtered.push_str(&css[cursor..]);
                        return filtered;
                    }
                }
                depth += 1;
                i += 1;
            }
            b'}' => {
                if depth > 0 {
                    depth -= 1;
                }
                i += 1;
            }
            _ => {
                i += 1;
            }
        }
    }

    if cursor < css.len() {
        filtered.push_str(&css[cursor..]);
    }

    filtered
}

fn find_selector_start(css: &str, brace_idx: usize) -> usize {
    let bytes = css.as_bytes();
    let mut start = brace_idx;
    while start > 0 {
        let c = bytes[start - 1];
        if c == b'}' || c == b';' {
            break;
        }
        start -= 1;
    }
    skip_ws_and_comments_forward(css, start, brace_idx)
}

fn skip_ws_and_comments_forward(css: &str, mut idx: usize, limit: usize) -> usize {
    let bytes = css.as_bytes();
    while idx < limit {
        match bytes[idx] {
            b' ' | b'\t' | b'\r' | b'\n' => idx += 1,
            b'/' if idx + 1 < limit && bytes[idx + 1] == b'*' => {
                idx += 2;
                while idx + 1 < limit {
                    if bytes[idx] == b'*' && bytes[idx + 1] == b'/' {
                        idx += 2;
                        break;
                    }
                    idx += 1;
                }
            }
            _ => break,
        }
    }
    idx
}

fn cleaned_selector(raw: &str) -> String {
    let mut cleaned = String::with_capacity(raw.len());
    let mut chars = raw.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '/' && chars.peek() == Some(&'*') {
            chars.next();
            while let Some(next) = chars.next() {
                if next == '*' && chars.peek() == Some(&'/') {
                    chars.next();
                    break;
                }
            }
            continue;
        }
        cleaned.push(ch);
    }

    cleaned.trim().to_string()
}

fn is_basic_selector(selector: &str) -> bool {
    if selector.is_empty() {
        return false;
    }
    if selector.starts_with('@') || selector.contains(',') {
        return false;
    }

    selector
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.' | '#'))
}

fn find_block_end(css: &str, open_idx: usize) -> Option<usize> {
    let bytes = css.as_bytes();
    let mut depth = 0usize;
    let mut i = open_idx;

    while i < bytes.len() {
        match bytes[i] {
            b'{' => {
                depth += 1;
                i += 1;
            }
            b'}' => {
                if depth == 0 {
                    return None;
                }
                depth -= 1;
                i += 1;
                if depth == 0 {
                    return Some(i - 1);
                }
            }
            b'"' | b'\'' => {
                i = skip_string(bytes, i);
            }
            b'/' if i + 1 < bytes.len() && bytes[i + 1] == b'*' => {
                i = skip_comment(bytes, i);
            }
            _ => {
                i += 1;
            }
        }
    }

    None
}

fn skip_string(bytes: &[u8], mut idx: usize) -> usize {
    let quote = bytes[idx];
    idx += 1;
    while idx < bytes.len() {
        match bytes[idx] {
            b'\\' => {
                idx += 2;
            }
            ch if ch == quote => {
                idx += 1;
                break;
            }
            _ => idx += 1,
        }
    }
    idx
}

fn skip_comment(bytes: &[u8], mut idx: usize) -> usize {
    idx += 2;
    while idx + 1 < bytes.len() {
        if bytes[idx] == b'*' && bytes[idx + 1] == b'/' {
            idx += 2;
            break;
        }
        idx += 1;
    }
    idx
}

#[cfg(test)]
mod tests {
    use super::filter_single_selectors;

    #[test]
    fn drops_basic_selectors() {
        let input = "div .a > p { color: green; }\n#b { color: red; }\n.main { color: blue; }\n";
        let output = filter_single_selectors(input);
        assert!(output.contains("div .a > p"));
        assert!(!output.contains("#b {"));
        assert!(!output.contains(".main {"));
    }

    #[test]
    fn keeps_comments_and_spacing() {
        let input = "/*comment*/\n#id { color: red; }\n";
        let output = filter_single_selectors(input);
        assert!(output.contains("/*comment*/"));
        assert!(!output.contains("#id {"));
    }

    #[test]
    fn keeps_at_rules() {
        let input = "@media screen {\n  a { color: red; }\n}\n";
        let output = filter_single_selectors(input);
        assert_eq!(output, input);
    }
}
