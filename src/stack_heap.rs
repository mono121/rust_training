enum List {
    //Box pointer：stackに格納されているデータをヒープ領域に移して、その先頭のポインタをstack領域に格納する。
    //そのポインタを格納しているアドレスをBox pointerと呼ぶ。
    Node(i32, Box<List>),
    //rustではコンパイル時にenumの取り得る最大のメモリを確保するが上のBoxがないとListが再帰的になり
    //取り得るメモリが無限になってしまう。なのでBox化し、第二引数を上限が決まっているポインタ型にする。
    Nil
}


pub fn run(){
    //配列は8MB
    // let a1: [u8; 7000000] = [1; 7000000];//7MB
    // let a1: [u8; 9000000] = [1; 9000000];//9MB stack overflow

    //vector
    let mut v1 = vec![1,2,3,4];
    let mut v3 = vec![9,10];//stringと同じで24bit。しかしキャパシティの値はバイト数ではなく要素数。
    println!("{:?}", v1);
    v1.insert(1, 10);
    println!("{:?}", v1);
    v1.remove(0);
    println!("{:?}", v1);
    v1.append(&mut v3);
    println!("{:?}", v1);
    println!("{:?}", v3);//[]
}