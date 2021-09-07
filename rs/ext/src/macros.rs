// #[macro_export]
// macro_rules! register{
//     ({$($para: ident: $data:expr),*  }) => {

//         // repeat
//         pub mod plate; use plate::Plate;
//         pub mod kick; use kick::*;

//         pub fn make_node_ext<const N: usize>(
//             name: &str,
//             paras: &mut Pairs<Rule>,
//             pos: (usize, usize),
//             samples_dict: &HashMap<String, &'static[f32]>,
//             sr: usize,
//             bpm: f32,
//         ) -> Option<GlicolNodeData<N>> {
//             let n = match name {

//                 // repeat
//                 "plate" => 1,
//                 "kick" => 1,

//                 _ => return None
//             };

//             let mut pv = vec![];

//             for i in 0..n {
//                 let mut p = match paras.next() {
//                     Some(v) => v.as_str(),
//                     None => return None
//                 };
//                 // no modulation here so far
//                 match p.parse::<f32>() {
//                     Ok(v) => pv.push(v),
//                     Err(_) => return None
//                 };
//             };
            
//             let node = match name {
                
//                 // only one para is supported
//                 "plate" => plate!(N => pv[0]), 
//                 "kick" => kick!(N => pv[0]),
//                 _ => unimplemented!()
//             };
            
//             Some(node)
//         }
//    }
// }