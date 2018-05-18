// FIXME: Make me compile! Diff budget: 2 lines.

// Do not change this definition.
enum MyEnum {
    A(String),
    B(String)
}

fn matcher(val: &mut MyEnum) { //-> &str {
    let y = 
	    match *val {
	        MyEnum::A(string) => 4,//string.as_str(),
	        MyEnum::B(string) => 4//string.as_str()
	    };
}

fn main() { }
