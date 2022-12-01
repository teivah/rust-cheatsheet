#![allow(unused)]

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

mod utils;

fn main() {
    // TODO to_owned()
    // TODO library (lib.rs) vs application (main.rs)
    // TODO Chapter 7

    cargo();
    comment();
    scalar_types();
    variables();
    const_static_variables();
    heap_stack();
    string();
    tuples();
    control_flow();
    loops();
    array();
    vector();
    functions();
    structs();
    enums();
    closures();
    hashmap();
    impls_traits();
    generics();
    pattern_matching();
    if_let();
    formatted_print();
    option();
    let _ = result();
    ownership();
    copy();
    borrowing();
    dangling();
    lifetimes();
    panic();
    iterator();
    smart_pointers();
    file();
    sort();
    testing();
}

fn cargo() {
    /*
    Cargo is Rust's build system and package manager

    Useful commands:
    - Build an app for testing:
      $ cargo build
    - Build an app for production (slowest to build but fastest at runtime because of optimizations)
      $ cargo build --release
    - Checks your code to make sure it compiles (doesn't produce an excutable):
      $ cargo check
    - Run an app:
      $ cargo run
     */
}

/// A 3-slash comment is used to create an exportable documentation.
/// It supports **Markdown**.
/// It can be generated and visualized if the project is a library using `$ cargo doc --open`
fn comment() {
    // Single line comment

    /*
    Multiline comment
     */

    //! Adds documentation to the item that contains the comment instead of the item following the comment
    //! In this case, the surrounding function: `comment`
}

fn scalar_types() {
    /*
    Rust scalar types:
    - Signed integers: i8, i16, i32, i64, i128, and isize (pointer size, depends on the architecture)
    - Unsigned integers: u8, u16, u32, u64, u128, and usize (pointer size, depends on the architecture)
    - Floating points: f32, f64
    - Character: char
    - Boolean: bool
    - The unit type () whose only possible value is an empty tuple ()

    Note on variable names: to avoid compiler warnings with unused variables or functions, we can either:
    - Name it _
    - Or prefix it with _ (e.g., _length)
    - Import #![allow(unused)] as done in this file
     */

    // Type inference
    let i = 1;
    // The previous declaration is similar to this
    let i: i32 = 1;

    // We can set the type alongside with the value
    let i = 1i8;

    // bool
    let b = false;

    // char
    let c = 'x';
}

fn variables() {
    // Using let, a variable is immutable
    let s = 1;
    // The following line would trigger a compilation error
    // s = 2;

    // Using let mut, a variable is mutable
    let mut s = 1;
    // Now we can mutate m
    s = 2;

    // Shadowing: same name but different type
    let shadowed = 1;
    let shadowed = false;

    // Conversion
    let i: i64 = 1;
    let j = i as i32;

    // Variable names and functions are based on snake_case
    let apple_price = 32;
}

fn const_static_variables() {
    // Const (convention: SCREAMING_SNAKE_CASE; e.g., FOO_BAR_BAZ)
    // Note that type inference doesn't work with constants
    const FOO: f32 = 3.1;

    // Immutable static variable
    static BAR: i32 = 3;

    // Static variable
    static mut BAZ: i32 = 3;
    // A static variable can be mutable compared to a constant
    // Yet, it requires to be done inside an unsafe block
    unsafe {
        BAZ = 4;
    }
}

fn heap_stack() {
    /*
    Stack: fixed size data (primitives or array of primitives)
    Memory is recaptured after the variable goes out of scope
    Default assignment is a copy

    Heap: data with an unknown size at compile time or a size that might change
    (vector, String, structs, etc.)
    Memory is recaptured after the last owner goes out of scope
    Default assignment is a transfer of ownership (see later)

    The heap is slower than the stack for pushing and accessing data
     */
}

