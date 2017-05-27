fn main() {
  //data_types()
  //func()
  ctl_flow();
}
/* Shadowing is different than marking a var as mut, because unless
 * we use the let keyword again, we'll get a compile time error if
 * we accidentally try to reassign to this variable. We can perform
 * a few transformations on a value but have the variable be immutable
 * after those transformations have been completed.
 *
 * Other difference between mut and shadowing is that because we're
 * effectively creating a new variable when we use the let keyword
 * again, we can change the type of the value *and* reuse the same name.
 *
 * let spaces = "    ";
 * let spaces = spaces.len();
 *
 * lets us use spaces for both instead of spaces_str and spaces_num.
 */

////////////////////////////////////////////////////////////////////////////////
// Data types
////////////////////////////////////////////////////////////////////////////////
// Every value in rust is of a certain *type*, telling Rust what kind of
// data is being specified so it knows how to work with that data.
// Two distinct sets of types: scalar and compound.
//
// Rust is **statically typed**, meaning it must know the types of all
// variables at compile time. types can usually be inferred based on
// the value and how we use it. When they're ambiguous, types can
// be annotated.
//
// let guess: u32 = "42".parse().expect("Not a number!");
//
// [Scalar Types]
// These types represent a single value. There are four primary ones,
// integers, floating-point nums, booleans, and characters.
// Integers: primary is usually fine, i32. isize/usize are primarily
// used indexing some sort of collection
//
// Default fpoint is f64 because it's roughly the same speed as f32 and
// offers more precision.
//
// Coolean is just `bool`. Two vals, true and false.
//
// char type is the most primitize alphabetic type.
// It's a unicode scalar value, meaning it can represent a lot more
// than just ascii.
//
// [Compound Types]
// These can group multiple values of other types into one type. Two
// primitive compound types are available: tuples and arrays.
//
// + Tuples
// General way of grouping together a number of other values with a
// variety of types into one compound type.
// Comma separated list of values inside paren.
//
// Pattern matching is used to destructure a tuple value see tuple.
// See tup.
// + Arrays, see arr()
//
// +
////////////////////////////////////////////////////////////////////////////////
fn data_types() {
  println!("Hello data types.");
  //tup();
  arr();
}

fn tup() {
  let tup = (500, 6.4, true);
  let (_, num, _) = tup;
  println!("Tuple time. Value of num is -> {}", num);

  // Tuples can also be indexed by the dot operator followed by the idx
  let last_val = tup.2;
  println!("Last value -> {}", last_val);
}

fn arr() {
  // Every element in an array must have the same datatype.
  // Arrays also have a fixed size. Once declared, they cannot grow or
  // shrink in size.
  let a = [1, 2, 3 ,4, 5];

  // They are useful when you want your data allocated on the stack
  // and not the heap, or when you want to ensure you always have a
  // fixed number of elements. They're not as flexible as a vector.
  // The vecors are dynamic and are allowed to grow and shrink in size.
  //
  // A good example for when to use an array is say, the months of the
  // year. That size will never change.
  //
  // As expected, values are extracted with the familiar idx op [idx]
  //
  // If an out of bounds access happens, a runtime error will occur.
  // If the element is greater than the length of the array, Rust
  // will *panic*, a term used when rust program exits in error.

  // It's a first example of rust's safety principles at work.
  // Most low langs do not perform this check, and when given a bad idx,
  // invalid memory can be accessed. Rust protects you against this
  // by immediately exiting instead of allowing the access of the mem.
}

