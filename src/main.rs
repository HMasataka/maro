#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

macro_rules! tuple {
    ($n: expr, $l: expr) => {
        ($n, $l)
    };
}

fn main() {
    let v = vec![1, 2];
    let t = tuple!(1, 2);
    println!("{:?}", v);
    println!("{:?}", t);
}
