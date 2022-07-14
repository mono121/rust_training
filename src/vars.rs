pub mod sub;

pub fn vars_run(){
     //pointer
     println!("Hello, world!");
     let x = 1;
     println!("pointer is {:p}", &x);
 
     //シャドーイング
     let y = 5;
     let y = y + 1;
     println!("y is {}", y);
     let y = y * 2;
     println!("y is {}", y);
 
     //タプル
     let t1 = (1,1.5,"dummy");
     let (_x, _y, _z) = t1;
     println!("t1 is {} {} {}", t1.0, t1.1, t1.2);
     let mut t2 = ((0, 1), (2, 3));
     let ((ref mut x1_ptr, _), _) = t2;
     //pointerを指定してアドレスの中身を書き換える
     *x1_ptr = 5;
     println!("{:?}", t2);
 
     //配列
     let a1 = [0; 10];
     println!("{:?}", a1);
 
     //文字列スライス&str
     let s1 = "helloこんにちわ";//20byte
     let s2 = "hello";
     println!("stack address of s1 is {:p}", &s1);//16byteのデータが割り当てられる、スタックには保存されていない。
     println!("stack address of s2 is {:p}", &s2);//最初の8byteは実データ、あとはバイト数
     println!("static memory address of s1 is {:?}", &s1.as_ptr());//最初の8byteの中の静的領域のアドレスの値をとってくる
     println!("static memory address of s2 is {:?}", &s2.as_ptr());
     println!("static memory address of s1 is {:?}", &s1.len());//後の8byteのデータの値
 
     //string
     let mut s1 = String::from("hello");//string型は文字の長さ可変
     //stringは24byteのデータが割り振られている。静的領域ではなくヒープ領域に保存されるのでアロケータによりデータの長さが可変になる。
     //最後の8byteはキャパシティと呼ばれ、ヒープ領域でのメモリの上限が格納されている。rust側で自動で割り振られる。
     println!("Capacity is {}", s1.capacity());
     s1.push_str("_new1");
     println!("s1 is {}", s1);

}