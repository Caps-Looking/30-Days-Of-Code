

    let mut reader = String::new();

    io::stdin().read_line(&mut reader)
        .expect("Failed to read line!");

    let input_integer: i32 = reader.trim().parse()
        .expect("Please, enter a number!");

    reader = String::new();
    io::stdin().read_line(&mut reader)
        .expect("Failed to read line!");

    let input_double: f64 = reader.trim().parse()
        .expect("Please, enter a valid number!");;

    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string)
        .expect("Failed to read line!");

    println!("{}", i + input_integer);
    println!("{}", d + input_double);
    println!("{} {}", s, input_string);
