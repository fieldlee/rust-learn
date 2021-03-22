pub enum WebEvent {
    PageLoad,
    PagePreLoad,
    KeyPress(char),
    Paste(String),
    Click(i32,i32),
}

pub fn inspect(e:WebEvent){
    match e{
        WebEvent::PageLoad=> println!("page load"),
        WebEvent::PagePreLoad => println!("page pre load"),
        WebEvent::KeyPress(c) => println!("your press {}",c),
        WebEvent::Paste(v) => println!("your paste {}",v),
        WebEvent::Click(x,y) => println!("your click x {},y {}",x,y),
    }
}