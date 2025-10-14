fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // Variables r1 and r2 will not be used after this point.
    let r4 = r2;
    let r5 = r4;
    println!("{r4}");
    println!("Original value: {s}");

    let r3 = &mut s; // no problem
    println!("{r3}");
    // string slicing
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_rewrite(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    let mut first_space = 0;
    let mut space_seen = false;
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if space_seen {
                return &s[first_space..i];
            } else {
                space_seen = true;
                first_space = i + 1;
            }
        }
    }
    &s[..]
}
