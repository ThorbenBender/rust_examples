fn main() {
    let vec: Vec<i32> = (0..=100).map(|num| num).collect();
    println!("{:#?}", vec);
}