fn string() {
    // String slice
    // Immutable, allocated on the stack (if declared from a literal); otherwise, on the heap
    let s: &str = "foo";

    // Second type of string
    // Mutable (it declared with mut), allocated on the heap
    // A string is a wrapper over a Vec<u8>
    // Note: The :: operator means that a function is associated to a type not to an instance (static in Java)
    let mut s2: String = String::from("foo");

    // Concatenation
    // With String type
    let mut s = String::from("abc");
    s.push('d');
    s.push_str("bar");
    // With a string slice
    let s1: &str = "foo";
    let s2: &str = &[s1, "bar"].concat();

    // Convert from and to string literal
    let s1 = "abc";
    let s2 = s1.to_string();
    let s1 = s2.as_str();

    // String slice is a reference to a subset of a string
    // Range indices must occur at valid UTF-8 character boundaries
    let s: String = String::from("foo");
    let slice: &str = &s[1..2]; // o
    let slice: &str = &s[..2]; // fo
    let slice: &str = &s[1..]; // oo
    let slice: &str = &s[..]; // foo

    // If we create a string slice in the middle of a multibyte character, the program will panic
    let s = String::from("д");
    // let slice = &s[0..1];

    // String format
    let i = 1;
    let s = format!["foo {}", i];

    // String concatenation
    let s1: String = String::from("foo");
    let s2: String = String::from("bar");
    let s: String = s1 + &s2;
    // String slice concatenation
    let s1: &str = "foo";
    let s2: &str = "foo";
    let s: String = s1.to_owned() + &s2.to_owned();

    // The following operation is not possible
    // A char can be encoded on multiple bytes
    // Rust does not allow this as may access an invalid character on its own
    // let c = s[0];
    // Instead, we have to iterate to get each grapheme clusters (a letter)
    for c in "foo".chars() {}
    // Or we can slice the string to get particular bytes
    let s: &str = &"foo"[..1];

    // Note: the convention in the standard library is:
    // * String: Owned variant
    // * Str: Borrowed variant
    // For example, the 0sString type is owned whereas 0sStr is borrowed
}

fn tuples() {
    // A tuple is a collection of values of different types
    let t = (true, 1);
    // Same as
    let t: (bool, i32) = (true, 1);

    // Assign elements from a tuple
    let i: bool = t.0;
    let j: i32 = t.1;

    // Or with a single line
    let (i, j) = t;
}

fn control_flow() {
    // If/else
    let i = 1;
    if i < 2 {
        // true
    } else {
        // false
    }

    // If/else assignment
    let n: bool = if i == 5 { true } else { false };
}

fn loops() {
    // For (included..excluded)
    for i in 0..3 {}

    // Reverse, end excluded
    for i in (1..3).rev() {}
    // Reverse, end included
    for i in (1..=3).rev() {}

    // While
    // i has to be mutable
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
    let b: bool = loop {
        sum += i;
        i += 1;
        if i == 3 {
            break sum < 10; // Returns a boolean
        }
    };
}

fn array() {
    // Fixed size array of 5 i32 elements
    let a = [1, 2, 3, 4, 5];
    // Same as
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // Access element
    let i = a[0];
    // The following line does not compile as the array is not mutable
    // a[1] = 10;
    // In order to mutate it we have to declare it mutable
    let mut a: [i32; 5] = [1, 2, 3, 4, 5];
    a[1] = 10;

    // Initializes the 5 elements to value 0
    let a: [i32; 5] = [0; 5];

    // Get array length: 5
    let n = a.len();

    // Create slice from array
    // A slice is a pointer on an array (it is not resizable)
    let p: &[i32] = &a;
    // We cannot modify a slice regardless if the backed array is mutable or not
    // s[1] = 6;

    // Create slice from sub array (included/excluded)
    let s = &a[1..3];
    // Slice length is 2
    let n = s.len();

    // Array iteration
    let a: [&str; 3] = ["foo", "bar", "baz"];
    // Over each element index
    for i in 0..a.len() {
        let n = a[i];
    }
    // Or directly over each element
    for element in a.iter() {
        let n = element;
    }

    // Two-dimensional array initialization
    let board = [[0u32; 4]; 4];
}

