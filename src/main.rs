use chrono::prelude::*;
use std::collections::HashMap;

fn main() {
let name = "Jack";
let a = 12;
let b = 16;
let a = a * 2 + 10;
let x = {
    let a = 100;
    a + 11
};
println!("a is {0}\nb is {1}\nx is {2}", a, b, x);

let mut s1 = String::from("yahoo");
let len = check_length(&s1);
// let s2 = &s1; 错误,禁止修改租借的值
let s2 = &mut s1;
s2.push_str("!!!");
println!("{}",s2);
println!("{} length is {}", s1, len);

let array1 = [1, 3, 5, 7, 9];
let array2 = &array1[0..3];
for i in array2 {
    println!("{}",i);
}
// arr 等同于 let arr = [3, 3, 3, 3, 3]
let arr = [3; 5];
println!("Hello, {}!", name);
println!("arr -> {:?}", arr);
another_function(100, 200);
println!("five的值为: {}",five());
println!("add(): {}",add(1, 2));
let number = if a > 0 { 1 } else { -1 };
println!("number为 {}", number);

// while循环
// let mut num = 1;
// while num != 4 {
//     println!("num = {}",num);
//     num += 1;
// }
// println!("EXIT");
// for循环
// let array = [10, 20, 30];
// for i in array.iter(){
//     println!("array-{}", i);
// }
// println!("EXIT");
// let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
// for i in 0..6 {
//     println!("arr-{}", arr[i]);
// }
// loop循环
// let s = ['B', 'A', 'I', 'D', 'U'];
// let mut i = 0;
// loop {
//     let ch = s[i];
//     if ch == 'U' {
//         break;
//     }
//     println!("{}", ch);
//     i += 1;
// }

// 元组结构体
struct Color(u8, u8, u8);
struct Point(f64, f64);

let black = Color(0, 0, 0);
let origin = Point(0.0, 0.0);

println!("{},{},{}",black.0, black.1, black.2);
println!("{},{}", origin.0, origin.1);

let rect1 = Rectangle { width: 10, height: 20 };
let rect2: Rectangle = Rectangle { width: 30, height: 20 };
println!("rect1's area is {}",rect1.area());
println!("{}",rect1.wider(&rect2));

let my_dog = Dog {
    name: String::from("coco"),
    age: 3
};
let my_dog_name_clone = my_dog.name.clone();
let my_dog_age_clone = my_dog.age.clone();
println!("{}",my_dog_name_clone);
println!("{}",my_dog_age_clone);
println!("{:?}",my_dog);
let book = Book::Papery(1001);
let ebook = Book::Electronic(String::from("url://..."));
println!("{:?}", book);
println!("{:?}", ebook);

#[derive(Debug)]
enum Book2 {
    #[allow(dead_code)]
    Papery { 
        name: String,
        price: u32
     },
    #[allow(dead_code)]
    Electronic {
        name: String,
        url: String
     }
}
let p_book2 = Book2::Papery { name: String::from("《Romance of the Three Kingdoms》"), price: 100 };
let e_book2 = Book2::Electronic { name: String::from("《All Men Are Brothers》"), url: String::from("url://...") };
println!("{:?}",p_book2);
println!("{:?}",e_book2);

enum Option<T> {
    Some(T),
    #[allow(dead_code)]
    None
}
let opt: Option<&str> = Option::Some("Hello");
match opt {
    Option::Some(something) => {
        println!("{}",something);
    }
    Option::None => {
        println!("opt is nothing")
    }
}
let t= Some(64);
match t {
    Some(64) => println!("Yes"),
    _ => println!("No")
}

// if let 语法
let i = 0;
match i {
    0 => println!("zero"),
    _ => {}
}
let i = 0;
if let 1 = i {
    println!("zero")
}
let _bk = Book::Electronic(String::from("url..."));

eat_at_restaurant();

// 向量
let mut vector = vec![1, 2, 4, 8]; // 通过数组创建向量
vector.push(16);
vector.push(32);

for i in vector { println!("{}",i) }

// 字符串
let one = 1.to_string();
let float = 1.0.to_string();
let slice = "slice".to_string();
println!("{0},{1},{2}",one,float,slice);
let mut bai = String::from("bai");
bai.push_str("du");

let goo = String::from("goo");
let gle = String::from("gle");
let google = goo + &gle;

println!("{},{}",bai,google);
println!("gle字符数量为->{},gle单个字符为->{:?}",gle.chars().count(),gle.chars().nth(1));

// 映射表
let mut map = HashMap::new();
map.insert("hello", "world");
map.insert("good", "bye");
if let Some(x) = map.get_mut(&"good") {
    *x = "night"; 
}
println!("{}",map.get("hello").unwrap());
for i in map.iter() {
    println!("{:?}",i);
}

// let mut a: Vec<&'static str> = Vec::new();
// a.push("a");
// a.push("b");

// let hour = 3600;
// let dt = FixedOffset::east_opt(5 * hour).unwrap().with_ymd_and_hms(2000,0,0,0,0,0).unwrap();
let fmt = "%Y-%m-%d %H:%M:%S";
let now: DateTime<Utc> = Utc::now();
let ts = now.timestamp_millis();
println!("{0},{1}",now.format(fmt),ts);

let e1 = Employee::new(1, String::from("Lisa"), String::from("后端开发工程师"), 1, now, 30000.00, 300.00, 1);
let e2 = Employee::new(2, String::from("Mark"), String::from("前端开发工程师"), 2, now, 20000.00, 100.00, 2);
let e3 = Employee::new(3, String::from("Jack"), String::from("UI设计工程师"), 3, now, 25000.00, 200.00, 3);


let dep1 = Department::new(1, String::from("开发部"), String::from("北京"), vec![e1, e2]);
let dep2 = Department::new(2, String::from("UI部"), String::from("上海"), vec![e3]);
println!("{:#?}",dep1);
println!("{:#?}",dep2);

}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn wider(&self, rect: &Rectangle) -> bool {
        self.width > rect.width
    }
}
#[derive(Debug)]
struct Dog {
    name: String,
    age: i8,
}
#[derive(Debug)]
enum Book {
    Papery(u32),
    Electronic(String)
}
fn another_function(x: i32, y: i32){
    println!("x 的值为 : {}", x);
    println!("y 的值为 : {}", y);
}
fn five() -> i32 {
    5
}
fn add(x: i32, y: i32) -> i32 {
    return x + y;
}
fn check_length(s: &String) -> usize{
    s.len()
}
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        #[allow(dead_code)]
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast { 
                toast: String::from(toast), seasonal_fruit: String::from("peaches") 
            }
        }
    }
}
pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please",meal.toast);
}
// 面向对象
#[derive(Debug)]
pub struct Department {
  pub id: i32,
  pub name: String,
  pub location: String,
  pub employee: Vec<Employee>,
}

impl Department {
  pub fn new(id: i32, name: String, location: String, employee: Vec<Employee>) -> Department {
    Department {
      id: id,
      name: name,
      location: location,
      employee: employee
    }
  }
  #[allow(dead_code)]
  pub fn pub_method(&self) {
      println!("from public method");
  }
  #[allow(dead_code)]
  fn private_method(&self) {
      println!("from private method");
  }
}

#[derive(Debug)]
pub struct Employee {
  pub id: i32,
  pub name: String,
  pub job: String,
  pub manager_id: i32,
  pub hire_date: DateTime<Utc>,
  pub salary: f64,
  pub commission: f64,
  pub department_id: i32,
}

impl Employee {
    pub fn new (id: i32, name: String, job: String, manager_id: i32, hire_date: DateTime<Utc>, salary: f64, commission: f64, department_id: i32) -> Employee {
        Employee {
            id: id,
            name: name,
            job: job,
            manager_id: manager_id,
            hire_date: hire_date,
            salary: salary,
            commission: commission,
            department_id: department_id
        }
    }
}