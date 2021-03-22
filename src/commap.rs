#[derive(Debug)]
pub enum Food{
    Apple,
    Carrot,
    Potato,
}
#[derive(Debug)]
pub enum Meat{
    Pork,
    Sheep,
    Chick,
}
#[derive(Debug)]
pub struct Peeled(Food,Meat);
#[derive(Debug)]
pub struct Chopped(Food,Meat);
#[derive(Debug)]
pub struct Cooked(Food,Meat);

pub fn peel(food : Option<Food>,meat :Option<Meat>)->Option<Peeled>{
    match food{
        Some(food)=> {
            match meat{
                Some(meat)=>Some(Peeled(food,meat)),
                None => None,
            }
        }
        None => None,
    }
}

pub fn peel2(food : Option<Food>,meat :Option<Meat>)->Option<Peeled>{
    Some(Peeled(food?,meat?))
}

pub fn chop(food : Option<Peeled>)->Option<Chopped>{
    match food{
        Some(Peeled(food,meat))=>Some(Chopped(food,meat)),
        None => None,
    }
}
pub fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
    chopped.map(|Chopped(food,meat)| Cooked(food,meat))
}