fn vector() {
    // A vector is a variable length array
    // It is backed by an array, with a length and capacity

    // Initialization - 0 capacity
    // The type has to be set: as we don't insert any values, Rust doesn't know what kind of
    // elements we intend to store
    let v: Vec<i32> = Vec::new();
    // Initialization using a macro - 0 capacity
    let v: Vec<i32> = vec![];
    // Initialization with i32 values
    let v = vec![1i32, 2, 3];
    // Initialization with initial capacity and mutable
    let mut v: Vec<i32> = Vec::with_capacity(10);

    // Add element, added to index 0
    v.push(10); // [10]

    // Added to index 1
    v.push(20); // [10, 20]

    // Remove latest element
    v.pop(); // [20]

    // Get an element returns an option (see later)
    let o: Option<&i32> = v.get(0);
    // Accessing an index outside the vector will not panic, it returns a None (see later)
    let o: Option<&i32> = v.get(100);

    // We can also access an element directly using its index but in this case, accessing an index
    // outside the vector will panic
    // let o = v[100];

    // Get vector length and capacity
    let n = v.len();
    let n = v.capacity();

    // Iteration
    let v = vec![1, 2, 3];
    for i in v {
        // We can access i
        println!("i={}", i);
        // But we can't modify the values of the vector
    }
    // We can't access v anymore as the iteration took the ownership of v (see ownership())
    // println!("v={:?}", v);
    // Note that the loop was the same as
    // for i in v.iter() {}

    // If we want to keep accessing v, we need an iteration reference (i is a &i32)
    let v = vec![1, 2, 3];
    for i in &v {}
    println!("v={:?}", v);

    // Mutable iteration (is is a &mut i32)
    // Note that v has to be mutable
    let mut v = vec![1, 2, 3];
    for i in v.iter_mut() {
        // Mutate the values of the vector
        *i = *i * 2;
    }
    println!("v={:?}", v);

    // Copy a vector
    let v = vec![1i32, 2, 3].to_vec();

    let mut v = vec![1, 2, 3];
    let first = &v[0];
    v.push(4);
    // The following line doesn't compile
    // Indeed, when we push an element, the vector might require allocating new memory and copying
    // the old elements to the new space. In that case, the reference to the first element would
    // be pointing to deallocated memory.
    // println!("{}", first);
}

fn functions() {
    // Function call
    let n = increment(1);

    // Higher order function
    let f = increment;
    // Same as
    let f: fn(i32) -> i32 = increment;
    let n = f(1);

    // Partially applied function
    let partially_applied_functions = |x| multiply(3, x);
    let f = partially_applied_functions(6);
}

// A simple function example
fn increment(a: i32) -> i32 {
    // The latest expression is the value returned
    a + 1
    // Equivalent to
    // return a + 1;

    /*
    Omitting return works only if the expression is the latest statement.
    For example, the following doesn't compile:

    fn calculate_price_of_apples(apples: i32) -> i32 {
        if apples > MAX {
            apples
        }
        apples * 2
    }

    Indeed, the `apples` statement isn't the last one of the function.
    Yet, the following code compiles:

    fn calculate_price_of_apples(apples: i32) -> i32 {
        if apples > MAX {
            apples
        } else {
            apples * 2
        }
    }
     */
}

// Another simple function example
fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn structs() {
    // 3 types of structs: structs (C-like), tuple structs and unit structs
    // A structure is immutable by default

    /* Structs (C-like) */

    // We need to assign a value for each member of the structure otherwise it won't compile
    let person = CLikePerson {
        name: "foo".to_string(),
        age: 18,
    };

    // Access
    let s = person.name;
    let s = person.age;

    // Copy elements
    let s = CLikePerson {
        name: "bar".to_string(),
        ..person // Copy the rest of the elements, does not compile without it
    };

    // If we hold variables whose name are the same than the fields, we can pass them directly
    let name = String::from("foo");
    let age = 1;
    let p = CLikePerson { name, age };

    let person = CLikePerson {
        name: "foo".to_string(),
        age: 18,
    };
    // Note: Using the :? specifier inside the brackets tells println! to use the Debug output format
    // It's possible because the struct has the #[derive(Debug)] annotation
    println!("{:?}", person);
    // The :#? specifier prints each field in a new line
    println!("{:#?}", person);

    // There are three types of methods
    // 1. Calling using a &self
    // The method doesn't take ownership and can't mutate the structure
    person.method1();
    // 2. Passing a &mut self allowing the structure to be mutated within the method
    let mut mutable_person = CLikePerson {
        name: String::from("foo"),
        age: 18,
    };
    mutable_person.method2();
    // 3. Transferring the ownership of the structure (see ownership())
    person.method3();

    /* Tuple structs */

    let tuple_person = TuplePerson(String::from("foo"), 20);
    // Accessing elements
    let n = tuple_person.0;

    /* Unit-like structs */

    // A simple structure without members
    // Useful in situation where we have to implement a trait on some type
    // But we don't have any data that we want to store in the type itself
    let n = UnitStruct;
}

