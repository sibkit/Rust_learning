use std::fmt::{Display, Formatter};

pub struct User
{
    pub login: String,
    pub name: String,
    pub email: String,
    pub sign_in_count: u64,
}

pub struct UserCheck
{
    pub check_one: i32,
    pub check_two: i32
}

impl User
{
    #[allow(dead_code)]
    pub fn get_login(&self) ->&String
    {
        &self.login
    }


}

impl Copy for UserCheck {}

impl Clone for UserCheck
{
    fn clone(&self) -> Self
    {
        UserCheck
        {
            check_one: self.check_one,
            check_two: self.check_two
        }
    }
}









impl Display for User
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        return write!(f,"User {{name: \"{}\', login: \"{}\"}}",self.name,self.login);
    }
}