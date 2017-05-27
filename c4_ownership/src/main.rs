fn main() {
  //scope()
  //moves_and_mem();
  //refs()
  slices()
}

////////////////////////////////////////////////////////////////////////////////
// What is Ownership?
////////////////////////////////////////////////////////////////////////////////
// Ownership is Rust's central feature.
// All programs have to manage the way they use a computer's memory while
// running. Some have garbage collection that constantly looks for no
// longer used memory as the program runs; in others, the programmer must
// explicitly allocate and release the memory. Rust uses a third approach:
//
// Memory is managed through a system of ownership with a set of rules
// that the compiler checks at compile time. No run-time costs are
// incurred for any of the ownership features.
//
// [Stack and Heap]
// In most langs, you really don't need to think about the stack and
// the heap often. In systems langs, whether the value is on the stack
// or on the heap has more of an effect on how the language behaves and
// why we have to make certain decisions.
//
// The stack and the heap are parts of memory that are available to your
// code to use at runtime, but they are structured differently. The stack
// stores values in the order it gets them and removes the values in
// the opposite order (LIFO). This is referred to as
// *pushing onto the stack* and *popping off of the stack*
//
// It's fast because of the way it accesses the data: it never has to
// search for a place to put new data or a place to get data from because
// that place is *always* the top of the stack. Another propery is that
// all data on the stack must take up a known, fixed size.
//
// For data that is an unknown size at compile time or a size that may
// changeo ver time, we can store that data on the heap instead. The heap
// is less organized; we just ask for some amount of space. The OS
// finds an empty spot somewhere that's big enough for the request, marks
// it as in use, and returns a pointer to that location. It's called
// *allocating on the heap*. Pushing onto the stack is not considered
// allocation. A pointer is a known, fixed size, so it can sit on the
// heap, but for actual data, we have to follow the pointer.
//
// The heap is slower than the stack because we have to follow a pointer
// to get there (a level of indirection). Processors are faster due to
// temporal and spacial locality and caching if they have to jump around
// less.
//
// When a function is called, the values passed into the function
// (including, potentially pointers to data on the heap) and the fns
// local vars get pushed onto the stack. When its over, the vals get
// popped off the stack.
//
// !!
// Keeping track of what code is using what data on the heap, minimizing
// duplicate data on the heap, and cleaning up unused data on the heap
// so we don't run out of space are all problems that ownership helps.
// Once ownership is understood, you won't have to think about the stack
// and the heap often, but knowing that managing heap data is why
// ownership exists can help explain why it works the way that it does.
// !!
//
// [Ownership Rules]
// There exist 3 very important rules to ownership in Rust:
//
// 1) Each value in Rust has a variable that's called its *owner*
// 2) There can only be one owner at a time (the highlander rule)
// 3) When the owner goes out of scope, the value will be dropped
//
// + Variable Scope
// See scope()
//
// + Memory and Allocation
// See moves_and_mem
//
// + References
// See refs()

fn scope() {
  // First example of ownership, we'll look at the *scope* of some
  // variables. Scope is the range within a program for which an item
  // is valid.
  // s is a string literal. the value of the string is hardcoded into
  // the text of the program. The variable is valid from the point
  // at which it's declared until the end of the current *scope*.

  { // s is not valid here, it's not yet declared
    let s = "hello"; // s is valid from this point forwards

    // do some stuff with s
  } // this scope is now over, and s is no longer valid

  // There are two important points in time here:
  // 1) When s comes *into* scope, it is valid.
  // 2) s remains valid until it is *out of scope*
  //
  // // The relationship between scopes and when variables are valid
  // is similar to other programming langs. Let's build on top
  // of this introducing the String type.
  //
  // + String type
  // We're going to illustrate the rules of ownership using a data type
  // that's more complex than the ones we've seen before. All the data
  // types we've seen before are stored on the stack and popped off the
  // stack when their scope is over, but we want to look at data
  // that's on the heap and explore how Rust knows to clean that up.
  //
  // We'll concentrate on the parts of String that relate to ownership.
  // They also apply to other complex data types provided by the
  // stdlib and those that you create.
  //
  // We've seen string literals hardcoded into the program. They're
  // convenient, but they aren't suitable for every situation in which
  // you want to use text. For one reason, they're immutable. Also, not
  // every string value is known when we write our code. The other type
  // is a String, which is allocated on the heap. It's able to store an
  // amount of text that is unknown at compile time. It's created from
  // a literal with a `from` function:
  let s = String::from("hello");

  // Again, double colon (::) is an op that allows us to namespace
  // this from function under the String type rather than using a name
  // like string_from. It can be mutated:
  let mut s = String::from("hello");
  s.push_str(", world!"); // appends a literal to a String
  println!("{}", s); // Will print the full string.

  // Why can Strings be mutated but literals cannot? Difference is
  // how they deal with memory.
}

