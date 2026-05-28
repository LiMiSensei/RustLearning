pub struct MyStruct
{
    width : u32,
    height : u32,
}

impl MyStruct
{
    pub fn new(width : u32, height : u32) -> MyStruct
    {
        MyStruct
        {
            width,
            height,
        }
    }
}