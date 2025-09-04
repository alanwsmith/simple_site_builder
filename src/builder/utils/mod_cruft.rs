// pub fn copy_files(&self) -> Result<()> {
//   copy_file_list(get_files(&self.config.content_root))
//     .iter()
//     .for_each(|input_file| {
//       let details = CopyFileDetails::new(
//         &self.config.content_root,
//         input_file,
//         &self.config.output_root,
//       );
//       let _ = copy_file_with_mkdir(
//         &details.input_path(),
//         &details.output_path(),
//       );
//     });
//   Ok(())
// }

// pub fn load_data(&self) -> Value {
//   let mut data_map = BTreeMap::new();
//   json_file_list(get_files(&self.config.content_root))
//     .iter()
//     .for_each(|input_file| {
//       let input_path =
//         self.config.content_root.join(input_file);
//       match fs::read_to_string(&input_path) {
//         Ok(json) => {
//           match serde_json::from_str::<Value>(&json) {
//             Ok(data) => {
//               data_map.insert(
//                 input_file.display().to_string(),
//                 data,
//               );
//             }
//             Err(e) => {
//               // TODO: Add better error handling here
//               dbg!(e);
//             }
//           }
//         }
//         Err(e) => {
//           // TODO: Add better error messaging here
//           dbg!(e);
//         }
//       }
//     });
//   Value::from_serialize(data_map)
// }

// pub fn load_highlighted_files(&self) -> Value {
//   let mut files = BTreeMap::new();
//   highlighted_file_list(get_files(
//     &self.config.content_root,
//   ))
//   .iter()
//   .for_each(|input_file| {
//     let input_path =
//       self.config.content_root.join(input_file);
//     // Reminder: files are already filter to make sure
//     // they have a valid extension so you
//     // can just unwrap:
//     let ext = input_path.extension().unwrap();
//     match fs::read_to_string(&input_path) {
//       Ok(code) => {
//         let highlighted = highlight_code(
//           &code,
//           &ext.display().to_string(),
//         );
//         files.insert(
//           input_file.display().to_string(),
//           highlighted,
//         );
//       }
//       Err(e) => {
//         dbg!(e);
//       }
//     }
//   });
//   Value::from_serialize(files)
// }

// file_list(&self.config.content_root).iter().for_each(
//   |file_detail| {
//     dbg!("asdf");
//   },
// );
// let data = self.load_data();
// let highlighted = self.load_highlighted_files();
// html_file_list(get_files(&self.config.content_root))
//   .iter()
//   .for_each(|input_path| {
//     let details = HtmlFileDetails::new(
//       input_path,
//       &self.config.output_root.clone(),
//     );
//     match env.get_template(&details.input_path_str())
//     {
//       Ok(template) => {
//         match template.render(context!(
//           // data => data,
//           // highlighted => highlighted,
//         )) {
//           Ok(output) => {
//             let _ = write_file_with_mkdir(
//               &details.output_path(),
//               &output,
//             );
//           }
//           Err(e) => {
//             // TODO: Throw here and print error
//             dbg!(e);
//           }
//         }
//       }
//       Err(e) => {
//         // TODO: Throw here and print error
//         dbg!(e);
//       }
//     }
//   });
