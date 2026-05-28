use std;
use std::io;

pub fn print_in()
{
    //打印信息
    print!("请输入信息{}\n","在控制台中：");
    
    //读取信息
    let mut r = String::new();
    let red = io::stdin().read_line(&mut r);//控制台的输入给r

    //输出输入
    match red {
        Ok(n) => {print!("\n你的输入合法：:{}",r)}
        Err(e) => {println!("\n你的输入不合法{}",e)}
    }

}