// C-like structure
// The naming convention for each structure type is pascal case
#[derive(Debug)]
struct CLikePerson {
    name: String,
    age: u32,
}

impl CLikePerson {
    fn method1(&self) {
        // The structure cannot be mutated so the following line will not compile
        // self.age = 20;
    }
    fn method2(&mut self) {
        // This time, the structure can be mutated
        self.age = 20;
    }
    fn method3(self) {}

    // An impl block can also contain functions called associated functions
    // Use cases: constructors or functions used only by methods of CLikePerson
    fn factory(name: String, age: u32) -> CLikePerson {
        CLikePerson { name, age }
    }
}

// Note that it's allowed to have multiple impl blocks for the same struct
impl CLikePerson {}

// Tuple structure
struct TuplePerson(String, u32);

// Unit structure
struct UnitStruct;

fn enums() {
    // Simple enum (based on units)
    let e = Enum::Foo;
    let e = Enum::Bar;

    // Enum with variants
    let e = EnumWithVariants::Foo { id: 1, age: 1 };
    let e = EnumWithVariants::Bar(String::from("foo"));
    let e = EnumWithVariants::Baz;
}

enum Enum {
    Foo,
    Bar,
}

enum EnumWithVariants {
    Foo { id: i32, age: i32 }, // Struct variant
    Bar(String),               // Tuple variant
    Baz,                       // Unit
}

