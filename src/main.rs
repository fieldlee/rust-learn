use std::fs::File;
use std::io::prelude::*;
use std::str;
mod add_substrct;
mod link_list;
mod web_event;
mod from_into;

use from_into::Number;

fn main() {
    let mut fs = File::create("test.txt").expect("create error");
    println!("{:?}", fs);
    let fs_res = fs.write_all(b"Hello, world!");
    println!("{:?}", fs_res);

    let fs_read = File::open("test.txt").unwrap();

    let mut vec_buf: Vec<u8> = vec![];
    for byte in fs_read.bytes() {
        match byte {
            Ok(b) => {
                vec_buf.push(b);
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }

    match str::from_utf8(vec_buf.as_slice()) {
        Ok(v) => {
            println!("{}", v)
        }
        Err(e) => {
            println!("{:?}", e)
        }
    }
    let pressed = web_event::WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted = web_event::WebEvent::Paste("my text".to_owned());
    let click = web_event::WebEvent::Click(20, 30);
    let load = web_event::WebEvent::PageLoad;
    let unload = web_event::WebEvent::PagePreLoad;
    web_event::inspect(pressed);
    web_event::inspect(pasted);
    web_event::inspect(click);
    web_event::inspect(load);
    web_event::inspect(unload);

    let x = add_substrct::Operate::Add;
    let y = add_substrct::Operate::SubStruct;
    let op_add_res = x.run(2, 3);
    let op_sub_res = y.run(2, 3);
    println!("{}", op_add_res);
    println!("{}", op_sub_res);

    //==============link_list
    let mut list = link_list::List::new();

    // Prepend some elements
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // Show the final state of the list
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());

    //==================from_into

    let num = Number::from(30);
    println!("{:?}", num);
    let f:i32 = num.into();
    println!("int {}", f);

    //==================from_string  To_String
   let v = "10".parse::<i32>().unwrap();
   println!("v {}", v);

   let n = 5;
   let big_n = if n<10 && n>-10 {
       10*n
   }else{
       n/2
   };
   println!("big_n {}", big_n);

   let mut counter = 0;
   let another_counter = loop{
        counter += 1;
        if counter == 10 {
            break counter;
        }
   };
   println!("another_counter {:?}", another_counter);

   let pair = (0, -2);
    // 试一试 ^ 将不同的值赋给 `pair`
    println!("Tell me about {:?}", pair);
    // match 可以解构一个元组
    match pair {
        // 解构出第二个值
        (0, y) => println!("First is `0` and `y` is `{:?}`", y),
        (x, 0) => println!("`x` is `{:?}` and last is `0`", x),
        _      => println!("It doesn't matter what they are"),
        // `_` 表示不将值绑定到变量
    }

    let mut mut_value = 6;
    match mut_value {
        mut m => {
            m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        },
    }

    match mut_value {
        0             => println!("I'm not born yet I guess"),
        // 可以直接 `match` 1 ..= 12，但怎么把岁数打印出来呢？
        // 相反，在 1 ..= 12 分支中绑定匹配值到 `n` 。现在年龄就可以读取了。
        n @ 1  ..= 12 => println!("I'm a child of age {:?}", n),
        n @ 13 ..= 19 => println!("I'm a teen of age {:?}", n),
        // 不符合上面的范围。返回结果。
        n             => println!("I'm an old person of age {:?}", n),
    }

    let age = Some(6);

    match age {
        // Got `Some` variant, match if its value, bound to `n`,
        // is equal to 42.
        Some(n @ 6) => println!("The Answer: {}!", n),
        // Match any other number.
        Some(n)      => println!("Not interesting... {}", n),
        // Match anything else (`None` variant).
        _            => (),
    }



    // 将 `optional` 设为 `Option<i32>` 类型
    let mut optional = Some(8);

    // 这读作：当 `let` 将 `optional` 解构成 `Some(i)` 时，就
    // 执行语句块（`{}`）。否则就 `break`。
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
        // ^ 使用的缩进更少，并且不用显式地处理失败情况。
    }

    //==============move function
    let hystack = vec![1,2,3];
    let contains = move |needle| hystack.contains(needle);
    println!("{}", contains(&1));
    println!("{}", contains(&4));
    // println!("{:?}", hystack);
    // ^ 取消上面一行的注释将导致编译时错误，因为借用检查不允许在变量被移动走
    // 之后继续使用它。
    
    // 在闭包的签名中删除 `move` 会导致闭包以不可变方式借用 `haystack`，因此之后
    // `haystack` 仍然可用，取消上面的注释也不会导致错误。
    let maxer = 10000;
    let sum_of_squared_odd_numbers = (0..).map(|n|n*n)
                                    .take_while(|&n|n<maxer)
                                    .filter(|&n|is_odd(n))
                                    .fold(0,|sum,i|sum+i);
    println!("sum_of_squared_odd_numbers {}",sum_of_squared_odd_numbers);

}
fn is_odd(n:i32)-> bool {
    n % 2 == 1
}