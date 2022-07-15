pub fn run(){
    //配列は8MB
    let a1: [u8; 7000000] = [1; 7000000];//7MB
    // let a1: [u8; 9000000] = [1; 9000000];//9MB stack overflow

    //vector
    let mut v1 = vec![1,2,3,4];
    let v2 = vec![5,6,7,8];
    let v3 = vec![9,10];
}