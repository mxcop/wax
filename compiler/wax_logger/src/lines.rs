// Functions below were sourced from `https://github.com/vallentin/line-span/blob/master/src/lib.rs`

/// For an index within a string find the starting index of it's line.
pub fn find_line_start(text: &str, idx: usize) -> usize {
  text[..idx].rfind('\n').map_or(0, |i| i + 1)
}

/// For an index within a string find the ending index of it's line.
pub fn find_line_end(text: &str, idx: usize) -> usize {
  let end = text[idx..]
      .find('\n')
      .map_or_else(|| text.len(), |i| idx + i);

  if (end > 0) && (text.as_bytes()[end - 1] == b'\r') {
      end - 1
  } else {
      end
  }
}

/// For an index within a string get the amount of lines before it.
pub fn get_line_num(text: &str, idx: usize) -> usize {
  text[..idx].chars().filter(|ch| *ch == '\n').count()
}
