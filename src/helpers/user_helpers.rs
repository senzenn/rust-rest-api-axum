
pub fn  say_hello()-> &'static str{
    "hello from user helpers"
}

pub fn  say_hello_to(name: &str)-> String{
    format!("hello {} from user helpers", name)
}