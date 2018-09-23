extern crate permutate;
use permutate::{OneSized, Permutator};

fn main() {
    println!("examples and use-cases:\n");

    //
    let single_a_b_list: &[&str] = &["A", "B"];
    let single_false_list: &[&bool] = &[&false];
    let multi_str_list = [
        &["0", "1"][..],
        &["A", "B"][..],
        &["a", "b", "c"][..],
        &["_"][..],
    ];
    //
    let single_tuple: &(&[&str],) = &(&["A", "B", "C"],);
    let double_tuple: &(&[&str], &[&i32]) = &(&["A", "B", "C"], &[&0, &1, &2]);
    let triple_tuple: &(&[&str], &[&i32], &[&bool]) =
        &(&["A", "B"], &[&0, &1, &2], &[&false, &true]);

    println!("\nfixed 1-sized wrapper of a str list");
    {
        let list = single_a_b_list.clone();
        let list = [list; 1];
        let permutator = Permutator::new(&list);
        // println!("{:#?}", &permutator);
        for v in permutator {
            println!("{:?}", &v);
        }
        println!("(this is a specialization) for the 1-sized case");
    }

    println!("\nsame, but coincidentaly 1-sized wrapper of a str list");
    {
        println!("this doesn't uses the 1-sized specialization");
        let list = single_a_b_list.clone();
        let list = &[list];
        let permutator = Permutator::new(&list[..]);
        // println!("{:#?}", &permutator);
        for v in permutator {
            println!("{:?}", &v);
        }

        println!(
            "\n(Permutator types annotations may be used to verify correct specialization usage)"
        );
        let permutator: Permutator<OneSized<_>, _> = Permutator::new(&list);
        // println!("{:#?}", &permutator);
        for v in permutator {
            println!("{:?}", &v);
        }
    }

    println!("\n1-sized case for other types (e.g. booleans)");
    {
        let list = single_false_list.clone();
        let list = [list; 1];
        let permutator = Permutator::new(&list[..]);
        // println!("{:#?}", &permutator);
        for v in permutator {
            println!("{:?}", &v);
        }
    }

    println!("\nmulti-line wrapper of str lists");
    {
        let lists = multi_str_list.clone();
        let permutator = Permutator::new(&lists[..]);
        // println!("{:#?}", &permutator);
        for v in permutator {
            println!("{:?}", &v);
        }
    }

    println!("\nsame, but in this case type annotations were required (for deref)");
    {
        let lists = multi_str_list.clone();
        let permutator: Permutator<[&[&str]], _> = Permutator::new(&&&&&lists[..]);
        // println!("{:#?}", &permutator);
        for v in permutator {
            println!("{:?}", &v);
        }
    }

    println!("\nsingle tuple case");
    {
        let tuples = single_tuple.clone();
        let permutator = Permutator::new(&tuples);
        // println!("{:#?}", &permutator);
        for v in permutator {
            println!("{:?}", &v);
        }
    }

    println!("\ndouble tuple case");
    {
        let tuples = double_tuple.clone();
        let permutator = Permutator::new(&tuples);
        // println!("{:#?}", &permutator);
        for v in permutator {
            println!("{:?}", &v);
        }
    }

    println!("\ntriple tuple case");
    {
        let tuples = triple_tuple.clone();
        let permutator = Permutator::new(&tuples);
        // println!("{:#?}", &permutator);
        for v in permutator {
            println!("{:?}", &v);
        }
    }

    {
        // empty list (panics on debug + creation)
        // 	let list: &[&bool] = &[];
        //     let list = [list; 1];
        //     let permutator  = Permutator::new(&list);
        //     // println!("{:#?}", &permutator);
        //     for v in permutator {
        //         println!("{:?}", &v);
        //     }
    }

    {
        // to use the option specialization (WIP)
        //    let list: &[&bool] = &[];
        //    let list = [list; 1];
        //    let permutator: Permutator<_, Vec<Option<_>>>  = Permutator::new(&list);
        //    // println!("{:#?}", &permutator);
        //    for v in permutator {
        //        println!("{:?}", &v);
        //    }
    }
}
