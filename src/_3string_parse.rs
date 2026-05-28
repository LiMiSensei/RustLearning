pub fn read_to_int()
{
    println!("请输入文本：");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input);


    let number  = input.trim().parse().unwrap_or_else(|_| 0);

    println!("数字为：{}", number);
}