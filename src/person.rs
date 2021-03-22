pub struct Person{
    pub job : Option<Job>
}
#[derive(Copy,Clone)]
pub struct Job {
    pub  phone_number:Option<PhoneNumber>
}

#[derive(Copy,Clone)]
pub struct PhoneNumber{
    pub area_code : Option<u32>,
    pub  number : u32,
}

impl Person{
    pub fn work_area_code(&self)->Option<u32>{
        self.job?.phone_number?.area_code
    }
}
