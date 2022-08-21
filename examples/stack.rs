use rust_linked_list::stack;

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
