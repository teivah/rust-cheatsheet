use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io;

mod utils;

fn main() {
    // To avoid compiler warnings with unused variables or function we can either:
    // * Name it _
    // * Or prefix it with _ (e.g. _length)

    // TODO to_owned()
    // TODO library (lib.rs) vs application (main.rs)

    cargo();
    comment();
    primitives();
    heap_stack();
    string();
    tuples();
    array();
    vector();
    functions();
    control_flow();
    loops();
    structs();
    enums();
    closures();
    collections_map();
    impls_traits();
    pattern_matching();
    if_let();
    formatted_print();
    generics();
    option();
    let _ = result();
    ownership();
    borrowing();
    dangling();
    lifetimes();
    panicking();
    iterator();
    testing();
}

fn cargo() {
    // Building an application for testing
    // $ cargo build
    // Building for production, slowest to build but fastest at runtime because of extra optimizations
    // $ cargo build --release
}

/// A 3-slash comment is used to create an exportable documentation.
/// It supports **Markdown**.
/// It can be generated and visualized if the project is a library using $ cargo doc -- open
fn comment() {
    // Single line comment

    /*
    Multiline comment
     */
}

fn primitives() {
    // Type inference
    let _ = 1;
    // We can set the type during the declaration
    let _ = 1i8;

    // Integers: from i8 to i128
    let _: i8 = 1;
    let _: i128 = 1;

    // Conversion
    let i: i64 = 1;
    let _: i32 = i as i32;

    // Unsigned integers: from u8 to u128
    let _: u8 = 1;
    let _: u128 = 1;

    // Integer or unsigned integer based on the architecture (32 or or 64 bits)
    let _: isize = 1;
    let _: usize = 1;

    // bool
    let _ = false;

    // Shadowing
    // Example: same name but different type
    let _ = "foo";
    let _ = false;

    // Immutability
    let _ = "foo";
    // Compilation error
    // _ = "bar";
    let mut _s = "bar";
    // Now we can mutate m
    _s = "bar";

    // Const
    const _FOO: f32 = 3.1;

    // Static variable
    // TODO Difference between const and static variable?
    static _BAR: i32 = 3;
}

fn heap_stack() {
    // Stack: fixed size variables (primitives or array of primitives)
    // Memory is recaptured after the variable goes out of scope
    // Default assignment is a copy

    // Memory than can grow in size (vector, String, structs, etc.)
    // Memory is recaptured after the last owner goes out of scope
    // Default assignment is a transfer of ownership (see later)
    // Slower than stack
}

fn string() {
    // String slice
    // Immutable, allocated on the stack (if declared from a literal) or the heap
    let s: &str = "foo";

    // Second type of string
    // Mutable (it declared with mut), allocated on the heap
    let mut s2: String = String::from("foo");

    // Concatenation
    let s: &str = &[s, "bar"].concat();
    s2.push_str("bar");
    // TODO: bench

    // Convert from and to string literal
    let _ = s.to_string();
    let _ = s2.as_str();

    // String slice is a reference to a subset of a string
    // If we create a string slice in the middle of a multibyte character, the program will exit
    let s: String = String::from("foo");
    let _slice: &str = &s[1..2]; // o
    let _slice: &str = &s[..2]; // fo
    let _slice: &str = &s[1..]; // oo
    let _slice: &str = &s[..]; // foo

    // Format a string
    let i = 1;
    let _s = format!["foo {}", i];

    // String concatenation
    let s1: String = String::from("foo");
    let s2: String = String::from("bar");
    let _: String = s1 + &s2;
    // String slice concatenation
    let s1: &str = "foo";
    let s2: &str = "foo";
    let _: String = s1.to_owned() + &s2.to_owned();

    // The following operation is not possible
    // An char can be encoded on multiple bytes
    // Rust does not allow this as may access an invalid character on its own
    // let _ = s[0];
    // Instead, we have to iterate to get each grapheme clusters (a letter)
    for _c in "foo".chars() {}
    // Or we can slice the string to get particular bytes
    let _s: &str = &"foo"[..1];
}

fn tuples() {
    // A tuple collection of values of different types
    let t = (true, 1);

    // Assign elements from a tuple
    let _ = t.0;
    let _ = t.1;

    // Or with a single line
    let (_i, _j) = t;
}

