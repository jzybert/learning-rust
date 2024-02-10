fn main() {
    let y = {
        let x = 5;
        x
    };

    println!("{y}");

    let x = plus_one(five());
    println!("Six: {x}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