fn closures() {
    // Three types of closure
    // FnOnce: consume the variables it captures (all closures implement FnOnce as they can all be called at least once)
    // FnMut: mutable borrow
    // Fn: immutable borrow

    // Closure example
    let c = |x: i32, y: i32| -> i32 { x + y };

    // To force a closure to take ownership of the values we can use move keyword
    let c = move |x: Point| -> Point { x };

    // Direct closure call
    let c: i32 = { 1 + 2 };

    // A struct can hold a closure
    let s = StructWithClosure { f: |x| x + 1 };
    let n = (s.f)(1);

    // A structure holding a closure with generics (see later)
    let s = StructWithClosureAndGenerics { f: |x| x + 1 };
    let n = (s.f)(1);
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

fn hashmap() {
    // Hashmap creation
    let m: HashMap<String, i32> = HashMap::new();
    // With initial capacity
    let mut map: HashMap<String, i32> = HashMap::with_capacity(32);

    // Note: a map stores its data on the heap

    // Insert elements
    let s = String::from("one");
    let i = 1;
    map.insert(s, i);
    // If a type implements the Copy trait (e.g. i32), the values are copied into the hash map
    // Hence it is valid to reuse i
    let n = i;
    // Otherwise, the values are moved
    // Hence, as it is invalid to reuse s, the following line wille not compile
    // let s2 = s;

    // Insert if the key does not already exist
    let v: &mut i32 = map.entry(String::from("two")).or_insert(2);
    // We can also mutate the entry value directly
    *v = 20;

    // Iteration
    // key is an &String whereas value is an &i32
    for (key, value) in &map {}

    // Create a hashmap from two vectors
    let v1 = vec![
        String::from("one"),
        String::from("two"),
        String::from("three"),
    ];
    let v2 = vec![1, 2, 3];
    // The <_, _> notation is needed as it's possible to collect into many different data structures
    // So Rust doesn't know which one we want unless it's specified
    let m: HashMap<_, _> = v1.iter().zip(v2.iter()).collect();
}

fn impls_traits() {
    // An impl is used to define method for structs and enums
    let p = Point { x: 1, y: 1 }.foo(Point { x: 2, y: 2 });

    // An associated (static) function call
    let point = Point::new(1, 1);

    // A trait can be seen as a contract, it's similar to interfaces in other
    // languages, with some differences

    // A trait can be a parameter
    function_accepting_trait(Point::new(1, 1));
    // A syntax variant called *trait bound* syntax
    // The previous variant is a syntax sugar
    function_accepting_trait_variant(Point::new(1, 1));
    // The trait bound syntax is useful for specific use cases. For example, if we want to
    // enforce two parameters to have the same type.
    function_accepting_two_parameters_of_the_same_type(Point::new(1, 1), Point::new(1, 1));

    // A function can also return a trait
    let t = function_returning_trait();
    // Note: it's only possible if the function returns a single type
    // For example, if `function_returning_trait` was returning two possible types implementing
    // `Trait`, the function wouldn't compile

    // Point also implements the generic trait TraitWithGenerics
    let p = Point::convert_to_tuple(Point { x: 1, y: 1 });

    // Inheritance example
    // Ferrari implementing the Vehicle and Car traits
    let f = Ferrari {
        id: String::from("001"),
        color: String::from("red"),
    };

    // Note: one restriction with trait implementation is that we can implement a trait on a type
    // only if:
    // * Either the trait is local to our crate
    // * Or if the type is local to our crate
    // We can't implement external traits on external types
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

fn function_accepting_trait(a: impl Trait) {
    a.default_method();
}

fn function_accepting_trait_variant<T: Trait>(a: T) {
    a.default_method();
}

fn function_accepting_two_parameters_of_the_same_type<T: Trait>(a: T, b: T) {
    a.default_method();
    b.default_method();
}

fn function_returning_trait() -> impl Trait {
    Point { x: 1, y: 1 }
}

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

fn generics() {
    // Generics in Rust do not have any performance impact
    // Rust accomplishes this using monomorphization
    // It is the process of turning generic code into specific code at compile time

    // Generic function call
    // The function accepts only structures implementing Trait
    let p = generic_function(Point { x: 1, y: 1 }, Point { x: 1, y: 1 });
    // This is different that the following function that accepts Trait implementation
    let p = function_accepting_implementation(Point { x: 1, y: 1 }, Point { x: 1, y: 1 });
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
    let p = s.generic_function();

    // A specific implementation for GenericStruct<i32> only
    let i = s.specific_function();
    let s = GenericStruct { x: "a", y: "b" };
    // Doesn't compile
    // s.specific_function();

    // Instantiate a generic enum
    let p = GenericEnum::<i32, String>::Foo(1);
    let p = GenericEnum::<i32, String>::Bar("foo".to_string());
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

fn pattern_matching() {
    // Pattern matching on an integer
    let i = 1;
    // The match has to cover all the cases
    // The following example does not compile if we omit the last case
    let n = match i {
        0 => "zero",
        1 => "one",
        _ => "other", // Other cases
    };

    // If we want to do nothing
    let n = match i {
        0 => println!("zero"),
        _ => (), // () is the unit value, so nothing will happen in this case
    };

    // Comparing integers
    // It uses std::cmp::Ordering
    let i = 1;
    let j = 2;
    let n = match i.cmp(&j) {
        Ordering::Less => "less",
        Ordering::Greater => "greater",
        Ordering::Equal => "equals",
    };

    // Matching an interval
    let level = 22;
    let n = match level {
        1..=5 => "beginner",
        6..=10 => "intermediate",
        11..=20 => "expert",
        _ => "other",
    };

    // Pattern matching on a tuple
    let i = 1;
    let j = 1;
    let n = match (i, j) {
        (1, 2) => "one, two",
        // Default values on a single element
        (1, _) => "1, default", // This is the one that will be matched
        // Default values on both elements
        (_, _) => "default, default",
    };

    // Enum matching
    let e = EnumWithVariants::Bar("foo".to_string());
    match e {
        // We have to use all possible enum values or use a default case
        EnumWithVariants::Foo { id, age } => println!("id={}, age={}", id, age),
        EnumWithVariants::Bar(foo) => println!("element={}", foo),
        _ => println!("else"), // Baz variant
    }
}

fn if_let() {
    // If we need a match that runs code when the value matches one pattern only, we can use if let
    let e = EnumWithVariants::Foo { age: 1, id: 3 };
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
    let s = CLikePerson {
        name: "foo".to_string(),
        age: 1,
    };
    println!("{:?}", s);
}

fn option() {
    // Rust does not have nulls
    // Instead, it has an Option enum that can encode the concept of a value being present or absent
    // Option means the possibility of absence:
    /*
    enum Option<T> {
        Some(T),
        None,
    }
     */

    let option: Option<String> = option_example(0);
    // We use pattern matching to check an option
    match option {
        None => println!("none"),
        Some(i) => println!("{}", i),
    }

    // We can also map directly a result using expect
    // The code will panic if result contains an error
    let option = option_example(0);
    let s: String = option.expect("foo");

    // We can use unwrap to get the value
    // Yet, it panics if the option is None
    let option = option_example(0);
    let s: String = option.unwrap();

    // unwrap_or_else takes a closure to return a default value
    let s: String = option_example(0).unwrap_or_else(|| String::from("foo"));
}

fn option_example(i: i32) -> Option<String> {
    if i == 0 {
        return Some("zero".to_string());
    }
    None
}

fn result() -> Result<i32, io::Error> {
    // Result means the possibility of an error
    // Compared to panic, it signals a recoverable error
    /*
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
     */

    let result = div(1, 1);

    // We can use pattern matching to check a result
    match result {
        Err(e) => println!("error: {}", e),
        Ok(i) => println!("value: {}", i),
    }

    // If the function returns a Result as well, we can use the ? operator
    // If the value of the Result is an Ok, it is returned from this expression
    // Otherwise, the Err will be returned from the surrounding function
    let _ = File::open("foo")?;

    // We can also map directly a result using expect
    // The code will panic if result contains an error
    let i = result.expect("failed to apply division");
    // Panic if the result contains an error
    let i = result.unwrap();
    // Returns a default value in case of an error
    let i = div(1, 0).unwrap_or_else(|error| {
        println!("error: {}, defaulting to 0", error);
        0
    });

    // A function calling another function that returns a result can use the ? operator
    let result = result_caller(1, 0);

    Ok(0) // Just to comply with the function signature
}

fn div(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 {
        return Err("b is nil");
    }
    Ok(a / b)
}

fn result_caller(a: i32, b: i32) -> Result<i32, &'static str> {
    // If result_example returns an error, we automatically returns the error
    let i = div(a, b)?;
    Ok(i)
}

fn ownership() {
    // Ownership rules:
    // * Each value has a variable that is called the owner
    // * Only one owner at a time
    // * When owner goes out of scope (its range), the value is dropped

    // When a variable goes out of scope, the memory is automatically returned
    {
        let _i = 1;
    } // i being out of scope at this stage, it is freed

    let i = String::from("hello");
    let j = i;
    // The following line doesn't compile as we transferred the ownership to j
    // println!("{}", i);
    // We say that i was moved into j

    // Yet, in this example the data is a primitive
    // Hence, the value is copied, not transferred to j
    // Both are owners of an i32 value
    let i = 1;
    let j = i;
    // Hence, this line compiles
    println!("{}", i);

    // A copy is used if the value:
    // * Is a primitive
    // * Is a tuple or an array of primitives

    // Example of an array of primitives copy
    let a = [1, 2, 3];
    let a2 = a;
    // Hence, it is valid to access both arrays at this stage
    println!("v={:?}, v2={:?}", a, a2);

    // The semantics for passing a value to a function is similar to variable assignment
    let s = String::from("foo");
    takes_ownership(s);
    // At this stage, we cannot use s anymore and we call this moving a variable
    // Returning a value from a function does also transfer ownership

    // Having a method that takes ownership of the instance by using just self is rare
    // It's usually used when the method transforms self into something else
    // Hence, we want to prevent the caller from using the original instance
    Foo {}.bar();

    // As a summary, an assignment or passing/returning a variable to/from a function is either a move or a copy
}

fn takes_ownership(_: String) {}

struct Foo {}

impl Foo {
    fn bar(self) {}
}

fn copy() {
    let i = String::from("hello");
    let j = i.clone();
    // Note: doing j = i means the String data is copied (pointer, length, and capacity)
    // These elements are stored on the stack, we don't copy the data on the heap that the pointer refers to

    // Regarding copies, Rust never automatically creates deep copies of the data
    // If we do want to deeply copy data of a string, for example, not just the stack data, we have to use the clone method
    let i = String::from("hello");
    let j = i.clone();

    // Types that implement the Copy trait:
    // * Primitives
    // * Tuple of Copy types

    // TODO: Copy trait
}

fn borrowing() {
    // To reuse a resource, we have to borrow it
    // Borrowing is about passing variable bindings either:
    // * To other functions
    // * Or to other variable bindings

    // Two types of borrowing:
    // * Shared: data can be borrowed by multiple users but must not be altered
    // * Mutable: data can be borrowed and altered by a single user

    // Benefit of these restrictions: data race analysis done by the compiler

    // Borrowing applies for both copies and moves

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
    // println!("v1={:?}, v2={:?}", v1, v2);
    // Here, only v2 should read and access the vector
    v2[0] = 10;
    let i = v2[0];

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
    let s1 = &s; // First immutable reference
    let s2 = &s; // Second immutable reference

    // Does not compile
    // let s3 = &mut s;

    // Rules summary:
    // * At any time, you can have either but not both:
    //      * One mutable reference
    //      * Any number of immutable references
    // * References must always be valid
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
    let s1;
    {
        let s2 = String::from("foo");
        s1 = &s2;
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
    let n = longest_with_lifetime("foo", "bar");

    // In this example, the function returns the input reference
    // Hence, the compiler does not need to know what's the lifetime
    // We could have omitted the lifetime annotation
    let s1 = "foo";
    let s2 = function_with_useless_lifetime(s1);

    // Returning a static reference
    let s = lifetime_function_static();

    // Lifetime on a structure
    let x = LifetimeStruct { s: "foo" };
    println!("{}", x.s);

    // All string literals have the 'static lifetime
    // The text of this string is stored directly in the binary of the program
    let s: &'static str = "I have a static lifetime.";

    // Generics and lifetimes
    let p = generics_with_lifetimes(&Point { x: 1, y: 1 });
}

// Inputs and output share the same lifetime
fn longest_with_lifetime<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn function_with_useless_lifetime<'a>(s: &'a str) -> &'a str {
    s
}

// Static means a the same life time than the whole application
fn lifetime_function_static() -> &'static str {
    "foo"
}

