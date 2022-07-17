enum OS {
    Windows(u32, String),
    Mac(u32,String),
    Linux(u32,String),
}
pub fn run(){
    let Linux = OS::Linux(1991, String::from("Linus"));
    let Windows = OS::Windows(1985, String::from("MicroSoft"));

    print_os_info(Windows);
    print_os_info(Linux);

}

fn print_os_info(os: OS) {
    match os {//パターマッチング：switchと同じ
        OS::Windows(year, who) => {
            println!("{} {}", year,who);
        }
        OS::Linux(year, who) => {
            println!("{} {}", year,who);
        }
        OS::Mac(_, _) => todo!(),
    }
}