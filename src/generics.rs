//構造体では型を複数定義する場合下のようにする。
struct Point<T, U>{
    x: T,
    y: U
}
//また構造体にはメソッドが下のように定義できる
impl <T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W>{
        Point { 
            x: self.x,
            y: other.y 
        }
    }
}

pub fn run(){
    let number_list = vec![1,2,3,4];
    // println!("{}", largest_i32(number_list));

    let char_list = vec!['a','b','c','d'];
    println!("{}", largest(char_list));
    println!("{}", largest(number_list));

    let p1 = Point {x: "rust", y: 'a'};
    let p2 = Point { x: 1, y: 2.5 };
    let p3 = p1.mixup(p2);
    println!("{} {}", p3.x, p3.y);

}

// fn largest_i32(list: Vec<i32>) -> i32 {
//     let mut largest = list[0];
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

//Tを指定することでいろんな型に対してもこの関数を適応できるようにする
fn largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    //Tだけだと比較できない型もあるのでエラーが出る
    //Partial~をつけると比較できる型だけ適応される
    let mut largest = list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}