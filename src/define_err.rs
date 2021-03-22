use std::fmt;
type Result<T> = std::result::Result<T,T>;

#[derive(Copy, Clone)]
pub struct DoubleErr;

impl fmt::Display for DoubleErr{
    fn fmt(&self,f:&mut fmt::Formatter)-> fmt::Result{
        write!(f,"invalid first item to double")
    }
}
pub fn double_first(vec:Vec<&str>) -> std::result::Result<i32, DoubleErr> {
    vec.first().ok_or(DoubleErr).and_then(|s|{
        s.parse::<i32>().map_err(|_|DoubleErr).map(|i|i*2)
    })
}