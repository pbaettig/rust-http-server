# Ownership
**Each value has exactly one owner**
When a variable is assigned to another one it's value is either moved or copied. If it's moved, the original variable is no longer valid, because the ownership of the value has been transferred to the new variable to enforce that every value has exactly one owner.
```rust
fn main() {
    let a = String::from("1234");
    
    // value of `a` is moved to `b`, invalidating `a`
    let b = a;

    // this works fine because `b` is the new owner
    println!("b:{}", b);

    // this fails becaues `a` does not own the data anymore
    println!("a:{}", a);
    //               ^ value borrowed here after move
}
```


Whether a value is moved or copied depends on if the type immplements the `Copy` trait. Complex, heap allocated types like `String` generally don't while basic types such as integers do.
```rust
fn main() {
    let a = 42;

    // value of `a` is copied to `b`. `b` now has it's own `5`
    let b = a;

    // `a` and `b` each have their own data, therefore both of these
    // statements work
    println!("b:{}", b);
    println!("a:{}", a);
}
```

We can manually copy types that don't implement the `Copy` trait.
```rust
fn main() {
    let a = String::from("1234");
    
    // value of `a` is copied to `b`, both `a` and `b` point to valid memory
    let b = a.clone();
    // or  let b = a.to_owned();
  
    println!("a:{}, b:{}", a, b);
}
```


The same applies not just for assigning variables but also when values are passed as parameters to functions
```rust
fn something(s: String) {}

fn main() {
    let a = String::from("abcdef");

    println!("a:{}", a);
    

    // value of `a` is moved to something...
    something(a);

    // and is invalid from there on
    println!("a:{}", a);
}
```

## But why?
As soon as a variable goes out of scope, rust `free`s the underlying memory. Obviously there's trouble if another variable still references that data.
```rust
fn something(s: String) {}

fn main() {
    // memory is initialized here
    let a = String::from("abcdef");
    
    // value of `a` is moved to something. the underlying memory of `a`
    // is freed as soon as the function returns
    something(a);

    // at this point the memory of `a` has already been freed and
    // can therefore no longer be used
    println!("a:{}", a);
}
```

# Borrowing
To let a variable use the value of another one or to pass a variable into a function **without** changing the owner we get a reference. In rust this is called "borrowing".
References can be mutable or immutable, just like variables. The mutability of the reference is independent of the mutability of the borrowed variable. We can have immutable references to mutable variables but not the other way round (mutable reference to immutable variable).
```rust
// say receives an immutable reference since it doesn't mutate
// the string
fn say(s: &String) {
    println!("{}", s)
}

// make_important receives a mutable reference because it mutates
// the string
fn make_important(s: &mut String) {
    s.push('!')
}

fn main() {
    let mut msg = String::from("hello world");
    
    let immutable_ref = &msg;
    say(immutable_ref);
    

    let mutable_ref = &mut msg;
    make_important(mutable_ref);
    say(&msg);

    // this doesn't work because `make_important` takes a
    // mutable reference
    make_important(immutable_ref);
}
```

we can grab multiple immutable references at the same time:
```rust
fn main() {
    let mut msg = String::from("hello world");
    
    let immutable_ref1 = &msg;
    let immutable_ref2 = &msg;
    let immutable_ref3 = &msg;
    say(immutable_ref1);
    say(immutable_ref2);
    say(immutable_ref3);
}
```

but if we have a mutable reference we can not grab any other (mutable or immutable) reference at that time
```rust
fn main() {
    let mut msg = String::from("hello world");
    
    let mutable_ref = &mut msg;

    // this fails because we already have a mutable referece for msg
    let immutable_ref = &msg;
   
}
```

References go out of scope like variables. As soon as we leave a scope the references declared in that scope are invalidated. In the code below we grab a mutable and an immutable reference to `msg`. These borrows are OK because they happen in different scopes. `mutable_ref` has been invalidated by the time we take `immutable_ref`.
```rust
fn main() {
    let mut msg = String::from("hello world");
    {
        let mutable_ref = &mut msg;
        make_important(mutable_ref);
    }
    
    let immutable_ref = &msg;
    say(immutable_ref);
    
```