use crate::and_then::eat;
use std::collections::HashMap;
#[macro_use]
pub mod rules;

fn main() {
    let test = "tset".to_string();
    say_hello!(test);
    foo();
    function_create!(foo);
    print_result!(1+1);
    print_result!({
        let x = 1u32;
        x * x + 2 * x - 1
    });
    println!("{}",find_min!(-1,2,3,4));
    person_area_code();
    com_map();
    and_then_eat();
    twoparse();
    hash_map();
    thread::thread_spawn();
    double_first();

}

fn foo(){
    println!("foo is called");
}

mod person;
fn person_area_code(){
    let person = person::Person{
        job:Some(person::Job{
           phone_number:Some(person:: PhoneNumber{
            area_code : None,
            number : 100,
           })
        })
    };
    match person.work_area_code(){
        Some(v)=>println!("work_area_code:{}",v),
        None=>println!("work_area_code:none"),
    };
}

mod commap;
mod and_then;
mod twoparse;
mod define_err;
mod hash_map;
mod thread;

fn com_map(){

    // let peel = commap::peel(Some(commap::Food::Apple),Some(commap::Meat::Chick));
    let peel = commap::peel2(Some(commap::Food::Apple),Some(commap::Meat::Chick));
    let chop = commap::chop(peel);
    let cok = commap::cook(chop);

    println!("{:?}",cok);
}

fn and_then_eat(){
    let (blue,steak,sushi) = (and_then::Food::CordonBleu,and_then::Food::Steak,and_then::Food::Sushi);
    eat(blue,and_then::Day::Monday);
    eat(steak,and_then::Day::Tuesday);
    eat(sushi,and_then::Day::Wednesday);
}

fn twoparse(){
    let result = twoparse::multiply2("3","4");
    let result2 = result.map(|re|re);
    println!("twoparse:{}",result2.expect("eror"));
}

fn double_first()  {
    let numbers = vec!["42", "93", "18"];
    let v = define_err::double_first(numbers).unwrap_err();
    println!("double_first:{}",v);
}

fn hash_map(){
    let mut contacts = HashMap::new();

    contacts.insert("Daniel", "798-1364");
    contacts.insert("Ashley", "645-7689");
    contacts.insert("Katie", "435-8291");
    contacts.insert("Robert", "956-1745");

    match contacts.get("Daniel") {
        Some(v)=>println!("Calling Daniel: {}", hash_map::call(v)),
        _ => println!("Don't have Daniel's number."),
    }

}