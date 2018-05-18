// FIXME: Make me compile! Diff budget: 1 line.
#[derive(Clone, Copy)]
struct MyType(usize);

// Do not modify this function.
pub fn main() {
    let x = MyType(1); //x is taking ownership
    let y = &x; //y takes a reference to MyType(1) (borrows it)
    let z = *y; //cannot do that, only the owner can move out 
}
