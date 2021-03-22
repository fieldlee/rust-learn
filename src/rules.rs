

macro_rules! say_hello {
    ($v:ident) => {
        println!("hello world in {} rules!",$v);
    };
}

macro_rules! function_create {
    ($func_name:ident) => {
        fn $func_name(){
            println!("You called {:?}()",
                     stringify!($func_name));
        }
    };
}

macro_rules!  print_result {
    ($exp:expr) => {
        println!("{} = {}",stringify!($exp),$exp);
    };
}

macro_rules! find_min {
    ($x : expr) => ($x);
    ($x : expr,$($y:expr),+) => {
        std::cmp::min($x, find_min!($($y),+))
    };
}