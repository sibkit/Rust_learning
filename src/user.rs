use std::fmt::{Display, Formatter};

pub struct User
{
    pub login: String,
    pub(crate) name: String,
    pub email: String,
    pub sign_in_count: u64,
}

impl Display for User
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        return write!(f,"User {{name: \"{}\', login: \"{}\"}}",self.name,self.login);
    }
}