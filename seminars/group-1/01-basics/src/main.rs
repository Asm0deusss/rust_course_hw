#[allow(unused_variables)] // removes warnings about unused variables

use std::convert::TryInto; // in order to use `.try_into()` function


fn integer_types() -> () {
    // https://doc.rust-lang.org/std/primitive.i32.html

    let x = 4;                  // default type is i32;
    let x: i64 = 4;             // enforsing type to be i64;
    let x = 4i64;               // another way to set type of x to `i64`

    /* ------ */
    let x: i32 = 10;            // variable`s shadowing,
                                // i.e. declaring variable again
                                // inside the same scope
    /* ------ */
    let y: i64 = -1;
    let x: i32 = y as i32;      // compiles ok
    let x: i32 = y.try_into()
                    .unwrap();  // compiles ok
    /* ------ */
    
    let x: i32 = y;             // a compile error occurs here, because you cannot assign i64 to i32
    let x = y;                  // since `y` is `i64`, then `x` is also an `i64` variable
    
    /* ------ */
    let y: i64 = 5_000_000_000; // too big to be an `i32`, hence using an `i64` type.
    let x: i32 = y.try_into()
                    .unwrap();  // compiles ok, though a runtime error occurs here
    
    /* ------ */
    let x: u8 = 500;            // compile error
    let x: u8 = -1;             // compile error
}


fn float_types() -> () {
    // https://doc.rust-lang.org/std/primitive.f64.html
    // https://doc.rust-lang.org/stable/reference/expressions/operator-expr.html#type-cast-expressions


    let x = 10;                 // since the right-hand side is an integer literal
                                // x has 'i32' type

    let x = 10.0;               // now the right side is a float literal,
                                // so by default x has 'f64' type
    let x: f32 = 10.0;          //  or `let x = 10.0f32;`

    /* ------ */

    let x: f64 = 10;            // compile error, you need to provide a float literal
    let x = 10.0 / 2;           // compile error, since you cannot divide a float by an integer
    let x = 10.0 / 2.0;         // works just fine (again, the result type by default is 'f64')

    /* ------ */
    let x = 10.0;
    let y: u8 = x as u8;        // compiles alright
    let x: f32 = 10.5;
    let y = x.floor() as u8;    // compiles alright
                                // but you need to explicitly specify type of x before
                                // because of side-effects of using `floor` function.
}


fn tuple_types() {
    // https://doc.rust-lang.org/rust-by-example/primitives/tuples.html

    let (x, y, z) = ('a', 123, 1.0f32);    // assigning each variable simultaniously;
    let x = ('a', 123, 10.0);           // x has type (&str, i32, f64)
    
    assert_eq!(x.0, 'a');               // using `.0` to address first element in tuple
    assert_eq!(x.1, y);

    if z as f64 == x.2 {                // cannot compare `z` and `x.2` since they have
                                        // different types (f32 and f64)
        println!("Comparison returned true");
    }
}

fn array_types() {
    // https://doc.rust-lang.org/stable/std/primitive.array.html

    let arr = [1, 2, 3, 4, 5];          // has type of [i32; 5]
    for elem in &arr {                  // do not forget this `&`!!!
        print!("{},", elem);            // prints the elements of the array
    }
    println!("{:?}", arr);              // prints the array

    assert_eq!(arr[0], 1);

    let arr: [i64; 2] = [10, 20];       // enforsing types to be `i64`
    assert_eq!(arr[0], 10i64);

    let arr = [10u8; 2];                // defines an array two `u8` elements
    assert_eq!(arr[0], arr[1]);         // they both are equal to 10
}

fn vector_types() {
    // https://doc.rust-lang.org/stable/std/vec/struct.Vec.html

    let vec = vec![1, 2, 3];            // using macro to define a vector;
    let vec = vec![3u8; 3];             // defining a vector of 3 elements, each of them equals to 3
    
    vec.push(4);                        // a compile error, because of immutablility of `vec`

    let mut vec = vec![1, 2, 3];
    vec.push(4);                        // ok

    let vec: Vec<i64> = vec![1, 2, 3];  // the way to set the result type of vector`s elements
}

fn empty_type() -> () {
    /* 
        If some function returns nothing
        then in Rust you can denote it as `fn func_name() -> () { ... code ... }`,
        where `-> ()` means `returns nothing`.

        We usually do not write this and simply put it like this
        `fn func_name() { ... code ... }`
    */
    let x = ();                         // what is unusual - we can declare `nothing`
    let y: () = {
        let x = 10;                     // scopes, that end with `;` also `return nothing
    };                                  // hence the type of `y` is also a `()`

    let z: () = {
        let x = 10;
        y                               // the last expression has type `()` and there is no `;` in the end
    };                                  // so the returned value of this scope is `()`
}


fn scopes() {
    // https://doc.rust-lang.org/rust-by-example/scope.html (it is hard to read)

    // In Rust scopes are declared by `{` and `}`.
    // All variables only live in the scope they are declared in.

    let x = 5;                          // this variable lives till the end of function `scopes`
    {
        let y = 5;                      // this variable lives till the end of this scope
    }

    println!("{y}");                    // you cannot access `y` here
    let y = 5;                          // but you can declare it again
    println!("{y}");                    // so it works just fine
    
    // The scope can return a value, if you leave last line without `;`
    // The result value of the scope will be this last line.
    // Otherwise the scope return `()`.

    let x: f32 = {
        let y = 5f32;                      // declaring y
        y                                  // returning y
    };

    let x: () = {
        let y = 5f32;                      // declaring y
        y;                                 // the `;` in the end, hence no result
    };

    fn some_func() -> i32 { 5 }            // the function returns `i32`
                                            // because the scope does not end with `;`
    fn some_func2() { 5; }                 // the functino return `()`
                                            // because the scope ends with `;`

    let x: i32 = some_func();
    assert_eq!(x, 5);
    let x: () = { let y = some_func(); };
    assert_eq!(x, some_func2());             // `()` equals to `()`
}

/*
    Задача: Даны имя файла и интервал. Нужно вывести все строки файла,
            номера которых лежат в заданном интервале.
*/
use std::ops::Index;            // for `.index()`
fn example_problem() {
    let args = vec!["20-21", "./bible.txt"];        // hardcoded arguments
    let (range, file_path) = (args.index(0), args.index(1));

    let (first_line_num, last_line_num) = range
                                        .split_once('-')    // split string
                                        .unwrap();          // unwrap result to the tuple
    let (first_line_num, last_line_num): (i32, i32) = (
        first_line_num.parse().unwrap(),                    // converts string to `i32` 
        last_line_num.parse().unwrap()                      // converts string to `i32`
    );

    let content = std::fs::read_to_string(file_path).unwrap(); // reading the file content
    
    for (counter, line) in content.lines().enumerate() {     // iterating over lines
        let idx = (counter + 1) as i32;                      // moving from 0-based index to 1-based index
        if first_line_num <= idx && idx <= last_line_num {   // checking if index is in interval
            println!("{} and {}", line, idx);                // printing the line and the index
        }
    }
}

fn main() {
    {
        integer_types();
        float_types();
        tuple_types();
        array_types();
        vector_types();
        empty_type();
        scopes();
    }

    example_problem();
}
