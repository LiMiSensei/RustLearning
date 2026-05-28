pub fn my_slice()
{
    //切片
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
}