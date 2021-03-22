use std::num::ParseIntError;


pub fn multiply(first_num_str:&str, second_num_str:&str) -> Result<usize,ParseIntError>{
    first_num_str.parse::<usize>().and_then(|first_num|{
        second_num_str.parse::<usize>().map(|second_num| first_num*second_num)
    })
}

pub fn multiply2(first_num_str:&str,second_num_str:&str) -> Result<usize,ParseIntError>{
    let first = first_num_str.parse::<usize>()?;
    let second = second_num_str.parse::<usize>()?;
    Ok(first*second)
}

// pub fn multiply3(first_num_str:&str,second_num_str:&str) -> Result<usize,ParseIntError>{
//     let first = try!(first_num_str.parse::<usize>());
//     let second = try!(second_num_str.parse::<usize>());
//     Ok(first*second)
// }