fn array() {
    // Fixed size array
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // Access element
    let _ = a[0];
    // Does not compile as the array is not mutable
    // a[1] = 10;
    // In order to mutate it we have to declare it mutable
    let mut _a: [i32; 5] = [1, 2, 3, 4, 5];
    // a[1] = 10;

    // Initializes the 5 elements to value 0
    let a: [i32; 5] = [0; 5];

    // Get array length: 5
    let _ = a.len();

    // Create slice from array
    // A slice is a pointer on an array (it is not resizable)
    let _: &[i32] = &a;
    // We cannot modify a slice regardless if the backed array is mutable or not
    // s[1] = 6;

    // Create slice from sub array (included/excluded)
    let s = &a[1..3];
    // Slice length is 2
    let _ = s.len();
}

fn vector() {
    // A vector is like a Go slice
    // Backed by an array, with length and capacity, resizable

    // Initialization - 0 capacity
    let _: Vec<i32> = Vec::new();
    // Initialization using a macro - 0 capacity
    let _: Vec<i32> = vec![];
    // Initialization with values
    let _ = vec![1i32, 2, 3];
    // Initialization with initial capacity and mutable
    let mut v: Vec<i32> = Vec::with_capacity(10);

    // Add element, added to index 0
    v.push(10); // [10]
    // Added to index 1
    v.push(20); // [10, 20]

    // Remove latest element
    v.pop(); // [20]

    // Get an element returns an option
    let _: Option<&i32> = v.get(0);
    // Accessing an index outside the vector will not panic, it returns a None
    let _: Option<&i32> = v.get(100);

    // Get vector length and capacity
    let _ = v.len();
    let _ = v.capacity();

    // Iteration reference
    // i being a &i32
    for _i in &vec![1i32, 2, 3] {}
    for _i in vec![1i32, 2, 3].iter() {}
    // Iteration mutable reference
    // i being an &mut i32
    for _i in &mut vec![1i32, 2, 3] {}
    for _i in vec![1i32, 2, 3].iter_mut() {}
    // Iteration ownership: into_iter takes the ownership of the vector values
    // i being an i32
    let v = vec![1i32, 2, 3];
    for _i in v {}
    // After this operation, we cannot use v anymore
    for _i in vec![1i32, 2, 3].into_iter() {}

    // Copy a vector
    let _ = vec![1i32, 2, 3].to_vec();
}

fn functions() {
    // Function call
    let _ = increment(1);

    // Higher order function
    let f = increment;
    let _ = f(1);

    // Partially applied function
    let partially_applied_functions = |x| multiply(3, x);
    let _ = partially_applied_functions(6);
}

fn increment(a: i32) -> i32 {
    // The latest expression is the value returned
    a + 1
    // Equivalent to
    // return a + 1;
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn control_flow() {
    // If/else
    let i = 1;
    if i < 2 {
        println!("less");
    } else {
        println!("more");
    }

    // If/else assignment
    let _ = if i == 5 { true } else { false };
}

fn loops() {
    // For (included/excluded)
    for _ in 0..3 {}

    // Reverse (still included/excluded)
    for i in (1..3).rev() {
        println!("rev={}", i)
    }

    // While
    // a has to be mutable
    let mut i = 0;
    while i <= 3 {
        i += 1;
    }

    // Labelled while
    let mut i = 0;
    'while1: while i < 3 {
        break 'while1;
    }

    // Loop without conditions
    loop {
        break;
    }

    // Loop assignement
    let mut sum = 0;
    let _ = loop {
        sum += i;
        i += 1;
        if i == 3 {
            break sum < 10;
        }
    };
    // Use case for a loop: retry an operation that might fail

    // Array iteration
    let a: [&str; 3] = ["foo", "bar", "baz"];
    // Over each element index
    for i in 0..a.len() {
        let _ = a[i];
    }
    // Or directly over each element
    for element in a.iter() {
        let _ = element;
    }
}

fn structs() {
    // 3 types of structs: C-like, tuple and units
    // A structure is immutable by default

    // C-like
    // We need to assign a value for each member of the structure otherwise it won't compile
    let person = Person1 {
        name: "foo".to_string(),
        age: 18,
    };
    // Access
    let _ = person.name;
    let _ = person.age;
    // Copy elements
    let _ = Person1 {
        name: "bar".to_string(),
        ..person // Copy the rest of the elements, does not compile without it
    };
    // If we hold variables whose name are the same than the fields, we can pass them directly
    let name = String::from("foo");
    let age = 1;
    let _ = Person1 { name, age };

    // Tuple
    let person = Person2("foo".to_string(), 20);
    // Accessing elements
    let _ = person.0;

    // Unit
    // A simple structure without members
    // TODO Use case?
    let _ = UnitStruct;
}

