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

/// For an index within a string get the offset on it's line.
pub fn get_char_num(text: &str, idx: usize) -> usize {
  let start = find_line_start(text, idx);
  idx - start
}

#[allow(unused)]
/// Get the entire line that contains a char. returns ``(line_num: usize, line: &str)``
pub fn get_line(file: &str, idx: usize) -> (usize, &str) {
  let start = find_line_start(file, idx);
  let end = find_line_end(file, idx);
  (get_line_num(file, idx) + 1, &file[start..end])
}

/// Get the line and two serrounding lines that contains a char. returns ``(center_line_num: usize, lines: [&str; 3])``
pub fn get_lines(file: &str, idx: usize) -> (usize, [&str; 3]) {
  let mut start = find_line_start(file, idx);
  let end = find_line_end(file, idx);

  let line_2 = &file[start..end];
  let line_1;
  
  if start >= 1 {
    let idx = start - 1;
    start = find_line_start(file, idx);
    let end = find_line_end(file, idx);

    line_1 = &file[start..end];
  } else {
    line_1 = "";
  }

  let line_0;

  if start >= 1 {
    let idx = start - 1;
    let start = find_line_start(file, idx);
    let end = find_line_end(file, idx);

    line_0 = &file[start..end];
  } else {
    line_0 = "";
  }

  (get_line_num(file, idx) + 1, [line_0, line_1, line_2])
}

/// Add a number of spaces to the current line.
pub fn add_space(n: usize) {
  for _ in 0..n {
    print!(" ");
  }
}

/// Get the length in digits of a usize.
pub fn usize_log10(mut i: usize) -> usize {
  let mut len = 0;
  let zero = 0usize;
  let ten = 10usize;

  while i > zero {
    i /= ten;
    len += 1;
  }

  len
}