////////////////////////////////////////////////////////////////////////////////
// Functions
////////////////////////////////////////////////////////////////////////////////
// Rust code uses *snake case* as a conventional style for function
// and variable names.
// Rust doesn't care where you define your functions as long as they're
// defined somewhere.
//
// Functions can accept parameters. The types of params *must* be
// declared; it's a deliberate decision made in Rust's design.
// Requiring this means the compiler almost never needs you to use them
// elsewhere in the code to figure out what you mean.
//
// Bodies are made up of a series of statements optionally ending in an
// expression. We've also seen expressions as parts of statements.
// Rust is an expression-based lang, it's important to understand the
// difference because other langs don't have the same distinction.
//
// + Statements and Expressions
// -> Statements are instructions that perform some action and do not
//    reeturn a value.
//  -> Expressions evaluate to a resulting value.
//
//  let y = 6; is a statement.
//
//  Function definitions are also statements, the entire definition is
//  a statement itself. Statements to not return values, Therefore, you
//  cannot assign the result of a *let* statement to another var.
//
//  This is different from other languages like C and Ruby, who allow
//  multiple assignment. x = y = 6.
//
//  Statements contain expressions. let y = 6;, 6 itself is an expression
//  that evaluates to 6. A function call is an expression.
//
//  The block that is used to create new scopes, {}, is an expression too!
//  See block_eval()
//
//  fns can return vals to the code that calls them. return vals are not
//  named, but the type is declared after an arrow.
//
//  fn duder() -> i32 { 5 }
//
//  The return value of the function is synonymous with the value of the
//  final expression in the block of the body of a function.
//
//  Note if a value is not returned, we'll see that the found type
//  is in fact an empty tuple, (). Nothing has been returned,
//  contradicting the function definition that expects a result.
////////////////////////////////////////////////////////////////////////////////

fn func() {
  block_eval()
}


fn block_eval() {
  let x = 5;

  // This is a block that evaluates to 4. Note the line without
  // the semicolon. Unlike most lines seen so fr, expressions do not
  // include ending semicolons. If you add a semicolon to the end of an
  // expression, its turned into a statement, which will not return a val.
  let y = {
    let x = 3;
    x + 1
  };

  println!("The value of (x, y) -> ({}, {})", x, y)
}

////////////////////////////////////////////////////////////////////////////////
// Control Flow
////////////////////////////////////////////////////////////////////////////////
// Deciding when to run code based on a condition of truth is a
// fundamental building block of programming. Most common ifs and loops.
//
// + if
// start with keyword followed by expression. The blocks associated with
// conditions are sometimes called arms, just like the arms of a match
// expression.
//
// Also worth noting the condition of an if *must* be a bool, you can't
// use something like "7". Rust will not automatically convert non-bool
// types to a boolean.
//
// + `else if` is available.
// Note: if you have more than one else if, consider using a more powerful
// match branching construct.
//
// + Using if in a let statement, see let_if()
//
// + Loops
// 3 different types of loops, loop, while, and for.
// see loops()
////////////////////////////////////////////////////////////////////////////////
fn ctl_flow() {
  //let_if()
  loops()
}

fn let_if() {
  // Remember blocks of code evaluate to the last expression in them, and
  // numbers by themselves are also expressions. Therefore, the value
  // of a whole if expression depends on which block of code executes.
  // It also means the values that have the potential to be results from
  // each if arm must be of the same type.
  // Rust must know the type of `number` here definitively at compile
  // time so it can verify the type is valid everywhere `number` is used.
  // It wouldn't be able to do that if the type of the variable `number`
  // was determed by runtime code.
  let cond = false;
  let number = if cond {5} else {6};
  println!("let_if cond -> {}", number);
}

fn loops() {
  // loop keyword tells rust to execute a block until you tell it to
  // stop.
  // loop {
  //   println!("derpyfoobar");
  // }
  //
  // break can be used to break out of a loop.
  //
  // + conditional loops with while
  //
  // while [condition] {}
  //
  // + Looping collections with for
  // Looping through things with while and indexes is error prone.
  // A better alternative is the for loop, used to execute some code
  // for each item in a collection:

  let a = [10, 20, 30, 40, 50];
  for item in a.iter() {
    println!("the value of the thing is {}", item);
  }

  // The important thing here is we've eliminated the possiblity
  // that we'll run passed the end of the array.
  //
  // The safety and conciseness of for loops make them the most
  // commonly used loop construct in Rust. Even in code you want
  // to run a certain number of times, most people would use
  // a for loop with a Range, a type provided by the stdlib that
  // generates all numbers in seq starting from one and ending before
  // antoher.

  println!("Using range.");

  for num in (1..4).rev() {
    println!("{}!", num);
  }
  println!("GO");
}
