fn main() {
    let mut x = 5;
    { //create a new scope to limit the lifetime of y
        let y = &mut x; 
        *y = 6;
    }
    let z = &mut x; 
    *z = 7;
    println!("x = {}", x);
}