// use std::fs;

// use blueprint::modules::parser::{
//   description::parse as description_parser
// };

// #[cfg(test)]
// mod test_parser {
//   use super::*;
//   #[test]
//   fn test_description() {
//     let file = fs::read_to_string("./examples/Button.svelte").expect("Unable to find file.svelte");
//     let desc = description_parser(&file);
//     assert_eq!(desc, Some("Componente de botón"))
//   }
// }