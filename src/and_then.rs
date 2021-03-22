#[derive(Debug)] pub enum Food { CordonBleu, Steak, Sushi }
#[derive(Debug)] pub enum Day { Monday, Tuesday, Wednesday }

pub fn have_recipte(food:Food)->Option<Food>{
    match food{
        Food::CordonBleu => None,
        _ => Some(food),
    }
}

pub fn have_ingredients(food:Food)->Option<Food>{
    match food {
        Food::Sushi => None,
        _           => Some(food),
    }
}

pub fn cookable_v1(food:Food)->Option<Food>{
    match have_recipte(food){
        None => None,
        Some(food)=>match have_ingredients(food){
            None => None,
            Some(food) => Some(food),
        }
    }
}

pub fn cookable_v2(food:Food)->Option<Food>{
    have_recipte(food).and_then(have_ingredients)
}

pub fn eat(food: Food, day: Day) {
    match cookable_v2(food) {
        Some(food) => println!("Yay! On {:?} we get to eat {:?}.", day, food),
        None       => println!("Oh no. We don't get to eat on {:?}?", day),
    }
}