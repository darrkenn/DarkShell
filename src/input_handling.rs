pub fn handle_input(input: Vec<&str>) {

    match input.as_slice() {

        /////////////////////
        ////    Cat      ////
        /////////////////////
        ["cat", args @ ..] if !args.is_empty() => {
            for word in args {
                println!("{}", word);
            }
        }
        ["cat"] => {
            println!("provide text");
        }

        /////////////////////
        ////    Clear    ////
        /////////////////////



        _ => {
            println!("Invalid");
        }
    }
}