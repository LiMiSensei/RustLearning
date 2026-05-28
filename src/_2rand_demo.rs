use rand::rngs;
use std::cmp::Ordering;

pub fn crat_rand()
{
    //生成随机数
    for i in 0..32 {
        let secret_number = rand::random_range(1..101);
        //println!("数据数为：{}",secret_number);

        //比较随机数
        let guess: u32 = 50;
        match secret_number.cmp(&guess) {
            Ordering::Less => println!("小于{}",&secret_number),
            Ordering::Equal => println!("等于{}",&secret_number),
            Ordering::Greater => println!("大于{}",&secret_number),
        }
    }


}