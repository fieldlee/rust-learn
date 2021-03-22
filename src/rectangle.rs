pub struct Point {
    x:u32,
    y:u32,
}
impl Point{
    fn origin()->Self{
        Point{
            x:0,
            y:0,
        }
    }
    fn new(x:u32,y:u32)->Self{
        Point{x,y}
    }
}

pub struct Rectange{
    p1:Point,
    p2:Point,
}

impl Rectange{
    fn area(&self) -> u32{
        ((self.p1.x-self.p2.x)*(self.p1.y-self.p2.y)).abs()
    }
    fn perimeter(&self) -> u32{
        2*((self.p1.x-self.p2.x).abs()+(self.p1.y-self.p2.y).abs())
    }
    fn translate(&mut self, x: u32, y: u32) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // 这个方法会 “消耗” 调用者的资源
    // `self` 为 `self: Self` 的语法糖
    fn destroy(self) {
        // 解构 `self`
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` 和 `second` 离开作用域后释放
    }
}