fn moves_and_mem() {
  // With string literals, we know the contents of the string at compile
  // time, so the text is literally hardcoded into the executable,
  // making them extremely fast and efficient. This property only comes
  // from its immutability. We can't put a blob of memory into the binary
  // for each piece of text whose size is unknown at compile time and
  // whose size might change while running the program.
  //
  // To support a mutable, growing piece of text, need to allocate an
  // amount of mem on the heap, unknown at compile time, to hold the
  // contents. This means:
  //
  // 1) The memory must be requested from the OS at runtime.
  // 2) Need a way of returning the mem to the OS when we're done with
  //    the allocated string.
  //
  // First part is done by us: the String::from implementation requests
  // the memory it needs from the OS. This is pretty standard for most
  // langs.
  //
  // The second part is different. In langs with GCs, it will keep track
  // and clean up mem that isn't used anymore, and the programmer doesn't
  // need to think about it. Without a GC, it's the programmer's
  // responsibility to know when that memory is no longer being used
  // and call code to explicitly return it.
  //
  // This has historically been a *very* difficult problem to solve.
  // If you forget to, we'll waste memory and leak it.
  // If we do it too early, we'll have an invalid variable (use after free)
  // If we do it twice, that's a bug too.
  //
  // We need to pair exactly one allocation with one free.
  //
  // Rust takes its own unique path: the memory is automatically
  // returned once the variable that owns it goes out of scope.
  // When a variable goes out of scope, Rust calls a special function
  // for us. The function is called drop, and it's where the author
  // of String can put the code to return the memory. Rust calls
  // `drop` automatically at the closing }.
  //
  // NOTE: C++ calls this pattern of deallocation at the end of its
  // lifetime RAII. The drop function in Rust is similar to a dtor
  //
  // The pattern has a profound impact on the way that Rust code is
  // written. Might seem simple, but the behavior of code can be
  // unexpected in more complicated situations when we want to
  // have multiple variables use the data that's been allocated
  // on the heap.
  //
  // + Ways variables and data interact: Move
  // Multiple variables can interact with the same data in different
  // ways in rust:
  // let x = 5;
  // let y = x;
  //
  // So here, we bind the value of 5 to x, then we make a copy
  // of the value in x and bind it to y.
  // We now have to vars x and y and both equal 5.
  // This is exactly what's happening because integers are simple
  // values with a known, fixed size, and these two 5 vals are
  // pushed onto the stack.
  //
  // let a = String::from("hello);
  // let b = a;
  //
  // This looks similar and you'd probably assume it behaves the same
  // way; the second would make a copy of the val in a and bind it to b.
  // This is not what happens.
  //
  // Under the covers, a String is actually a type with a few values:
  // ptr to some memory holding the string, a length, and a capacity.
  // This group is stored on the stack. The length is how much memory
  // in bytes the contents of the String is curreently using.
  // The capacity is the total amount of memory, in bytes, the String
  // has received from the OS. Difference between len and cap matters,
  // but not the point of this.
  //
  // When a is assigned to b, the String data is copied, meaning we copy
  // the pointer, the len, and the cap on the stack. The heap data is
  // not copied, so b's pointer is going to refer to the same heap
  // data that a does.
  //
  // Earlier we said when a variable goes out of scope, Rust will
  // automatically call the drop function and clean up the heap mem.
  // But in this case, both pointers are pointing to the same heap
  // memory. Thiis is a problem. When a and b go out of scope, they
  // will both attempt to free the same memory. This is a *double free*
  // error and is one of the memory safety bugs we mentioned previously.
  // Freeing mem twice can lead to mem corruption, which can lead
  // to security vulnerabilities.
  //
  // To ensure mem safety, there's another detail to what happens in
  // this situation in Rust. Instead of trying to copy the allocated
  // memory, Rust considers a to no longer be valid and therefore, Rust
  // doesn't need  to free anything when a goes out of scope.
  //
  // If you were to try to use a after copying it to b, an error
  // is thrown at compile time.
  //
  // The ideas of "shallow copy" and "deep copy" apply here. The
  // concept of copying the pointer, length, and capacity without
  // copying the data psounds like a shallow copy. But because rust
  // also invalidates the first variable, instead of calling this
  // a shallow copy, it's known as a *move*. We would read this by
  // saying that a was *moved* into b.
  //
  // This solves the problem, because with only b as valid, when it
  // goes out of scope, it alone will free the mem.
  //
  // There is an additional design choice implied by this:
  // ** Rust will never automatically create "deep" copies of data. **
  // Therefore, any *automatic* copying can be assumed to be
  // inexpensive in terms of runtime performance.
  //
  // + Ways variables and data interact: Clones
  //
  // If you *do* wawnt to deeply copy the heap data of a String, not
  // just the stack data, a common method can be used called a *clone*.
  //
  // let a = String::from("hello");
  // let b = a.clone();
  //
  // When you see a clone call, you know some arbitrary code is being
  // executed and that code may be expensive. It's an indiator that
  // something different is going on.
  //
  // + Stack only data: clone
  // There's another wrinkle we haven't talked about yet. This code
  // is using integers:
  // let x = 5;
  // let y = x;
  //
  // println!("x = {}, y = {}", x, y);
  //
  // This seems to contradict what we said; we don't have to call
  // clone, but x is still valid and wasn't moved to y.
  //
  // The reason is types like integers that have a known size at
  // compile time are stored *entirely* on the stack, so copies
  // of the actual values are very quick to make. There's no reason
  // we would want to prevent x from being valid after we create the
  // variable y.
  //
  // In other words, there's no different between deep and shallow
  // copying here, so calling clone wouldn't do anything different
  // from the usual shallow copying and we can leave it out.
  //
  // Rust has a special annotation called the Copy trait that can
  // be placed on types like integres that are stored on the stack.
  // If a type has the Copy trait, an older variable is still usable
  // after assignment. Rust won't let us annotate a type with the Copy
  // trait if the type, or any of its parts, has implemented the Drop
  // trait.
  //
  // If the type needs something special to happen when the value goes
  // out of scope and we add the Copy annotation to that type, we'll get
  // a compile time error.
  //
  // What types are Copy? Docs can/should be read, but as a general rule,
  // any group of simple scalar values can be Copy, and nothing that
  // requires allocation or is some form of resource is Copy.
  // -> ints, bools, floats, tuples (only if they contain also Copys).
  //
  // + Ownership and Functions]
  // Semantics for passing a value to a function are similar to
  // assigning a value to a variable. Passing a variable to a func
  // will move or copy just like assignment.
let s = String::from("derpyfoobar"); // s comes into scope
takes_ownership(s); // s's value moves into the function...
// ... and so is no longer valid here.
//println!("{}", s); // COMPILE ERROR!

let x = 5;
makes_copy(x);
println!("{} ", x); // This is fine, because it was a copy.

// [Return values and scope]
// Returning values can also transfer ownership. Here's an ex
// with similar annotations to previous examples:
{
  let baz = gives_ownership(); // gives ownership moves its return
  // value into baz
  let duder = String::from("duder"); // duder comes into scope
  let lucha = takes_and_gives_back(duder); // duder is moves into
  // takes_and_gives_back, which also moves its return value into lucha
  println!("lucha! {}", lucha);
} // Here lucha goes out of scope and is dropped. duder goes out of
// scope but was moved

// The ownership of a variable follows the same pattern every time:
// **assigning a value to another variable moves it**. When a variable
// that includes data on the heap goes out of scope, the value will be
// cleaned up by `drop` unless the data has been moved to be owned by
// another variable.
//
// Taking ownership and then returning ownership with every fn is
// tedious. What if we need to let a function use a value but not take
// ownership? It's quite annoying that anything we pass in also needs
// to be passed back if we want to use it again, in addition to any
// data resulting from the body of the fn that we may want to return
// as well.
//
// It's possible to return multiple values using a tuple.
// But it's still obnoxious to constantly pass back a ton of stuff.
//
// Rust has a way to address this, and its called references!!!
//
////////////////////////////////////////////////////////////////////////////////
}

