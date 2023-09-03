#[macro_export]
macro_rules! rev {
    ( $( $x:expr )*; $y:expr ) => {
        {
            let mut sum_temp = String::new();
            $(
                sum_temp = sum_temp + &$x.to_string();
            )*
            println!("Print from macro: {:?}", $y);
            sum_temp
        }
    };
}

fn main() {
    let x = 30.0;
    let y = 15.0;
    println!("{}", rev!(x y 22 44 "Hello"; Some(1)));
    // println!("{:?}", vec!(0, 1, 2));
}
