fn main() {
    let candles: [i32; 4] = [ 3, 2, 1, 3 ];
    let sum = sum_of_tallest(&candles);
    println!("{sum}");
}

fn sum_of_tallest(candles: &[i32]) -> i32 {
    let (mut max, mut total) : (i32, i32) = (0, 0);

    for i in 0..candles.len(){
        if candles[i] > max {
            max = candles[i];
        }
    }

    for i in 0..candles.len(){
        if max == candles[i] {
            total += 1;  
        }
    }
    
    total
}
