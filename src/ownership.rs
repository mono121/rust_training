pub fn run() {
    let s1 = String::from("hello");
    let s2 = s1;//所有権のmove
    println!("{} ", s2);

    let s3 = String::from("Hello");
    let s4 = s3.clone();//deep copy ヒープ内のデータがコピーされる。ただしヒープ領域圧迫
    println!("{} {}", s3, s4);

    let s5 = String::from("Hello");
    take_ownership(s5);
    // println!("{}", s5);//所有権が関数の引数にmoveしているのでエラー

    let s6 = String::from("hello");
    //所有権が最初はs6にあるが、関数に渡されreturnで最終的にs7に渡される。
    let _s7 = take_giveback_ownership(s6);

    let mut _s10 = String::from("hello");
    // let r1 = &s10;
    //一度参照されると,その値を変更されたくないのでmutで参照できなくなる。
    // let r2 = &mut s10;
    // println!()
    
}
fn take_ownership(s: String){
    println!("{}", s);
}

//&をつけることで参照できる。所有権を気にしないでコピーできる
fn _calc_len(s: &String) -> usize {
    s.len()
}

fn take_giveback_ownership(s: String) -> String{
    s//rustでは関数の最後の行で;がな行がreturn文となる。
}