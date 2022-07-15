pub fn run() {
    let s1 = String::from("hello");
    let s2 = s1;//所有権のmove
    println!("{} ", s2);

    let s3 = String::from("Hello");
    let s4 = s3.clone();//deep copy ヒープ内のデータがコピーされる。ただしヒープ領域圧迫

}