// C-like structure
// The naming convention for each structure type is pascal case
#[derive(Debug)]
struct Person1 {
    name: String,
    age: u32,
}

// Tuple structure
struct Person2(String, u32);

// Unit structure
struct UnitStruct;

fn enums() {
    // Simple enum (based on units)
    let _ = Enum::Foo;
    let _ = Enum::Bar;

    // Enum with variants
    let _ = EnumWithVariants::Foo { id: 1, age: 1 };
    let _ = EnumWithVariants::Bar("foo".to_string());
}

enum Enum {
    Foo,
    Bar,
}

enum EnumWithVariants {
    Foo { id: i32, age: i32 }, // Struct variant
    Bar(String),               // Tuple variant
}

fn closures() {
    // Three types of closure
    // FnOnce: consume the variables it captures (all closures implement FnOnce as they can all be called at least once)
    // FnMut: mutable borrow
    // Fn: immutable borrow

    // Closure example
    let _ = |x: i32, y: i32| -> i32 { x + y };

    // To force a closure to take ownership of the values we can use move keyword
    let _ = move |x: Point| -> Point { x };

    // Direct closure call
    let _: i32 = { 1 + 2 };

    // A struct can hold a closure
    let s = StructWithClosure { f: |x| x + 1 };
    let _ = (s.f)(1);

    // A structure holding a closure with generics
    let s = StructWithClosureAndGenerics { f: |x| x + 1 };
    let _ = (s.f)(1);
}

struct StructWithClosure {
    f: fn(i32) -> i32,
}

struct StructWithClosureAndGenerics<T>
    where
        T: Fn(u32) -> u32,
{
    f: T,
}

fn collections_map() {
    // Hashmap creation
    let _: HashMap<String, i32> = HashMap::new();
    // With initial capacity
    let mut map: HashMap<String, i32> = HashMap::with_capacity(32);

    // Insert elements
    let s = String::from("one");
    let i = 1;
    map.insert(s, i);
    // If a type implements the Copy trait (e.g. i32), the values are copied into the hash map
    // Hence it is valid to reuse i
    let _ = i;
    // Otherwise, the values are moved. Hence, it is invalid to reuse s
    // let _s2 = s;

    // Insert if the key does not already exist
    let v: &mut i32 = map.entry(String::from("two")).or_insert(2);
    // At this stage, we can also mutate v to mutate the value inside the map
    *v = 20;

    // Iteration
    // key is an &String whereas value is an &i32
    for (_key, _value) in &map {}

    // Create a hashmap from two vectors
    let v1 = vec![
        String::from("one"),
        String::from("two"),
        String::from("three"),
    ];
    let v2 = vec![1, 2, 3];
    let _: HashMap<&String, &i32> = v1.iter().zip(v2.iter()).collect();
}

fn impls_traits() {
    // An impl is used to define method for structs and enums
    let _ = Point { x: 1, y: 1 }.foo(Point { x: 2, y: 2 });

    // An associated (static) function call
    let point = Point::new(1, 1);

    // Point implements the Adder trait
    // The following function an Adder
    // We pass the structure pointer
    function_accepting_trait(&point);

    // Point also implements the generic trait TraitWithGenerics
    let _ = Point::convert_to_tuple(Point { x: 1, y: 1 });

    // Inheritance example
    // Ferrari implementing Vehicle and Car
    let _ = Ferrari {
        id: "001".to_string(),
        color: "red".to_string(),
    };
}

struct Point {
    x: i32,
    y: i32,
}

// A collection of methods on Point structure
impl Point {
    // TODO self (ownership), &self (reference) or &mut self (mutable reference)?
    fn foo(self, point: Point) -> Point {
        Point {
            x: self.x + point.x,
            y: self.y + point.y,
        }
    }

    // Associated function (static)
    // It does not take a self argument
    fn new(a: i32, b: i32) -> Self {
        Point { x: a, y: b }
    }
}

