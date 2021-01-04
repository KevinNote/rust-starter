fn main() {
    println!("[i32 Range 1]");
    let i32_range = [1, 5];
    for item in i32_range.iter() {
        print!("{} ", item);
    }

    println!("\n[i32 Range 2]");
    let i32_range_2: [i32; 5] = [1; 5];
    for item in i32_range_2.iter() {
        print!("{} ", item);
    }

    println!("\n[print with traditional for]");
    for i in 1..5 {
        print!("{} ", i);
    }
}