struct LifetimeStruct<'a> {
    // s should live as long as LifetimeStruct exists
    s: &'a str,
}

fn generics_with_lifetimes<'a, T>(x: &'a T) -> &'a T
where
    T: Trait,
{
    x
}

fn panic() {
    // A panic is an unrecoverable error
    // It's specific to a thread, not to an application

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
    let s = match level {
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
    let o: Option<i32> = (1..).filter(|i| is_prime_number(*i)).nth(n);

    // An example producing a vector or i32 by mapping each element and collecting the results
    // We iterate from i1 to i2 (included because of =i2) reversely
    let i1 = 0;
    let i2 = 10;
    // increment being a i32->i32 function
    let v: Vec<i32> = (i1..=i2).rev().map(increment).collect::<Vec<_>>();

    // Chaining example with a single element
    // It produces: 1,2,3,4
    let s: String = (1..3)
        .map(|x| x.to_string())
        .chain(std::iter::once("4".to_string()))
        .collect::<Vec<_>>()
        .join(",");

    // Custom iterator
    let mut custom_iterator = CustomIterator::new(vec![1, 2, 3]);
    let o = custom_iterator.next(); // 1
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

fn smart_pointers() {
    // A smart pointer is a data structure that not only act like a pointer
    // but also have additional metadata and capabilities

    // Distinction between smart pointer and ordinary struct:
    // Smart pointers implement the following traits:
    // * Deref: allows an instance of the smart pointer struct to behave like a reference
    // * Drop: allows to customize the code that is run when an instance of the smart pointer goes out of scope
}

fn file() -> Result<(), Box<dyn std::error::Error>> {
    // Iterate over a file (input: string literal)
    include_str!("../.gitignore")
        .lines()
        .for_each(|x| println!("{}", x));

    // Iterate over a file
    let reader = BufReader::new(File::open("../.gitignore")?);
    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}

fn sort() {
    let mut input = vec![1, 3, 2];
    // Sort input (mutates input directly)
    input.sort();
    // Sort in the decreasing order
    input.sort_by(|a, b| b.cmp(a));
}

fn testing() {
    // The best practice is to keep unit tests in a tests module in the end of the file
    // Integration tests should live in /tests folder, next to /src

    // To avoid running tests in parallel: $cargo test -- --test-threads=1
}

fn sum(i: i32, j: i32) -> i32 {
    i + j
}

#[cfg(test)]
mod tests {
    // Import the parent module
    use super::*;

    #[test]
    // We can use #[ignore] to ignore a test
    fn test_sum() {
        assert_eq!(sum(2, 2), 4);
    }
}