// TODO: dyn keyword?
fn function_accepting_trait(_: &dyn Trait) {}

// Trait example
trait Trait {
    fn add(&self, p2: Point) -> Point;
    // An interface can also have default methods
    fn default_method(&self) {
        println!("default method");
    }
}

// The implementation of the trait has to be done explicitely
impl Trait for Point {
    fn add(&self, point: Point) -> Point {
        Point {
            x: self.x + point.x,
            y: self.y + point.y,
        }
    }
}

// Trait with generics
trait TraitWithGenerics<T> {
    fn convert_to_tuple(t: T) -> (i32, i32);
}

impl TraitWithGenerics<Point> for Point {
    fn convert_to_tuple(point: Point) -> (i32, i32) {
        (point.x, point.y)
    }
}

trait Vehicle {
    fn id(self) -> String;
}

// Car inherits from vehicle
trait Car: Vehicle {
    fn color(self) -> String;
}

struct Ferrari {
    id: String,
    color: String,
}

// CarImpl has to implement both traits: Vehicle and Car
// Vehicle implementation
impl Vehicle for Ferrari {
    fn id(self) -> String {
        self.id
    }
}

// Car implementation
impl Car for Ferrari {
    fn color(self) -> String {
        self.color
    }
}

fn pattern_matching() {
    // Pattern matching on an integer
    let i = 1;
    // The match has to cover all the cases
    // The following example does not compile if we omit the last case
    let _ = match i {
        0 => "zero",
        1 => "one",
        _ => "other", // Other cases
    };

    // If we want to do nothing
    let _ = match i {
        0 => println!("zero"),
        _ => (),
    };

    // Comparing integers
    // It uses std::cmp::Ordering
    let i = 1;
    let j = 2;
    let _ = match i.cmp(&j) {
        Ordering::Less => "less",
        Ordering::Greater => "greater",
        Ordering::Equal => "equals",
    };

    // Matching an interval
    let level = 22;
    let _ = match level {
        1..=5 => "beginner",
        6..=10 => "intermediate",
        11..=20 => "expert",
        _ => "other",
    };

    // Pattern matching on a tuple
    let i = 1;
    let j = 1;
    let _ = match (i, j) {
        (1, 2) => "one, two",
        // Default values on a single element
        (1, _) => "1, default", // This is the one that will be matched
        // Default values on both elements
        (_, _) => "default, default",
    };

    // Enum matching
    let e = EnumWithVariants::Bar("foo".to_string());
    match e {
        // We have to use all the defined in the structure and their field names
        // TODO Do we?
        EnumWithVariants::Foo { id, age } => println!("id={}, age={}", id, age),
        EnumWithVariants::Bar(foo) => println!("element={}", foo),
    }
}

fn if_let() {
    // If we need a match that runs code when the value matches one pattern only, we can use if let
    let i = 1;
    if let 0 = i {
        println!("zero");
    }

    // Example with enum
    let e = EnumWithVariants::Bar("foo".to_string());
    if let EnumWithVariants::Foo { id, age } = e {
        println!("id={}, age={}", id, age);
    }
}

fn formatted_print() {
    // Formatted print
    println!("Hello, world! {}", 42);
    println!("Hello, world! {} {}", 42, "foo");

    // Indexed
    println!("Hello {0} {1}, {1} {0}", "foo", "bar");

    // Named
    println!("hello {name}", name = "foo");

    // Specific type
    println!("{:b} in binary", 3);

    // {:?} is the debug output format
    let v = vec![1i32, 2, 3];
    println!("{:?}", v);
    // It works with structures implementing the Debug trait or the annotation #[derive(Debug)]
    let s = Person1 {
        name: "foo".to_string(),
        age: 1,
    };
    println!("{:?}", s);
}

