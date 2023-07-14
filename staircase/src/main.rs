fn main() {
    let size : i32 = 9;
    create_staircase(size);
}

fn create_staircase(n: i32){
    for i in 1..n+1 {
        let mut step = String::new();
        
        for _ in 0..n-i {
            step.push(' ');
        }
        for _ in 0..i {
            step.push('#');
        }
        println!("{}", step);
    }
}
