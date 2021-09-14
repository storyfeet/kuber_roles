//mod err;
mod roles;

fn main() {
    let s = crate::roles::get_roles().expect("Got Good result");
    println!("ROLES === {:?} === ROLES", s)
}
