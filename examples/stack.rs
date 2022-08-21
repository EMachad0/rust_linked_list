use linked_lists_rs::stack;

fn main() {
    let mut stack = stack::Stack::new();

    for i in 0..10 {
        stack.push(i)
    }

    for i in &mut stack {
        *i *= 2;
    }

    println!("Stack");
    stack.iter().for_each(|i| println!("{:?}", i));
}
