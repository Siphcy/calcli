
pub fn insert_implicit_multiplication(input: &str) -> String {
      let mut exempt_bracket_depth = 0;
      let mut result = String::new();
      let mut chars = input
        .chars()
        .filter(|c| !c.is_whitespace())
        .peekable();

      if chars.peek() == Some(&'.'){
        result.push('0');
        }

      while let Some(c) = chars.next() {

         if c == '[' {
            // Add implicit multiplication before '[' if needed
            if let Some(last) = result.chars().last() {
                if last.is_ascii_digit() || last == ')' {
                    result.push('*');
                }
            }
            result.push('[');
            exempt_bracket_depth += 1;
            continue;
          }

         if c ==']' {
            result.push(']');
            exempt_bracket_depth -= 1;
            // Add implicit multiplication after ']' if needed
            if let Some(&next) = chars.peek(){
                if next.is_alphanumeric() || !next.is_ascii() || next == '(' || next == '[' || next == '.'{
                result.push('*');
                }
            }
            continue;
        }

        result.push(c);
        if exempt_bracket_depth > 0 {
            continue;
        }

          if let Some(&next) = chars.peek() {

            if (c.is_ascii_digit() || c == '.') && (next.is_alphabetic() || !next.is_ascii()) {
                result.push('*');
            }

            if c.is_ascii_digit() && (next == '(' || next == '[') {
                result.push('*');
            }

            if c == ')' && (next.is_alphanumeric() || !next.is_ascii() || next == '(' || next == '[' || next == '.') {
                result.push('*');
            }

            if (!c.is_ascii_digit()) && (next == '.') {
                result.push('0');
            }
          }
      }

      result
  }
