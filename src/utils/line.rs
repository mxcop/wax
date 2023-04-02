// /// Get the (row, col) for an index within a string.
// /// 
// /// Credit : TheBerkin (https://github.com/TheBerkin/line-col-rs)
// pub fn get_row_col(file: &str, index: usize) -> (usize, usize) {
//   if index > file.len() {
//     panic!("Index cannot be greater than the length of the input slice.");
//   }

//   let heads = heads(file);

//   // Perform a binary search to locate the line on which `index` resides
//   let mut line_range = 0..heads.len();
//   while line_range.end - line_range.start > 1 {
//     let range_middle = line_range.start + (line_range.end - line_range.start) / 2;
//     let (left, right) = (line_range.start..range_middle, range_middle..line_range.end);
//     // Check which line window contains our character index
//     if (heads[left.start]..heads[left.end]).contains(&index) {
//       line_range = left;
//     } else {
//       line_range = right;
//     }
//   }

//   let line_start_index = heads[line_range.start];
//   let line = line_range.start + 1;
//   let col = index - line_start_index + 1;

//   return (line, col);
// }

// /// Get the indicies of the start of each line within a string.
// /// 
// /// Credit : TheBerkin (https://github.com/TheBerkin/line-col-rs)
// fn heads(file: &str) -> Vec<usize> {
//   std::iter::once(0)
//     .chain(
//       file
//         .char_indices()
//         .filter_map(|(i, c)| Some(i + 1).filter(|_| c == '\n')),
//     )
//     .collect()
// }
