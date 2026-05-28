pub fn my_loop()
{
    let array:[i32;10] = [10, 20, 30, 40, 50,1,2,3,4,5];
    
    //loop循环
    let mut counter = 0;
    loop{
        counter += 1;
        if counter >= 10
        {
            break;
        }
        println!("loop循环！")
    }
    
    //while循环
    let mut whilecounter = 0;
    while whilecounter < 10 {
        whilecounter += 1;
        println!("while循环")
    }
    
    //for循环
    for i in 0..10{
        println!("for循环");
    }
}