use std::convert::{From,Into,TryFrom,TryInto};

#[derive(Debug)]
pub struct Number{
    value:i32,
}

impl From<i32> for Number{
     fn from(item: i32) -> Self{
        Number { value: item }
    }
}

impl Into<i32> for Number{
     fn into(self) -> i32{
        self.value
    }
}



#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber{
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error>{
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}
