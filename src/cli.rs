/// Echo Function
/// Prints out the user's input on the interface
/// # Arguments
/// ```args: Vec<&str>``` - vector representing space-separated inputs from user
pub fn echo(args: Vec<&str>) {
    let mut out: String = String::new();
    for input in args {
        out += input;
        out += " ";
    }
    if out.len() > 1 {
        out = out[..out.len() - 1].to_string();
    }
    println!("{}", out);
}