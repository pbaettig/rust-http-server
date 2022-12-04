use std::vec;

fn double_in_place(nums: &mut Vec<i64>) {
    for n in nums {
        *n = *n * 2 ;
    }
}

fn sum_integers(nums: Vec<i64>) -> i64 {
    return nums.iter().sum();
}

fn something(s: String) {}

fn say(s: &String) {
    println!("{}", s)
}

fn make_important(s: &mut String) {
    s.push('!')
}

fn main() {
    let mut msg = String::from("hello world");
    {
        let mutable_ref = &mut msg;
        make_important(mutable_ref);
    }
    
    
    
    let immutable_ref = &msg;
    say(immutable_ref);

    
    
    // this doesn't work because `make_important` takes a
    // mutable reference
    // make_important(immutable_ref);
}