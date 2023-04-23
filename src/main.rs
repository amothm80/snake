//use rand;
//use piston_window;
mod draw;
mod snake;
mod game;
fn main() {
    let mut d:snake::Direction = snake::Direction::Up;
    d.opposite();
    println!("Hello, world!");
}
