// test on an empty list
//
// {
// empty list (panics on debug + creation)
// 	let list: &[&bool] = &[];
//     let list = [list; 1];
//     let permutator  = Permutator::new(&list);
//     // println!("{:#?}", &permutator);
//     for v in permutator {
//         println!("{:?}", &v);
//     }
// }

// use option specialization
//
// {
// to use the option specialization (WIP)
//    let list: &[&bool] = &[];
//    let list = [list; 1];
//    let permutator: Permutator<_, Vec<Option<_>>>  = Permutator::new(&list);
//    // println!("{:#?}", &permutator);
//    for v in permutator {
//        println!("{:?}", &v);
//    }
// }
