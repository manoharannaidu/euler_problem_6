fn sum_n_squares(n:f32) -> f32 {
    n * (n+1.0) * ((2.0*n)+1.0) * (1.0/6.0)
}

fn square_sum_n(n:f32) -> f32 {
    (n * (n+1.0) * (1.0/2.0)).powf(2.0)
}

fn main() {
    let result =  square_sum_n(100.0) - sum_n_squares(100.0);
    print!("{}",result);
}
