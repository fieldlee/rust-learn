#[warn(dead_code)]
pub enum AddSubstrct{
    Add,
    SubStruct,
}

pub type Operate = AddSubstrct;

impl Operate{
    pub fn run(&self,x:i32,y:i32)->i32{
        match self {
            Operate::Add =>x+y,
            Operate::SubStruct=>x-y,
        }
    }
}