fn generics() {
    // Generics in Rust do not have any performance impact
    // Rust accomplishes this using monomorphization
    // It is the process of turning generic code into specific code at compile time

    // Generic function call
    // The function accepts only structures implementing Trait
    let _ = generic_function(Point { x: 1, y: 1 }, Point { x: 1, y: 1 });
    // This is different that the following function that accepts Trait implementation
    let _ = function_accepting_implementation(Point { x: 1, y: 1 }, Point { x: 1, y: 1 });
    // In the first example, we have to passe the same structure type
    // In the second example, we can pass two different structure types as long as they both implement Trait

    // A function can also specify multiple traits
    specifying_multiple_trait(utils::Struct {}, utils::Struct {});
    // An alternative syntax
    specifying_multiple_trait_alternative_syntax(utils::Struct {}, utils::Struct {});

    // Instantiate a generic structure
    let s = GenericStruct { x: 1, y: 2 };
    println!("{} {}", s.x, s.y);

    // A generic implementation
    let _ = s.generic_function();

    // A specific implementation for GenericStruct<i32> only
    let _ = s.specific_function();

    // Instantiate a generic enum
    let _ = GenericEnum::<i32, String>::Foo(1);
    let _ = GenericEnum::<i32, String>::Bar("foo".to_string());
}

fn generic_function<T: Trait>(x: T, _: T) -> T {
    x
}

fn function_accepting_implementation(x: impl Trait, _: impl Trait) -> impl Trait {
    x
}

fn specifying_multiple_trait<T: utils::Trait1 + utils::Trait2>(x: T, _: T) -> T {
    x
}

fn specifying_multiple_trait_alternative_syntax<T>(x: T, _: T) -> T
    where
        T: utils::Trait1 + utils::Trait2,
{
    x
}

struct GenericStruct<T> {
    x: T,
    y: T,
}

impl<T> GenericStruct<T> {
    fn generic_function(&self) -> &T {
        &self.x
    }
}

impl GenericStruct<i32> {
    fn specific_function(&self) -> i32 {
        self.x + self.y
    }
}

enum GenericEnum<T, E> {
    Foo(T),
    Bar(E),
}

fn option() {
    // Rust does not have nulls. Instead, it has an unum that can encode this concept of present or absent.

    // Option means the possibility of absence
    let option = option_example(0);

    // We use pattern matching to check an option
    match option {
        None => println!("none"),
        Some(i) => println!("{}", i),
    }

    // We can also map directly a result using expect
    // The code will panic if result contains an error
    let option = option_example(0);
    let _: String = option.expect("foo");

    // We can use unwrap to get the value
    // Yet, it panics if the option is None
    let option = option_example(0);
    let _: String = option.unwrap();

    // unwrap_or_else takes a closure to return a defaault value
    let _: String = option_example(0).unwrap_or_else(|| String::from("foo"));
}

fn option_example(i: i32) -> Option<String> {
    if i == 0 {
        return Some("zero".to_string());
    }
    None
}

fn result() -> Result<i32, io::Error> {
    // Result means the possibility of an error
    let result = result_example(1, 1);

    // We can use pattern matching to check a result
    match_result(result);
    // If the function returns a Result as well, we can use the ? operator
    File::open("foo")?;

    // We can also map directly a result using expect
    // The code will panic if result contains an error
    let _i: i32 = result.expect("foo");
    // Panic if the result contains an error
    let _i: i32 = result.unwrap();

    // A function calling another function that returns a result can use the ? operator
    let _result = result_caller(1, 0);

    Ok(0) // Just to comply with the function signature
}

fn result_example(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 {
        return Err("b is nil");
    }
    Ok(a / b)
}

fn match_result(result: Result<i32, &str>) {
    match result {
        Err(e) => println!("error: {}", e),
        Ok(i) => println!("value: {}", i),
    }
}

fn result_caller(a: i32, b: i32) -> Result<i32, &'static str> {
    // If result_example returns an error, we automatically returns the error
    let i = result_example(a, b)?;
    Ok(i)
}

fn ownership() {
    // Ownership rules:
    // * Each value has a variable that is called the owner
    // * Only one owner at a time
    // * When owner goes out of scope (its range), the value is dropped

    // When a variable goes out of scope, the memory is automatically returned
    {
        let _ = 1;
    } // i being out of scope at this stage, it is freed

    // This example assign a primitive
    // Hence, the value is copied to j
    // Both are owners of an i32 value
    let i = 1;
    let _j = i;

    // A copy is used if the value:
    // * Is a primitive
    // * Is a tuple or an array of primitives

    // Example of an array of primitives copy
    let a = [1, 2, 3];
    let a2 = a;
    // Hence, it is valid to access both arrays at this stage
    println!("v={:?}, v2={:?}", a, a2);

    let s = String::from("foo");
    // We transfer the ownership of the string value to s2
    let _ = s;
    // Hence, Rust will free s so the following code will not compile anymore
    // println!("{:?}", s);

    // The semantics for passing a value to a function is similar to variable assignment
    let s = String::from("foo");
    takes_ownership(s);
    // At this stage, we cannot use s
    // Returning a value from a function does also transfer ownership

    // Having a method that takes ownership of the instance by using just self is rare
    // Usually used when the method transforms self into something else
    // Hence, we want to prevent the caller from using the original instance
    Foo {}.bar();
}

