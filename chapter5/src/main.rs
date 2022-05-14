struct User {
    active: bool,
    email: String,
    sign_in_count: u64,
    name: String,
}

fn main() {
    //To use a struct after we’ve defined it, we create an instance of that struct by specifying concrete values for each of the fields.
    let user1 = User {
        active: true,
        email: String::from("daviddamilola20@gmail.com"),
        sign_in_count: 2,
        name: String::from("king david"),
    };
    // To get a specific value from a struct, we use dot notation.
    // If the instance is mutable, we can change a value by using the dot notation and assigning into a particular field.

    let mut user2 = User {
        active: true,
        email: String::from("daviddamilola20+2@gmail.com"),
        sign_in_count: 2,
        name: String::from("king david2"),
    };
    user2.name = String::from("david_d_slayer");

    let user3 = build_user(
        String::from("daviddamilola20+3@gmail.com"),
        String::from("david_d_slayer3"),
    );

    // Creating Instances From Other Instances With Struct Update Syntax
    // kinda like polymorphism
    // create a new instance of a struct that includes most of the values from another instance, but changes some.
    // The syntax .. specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance.
    //  The ..user1 must come last to specify that any remaining fields should get their values from the corresponding fields in user1
    // we can choose to specify values for as many fields as we want in any order, regardless of the order of the fields in the struct’s definition.
    // the struct update syntax uses = like an assignment; this is because it moves the data
    //we can no longer use user1 after creating user4 because the String in the email field of user1 was moved into user4
    // The types of active and sign_in_count are types that implement the Copy trait, so they are copied not moved.
    let user4 = User {
        name: String::from("king david4"),
        ..user1
    };
    println!(
        "user 1 name = {}, user 2 name = {}, user 3 = {}, user 4 = {}",
        user1.name, user2.name, user3.name, user4.name
    );

    // if a struct is going to be mutable, the entire instance must be mutable; Rust doesn’t allow us to mark only certain fields as mutable.
}

// we can construct a new instance of the struct as the last expression in the function body to implicitly return that new instance.
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        email,
        sign_in_count: 2,
        name: username,
    }
}

//

// Tuple structs are useful when you want to give the whole tuple a name and make the tuple a different type from other tuples, and when naming each field as in a regular struct would be verbose or redundant.

fn tupple_struct() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    // Each struct you define is its own type, even though the fields within the struct have the same types.
    // tuple struct instances behave like tuples: 
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}


// Unit-Like Structs Without Any Fields
// Unit-like structs can be useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself.

fn unit_struct() {
    struct AlwaysEqual;
    let subject = AlwaysEqual;
}

// User struct definition used the owned String type rather than the &str string slice type in the examples above
// this is because we want each instance of this struct to own all of its data and for that data to be valid for as long as the entire struct is valid.
// It’s also possible for structs to store references to data owned by something else, but to do so requires the use of lifetimes,