use crate::learning_game::run_game;
use crate::user::User;

mod user;
mod tree_node;
mod learning_game;



fn main()
{
   let user = User
   {
      login: "first".to_string(),
      name: "Fin".to_string(),
      email: "fin@refactor.team".to_string(),
      sign_in_count: 45,
   };

   if user.login == "first".to_string() || user.name == "Fin".to_string()
   {
      println!("{} {}", user.name, user.login)
   }

   run_game();

}