fn refs() {
  // [References and Borrowing]
  // The issue with the returning tuple code we've seen elsewhere in
  // the ownership section is that we have to return the String to
  // the calling function so we can still use the String after the call.
  // Here we define calculate_length so that it uses a *reference* to
  // an object as a param instead of taking ownership of the value.

  let calc_len = |s: &String| -> usize {
    s.len()
  };

  let duderington = String::from("duderington");
  println!("the length of the string. -> {}", calc_len(&duderington));

  // First, all the tuple code in the variable declaration is gone.
  // We pass the string into the function as &duderington, and in the
  // definition, we take &String rather than String.
  //
  // The ampersands are *references*, and they allow you to refer to
  // some value without taking ownership over it.
  // s inside of calc_len becomes a pointer to the String struct bound
  // to duderington, which itself contains a ptr to the actual string
  // data on the heap.
  //
  // A closer look at the function call:
  //
  // let ano = String::from("ano");
  // let len = calc_len(&ano);
  //
  // The &ano syntax lets us create a ref that refers to the value
  // of ano, but does not own it. Because it does no own it, the val
  // it points to will not be dropped when the ref goes out of scope.
  //
  // Likewise, the sig of the fn uses & to indicate the type of the
  // param s is a ref.
  //
  // The scope in which the variable s is valid is the same as any
  // fn param scope, but we don't drop what the ref points to when it
  // goes out of scope because we don't have ownership.
  // Functions that have refs as params instead of vals mean we don't
  // need to return the vals in order to give back ownership, since
  // we never had ownership in the first place.
  //
  // What happens if we try to mutate something we borrowed?
  // Compiler errors. As vars are immutable by default, so are refs.
  // We are not allowed to modify a vanilla ref.
  //
  // + Mutable refs
  let change = |some_str: &mut String| {
    some_str.push_str(" fu.");
  };

  let mut s = String::from("mutref");
  change(&mut s);
  println!("mutref string after stuff: [{}]", s);

  // Mutable refs have a big caveat: you can only have one mutable ref
  // to a particular piece of data in a particular scope. This will fail:
  //
  // let mut s = String::from("fert");
  //
  // let r1 = &mut s;
  // let r2 = &mut s;
  //
  // This restriction allows for mutation but in a very controlled
  // fashion. It's something that new Rusters struggle with, because most
  // langs let you mutate whenever you'd like. Benefit is that Rust
  // can prevent data races at compile time.
  //
  // A data race is a particular type of race condition in which these
  // three behaviors occur:
  // 1) Two or more pointers access the same data at the same time
  // 2) At least one of the pointeres is being used to write to the data
  // 3) No mechanism being used to sync the access to the data
  //
  // Data races cause undefined behavior and can be very difficult to
  // diagnose and solve when you're trying to figure out what's
  // happening at runtime. Rust won't even let you compile it.
  //
  // Rust will let you create a new scope allowing for multiple mut
  // refs, but just not *simultaneous* ones!
  //
  let mut s = String::from("fert");
  {
    let r1 = &mut s;
  } //r1 goes out of scope here, so we can make a new ref with no prob
  let r2 = &mut s;

  // A similar rule exists for combining immutable refs.
  // This will error out.
  //let r1 = &s;
  //let r2 = &s;
  //let r3 = &mut s;
  //
  // Rust *also* does not allow for a mut ref while we have an
  // immutable one. Users of an immutable ref don't expect the vals
  // to change from under them. Multiple immutable refs are okay
  // because no one who is reading the data has the ability to modify
  // anybody else's data.
  //
  // + Dangling refs
  // In langs with pointers, it's easy to erroneously create dangling
  // pointers, or pointers that ref memory that may have been given
  // to someone else, by freeing some meory while preserving a pointer
  // to that memory. This is called a use after free.
  //
  // If we have a ref to some data, the compiler ensures that the data
  // will not go out of scope before the ref to the data does.
  // This will error out:
  //

  /*
  let dangle = || -> &String{
    let s = String::from("hello");
    &s
  }
  let ref_to_nothign = dangle();
  */

  // This will complain about something we haven't covered yet:
  // *lifetimes*.

  // The key here is the return type contains a borrowed value, but there
  // is no value for it to be borrowed from.
  // Because s is created inside dangle, when the code of dangle is
  // finished, s will be deallocated. But we tried to return a ref
  // to it. That means this ref would be pointing to an invaild String.
  // Rust won't let us do that.

  // The solution here is to return the String directly, thus transfering
  // ownership via a move to the caller of the fn.
  let no_dangle = || -> String {
    let s = String::from("no dangle");
    s
  };

  let ndstr = no_dangle();
  println!("{}", ndstr);

  // Recapping the rules of refs:
  // 1) At any given time you can have *either* but not both of:
  //   -> One mutable ref
  //   -> Any number of immutable refs
  // 2) References must always be valid
}

fn takes_ownership(some_string: String) { // some_string comes into scope
  println!("{}", some_string);
} // here some string goes out of scope and `drop` is called. The
// backing memory is freed.

fn makes_copy(some_integer: i32) { // some integer comes into scope.
  println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String { // Gives ownership will move its return
  // value into the function that calls it
  let derp = String::from("derp"); // derp comes into scope
  derp // derp is returned and moves out to the calling function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(foo: String) -> String { // foo comes into scope
  foo // foo is returned and moves out to the calling fn
}
