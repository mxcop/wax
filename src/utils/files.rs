// use std::{fs::{DirEntry, self}, path::Path, io};

// /// Recursively visit all files within a directory.
// pub fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
//   if dir.is_dir() {
//       for entry in fs::read_dir(dir)? {
//           let entry = entry?;
//           let path = entry.path();
//           if path.is_dir() {
//               visit_dirs(&path, cb)?;
//           }
//           else {
//             cb(&entry);
//           }
//       }
//   }
//   Ok(())
// }

// use std::path::Path;

// /// Load a file using relative or absolute path */
// pub fn load_file(path: &str) -> std::io::Result<String> {
//   std::fs::read_to_string(Path::new(path))
// }