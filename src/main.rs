mod bubble;

fn main() {
    let mut arr = [ 1, 3, 2, 5, 1 ];
    bubble::bubble_sort(&mut arr);
    println!("{:?}", arr);
}
