pub enum IpAddrKind
{
    V4,
    V6,
}

impl IpAddrKind
{
    pub fn kind(&self) -> IpAddrKind
    {
        IpAddrKind::V4
    }
    
    pub fn mu(IP:IpAddrKind) -> u32
    {
        match IP {
            IpAddrKind::V4 => {4}
            IpAddrKind::V6 => {256}
        }
    }
}