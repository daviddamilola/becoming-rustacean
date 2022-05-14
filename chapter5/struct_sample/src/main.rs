//  The Debug trait enables us to print our struct in a way that is useful for developers so we can see its value while we’re debugging our code
//  we have to explicitly opt in to make debug functionality available for our struct.
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Methods can take ownership of self, borrow self immutably as we’ve done here, or borrow self mutably, just as they can any other parameter
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // methods can take parameters other than self
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    //methods are a kind of associated functions
    //associated functions can take self and may not have self as the first paramenter
    // to use associated functions that dont have self as the first parameter we do Rectangle::square according to the defined method below
    // however it must return the struct type it implements
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // calculates area of a rectangle
    // dbg! macro takes ownership of an expression, prints the file and line number of where that dbg! macro call occurs in your code along with the resulting value of that expression, and returns ownership of the value.
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    //by default, the curly brackets tell println! to use formatting known as Display
    //  note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
    dbg!(&rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

// METHOD SYNTAX
// Unlike functions, methods are defined within the context of a struct (or an enum or a trait object)
//  their first parameter is always self, which represents the instance of the struct the method is being called on.

// A struct is allowed to have multiple  impl blocks
