fn print_padovan() {
    let mut padovan = vec![1, 1, 1];
    for i in 3..10 {
        println!(
            "p1 = {}, 
             p2 = {} , 
             p1 + p2 = {}",
            padovan[i - 3],
            padovan[i - 2],
            padovan[i - 3] + padovan[i - 2]
        );
        let next = padovan[i - 3] + padovan[i - 2];
        padovan.push(next);
    }
    println!("P(1..10)= {:?}", padovan);
}

fn main() {
    print_padovan();

    {
        let b = Box::new("some string".to_string());
        println!("{:?}", b);
    }

    let bytes_arr = [0b1001000, 0b1100101, 0b1101100, 0b1101100, 0b1101111];
    let decimal_value: Vec<u8> = bytes_arr.iter().map(|byte| *byte as u8).collect();
    let string_val = String::from_utf8(decimal_value);
    match string_val {
        Ok(val) => println!("{val}"),
        Err(e) => eprintln!("{e}"),
    }

    // move and indexed content

    let mut v = Vec::new();

    for i in 101..106 {
        v.push(i.to_string());
    }

    // 1. Pop a value off the end of the vector:
    let fifth = v.pop().expect("vector empty!");
    assert_eq!(fifth, "105");

    //2 move a value out of a given index in the vector,
    //and move the last element into its spot:
    let second = v.swap_remove(1);
    assert_eq!(second, "102");

    // swap in another value for the one we're taking out:
    let third = std::mem::replace(&mut v[2], "subtitute".to_string());
    assert_eq!(third, "103");

    assert_eq!(v, vec!["101", "104", "subtitute"])
}