fn takes_ownership(_: String) {}

struct Foo {}

impl Foo {
    fn bar(self) {}
}

fn borrowing() {
    // To reuse a resource, we have to borrow it
    // Borrowing is about passing variable bindings either:
    // * To other functions
    // * Or to other variable bindings

    // Two types of borrowing:
    // * Shared: data can be borrowed by multiple users but should not be altered
    // * Mutable: data can be borrowed and altered by a single user

    // Benefit of these restrictions: data race analysis done by the compiler

    // Rules:
    // 1. A borrow is either a shared or a mutable
    // 2. Borrowing applies for both copy and move

    // Shared borrowing
    // Multiple immutable references are allowed
    let v1 = vec![1, 2, 3];
    let v2 = &v1;
    let v3 = &v1;
    println!("v={:?}, v2={:?}, v3={:?}", v1, v2, v3);
    // Yet, this code does not compile as v2 have v3 have a reference to the vector
    // As long as a reference exists, we cannot modify the value
    // v1.push(4);

    // Mutable borrowing
    // Only one mutable reference is allowed
    let mut v1 = vec![1, 2, 3];
    let v2 = &mut v1;
    // Does not compile as a mutable borrow is owned by a single user
    // println!("v={:?}, v2={:?}", v, v2);
    // Here, only v2 should read and access the vector
    v2[0] = 10;
    let _ = v2[0];

    // The same applies while calling a function
    // Shared borrowing
    let v = vec![1, 2, 3];
    function_accepting_shared_borrowing_vector(&v);
    // Mutable borrowing
    let mut v = vec![1, 2, 3];
    function_accepting_mutable_borrowing_vector(&mut v);
    // At this stage, the ownership has been automatically returned to v
    v.push(4);

    // Also, it's not possible to have a mutable reference while having an immutable one first
    // Users of an immutable reference don't expect the value to change
    let s = String::from("foo");
    let _ = &s; // First immutable reference
    let _ = &s; // Second immutable reference

    // Does not compile
    // let s3 = &mut s;
}

fn function_accepting_shared_borrowing_vector(_: &Vec<i32>) {}
fn function_accepting_mutable_borrowing_vector(_: &mut Vec<i32>) {}

fn dangling() {
    // A dangling pointer is a pointer that references an invalid location in memory
    // Rust prevent this from happening
}

// The code will not compile
// fn dangling_example() -> &String {
//     let s = String::from("foo");
//     &s
// }

fn lifetimes() {
    // A lifetime is a construct the compiler (borrow checker) uses to ensure all borrows are valid
    // It checks how long a reference should be valid
    // Lifetime annotations are check at compile-time (main reason for slower compilation times)
    // Yet, it brings safety to the language

    // Lifetimes are marked with an apostrophe
    // Convention: lowercase, starts with 'a and follows alphabetic order (if multiple annotations)

    // **Every reference should have a lifetime** (either manual or added by the compiler if possible)

    // The three compiler rules regarding lifetimes references:
    // 1. Each parameter that is a reference gets itw own lifetime parameter
    // A function with one paramter gets one lifetime, a function with two gets two separate lifetime parameters
    // Example: fn foo<'a, 'b>(x: 'a i32, y: &'b i32)
    // 2. If there is exactly one input lifetime parameter, the lifetime is assigned to all out lifetime parameters
    // If there are multiple input lifetime parameters but one of them is &self or &mut self (a method)
    // the lifetime of self is assigned to all output lifetime parameters

    // This code does not compile
    let _s1;
    {
        let s2 = String::from("foo");
        _s1 = &s2;
    }
    // The following code will raise the following error:
    // "borrowed value does not live long enough"
    // This is because the ownership of the memory was never transferred to s1
    // Hence, it will be cleaned up at the end of the internal scope (the {})
    // println!("{}", s1);

    // The following function would not compile:
    /*
    fn longest(x: &str, y: &str) -> &str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
     */
    // Rust can't tell whether the reference returned refers to x or y

    // Instead, we have to set a lifetime annotation
    let _ = longest_with_lifetime("foo", "bar");

    // In this example, the function returns the input reference
    // Hence, the compiler does not need to know what's the lifetime
    // We could have omitted the lifetime annotation
    let s1 = "foo";
    let _ = _function_with_useless_lifetime(s1);

    // Returning a static reference
    let _ = lifetime_function_static();

    // Lifetime on a structure
    let x = _LifetimeStruct { s: "foo" };
    println!("{}", x.s);

    // All string literals have the 'static lifetime
    // The text of this string is stored directly in the binary of the program
    let _: &'static str = "I have a static lifetime.";

    // Generics and lifetimes
    let _ = generics_with_lifetimes(&Point { x: 1, y: 1 });
}

// Inputs and output share the same lifetime
fn longest_with_lifetime<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn _function_with_useless_lifetime<'a>(s: &'a str) -> &'a str {
    s
}

// Static means a the same life time than the whole application
fn lifetime_function_static() -> &'static str {
    "foo"
}

struct _LifetimeStruct<'a> {
    // s should live as long as LifetimeStruct exists
    s: &'a str,
}

fn generics_with_lifetimes<'a, T>(x: &'a T) -> &'a T
    where
        T: Trait,
{
    x
}

fn panicking() {
    // Panic is **specific to a thread**
    // We can format the panic message
    if false {
        panic!("{}", 0);
    }

    // When a panic occurs, the program starts unwinding to collect the stack
    // There is an alternative to abort instead by adding this configuration in Cargo.toml
    // [profile.release]
    // panic = 'abort'

    // If we want to see the user backtrace (the list of calling functions) leading to the panic
    // We can run the application this way: $ RUST_BACKTRACE=1 cargo run

    // Panic "not yet implemented"
    if false {
        todo!();
    }

    // Panic "not implemented"
    if false {
        unimplemented!();
    }

    // If we enter a route we should not
    // Panic "internal error: entered unreachable code"
    let level = 18;
    let _ = match level {
        1..=5 => "beginner",
        6..=10 => "intermediate",
        11..=20 => "expert",
        _ => unreachable!(),
    };
}

fn iterator() {
    // An iterator is lazy
    // It is a zero-cost abstraction, meaning no additional runtime overhead

    // Closure to check if a number is a prime number
    let is_prime_number = |n: i32| -> bool { !(2..n - 1).any(|i| n % i == 0) };

    // Get nth prime number
    let n = 5;
    let _: Option<i32> = (1..).filter(|i| is_prime_number(*i)).nth(n);

    // An example producing a vector or i32 by mapping each element and collecting the results
    // We iterate from i1 to i2 (included because of =i2) reversely
    let i1 = 0;
    let i2 = 10;
    // increment being a i32->i32 function
    let _: Vec<i32> = (i1..=i2).rev().map(increment).collect::<Vec<_>>();

    // Chaining example with a single element
    // It produces: 1,2,3,4
    let _: String = (1..3)
        .map(|x| x.to_string())
        .chain(std::iter::once("4".to_string()))
        .collect::<Vec<_>>()
        .join(",");

    // Custom iterator
    let mut custom_iterator = CustomIterator::new(vec![1, 2, 3]);
    let _ = custom_iterator.next(); // 1
}

struct CustomIterator {
    vec: Vec<i32>,
    index: usize,
}

impl CustomIterator {
    fn new(vec: Vec<i32>) -> CustomIterator {
        CustomIterator { vec, index: 0 }
    }
}

impl Iterator for CustomIterator {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.vec.len() {
            self.index += 1;
            return Some(self.vec[self.index - 1]);
        }
        return None;
    }
}

fn testing() {
    // The best practice is to keep unit tests in a tests module in the end of the file
    // Integration tests should live in /tests folder, next to /src

    // To avoid running tests in parallel: $cargo test -- --test-threads=1
}

fn _sum(i: i32, j: i32) -> i32 {
    i + j
}

#[cfg(test)]
mod tests {
    // Import the parent module
    use super::*;

    #[test]
    // We can use #[ignore] to ignore a test
    fn test_sum() {
        assert_eq!(_sum(2, 2), 4);
    }
}
