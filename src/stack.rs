//! A [`Stack`] is a linear data structure that follows the principle of
//! Last In First Out (LIFO).
//!
//! This means the last element inserted inside the stack is removed first.
//!
//! This Stack implementation uses the linked list concept.
//! This implementation is 100% safe rust
//!

use std::mem;

type Link<T> = Option<Box<Node<T>>>;

/// Stack Struct
#[derive(Debug)]
pub struct Stack<T>(Link<T>);

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Link<T>,
}

impl<T> Stack<T> {
    /// Creates a new [`Stack`]
    ///
    /// # Example
    /// Creating a new [`Stack`] of `i32`
    /// ```
    /// use linked_lists_rs::stack::Stack;
    /// let stack: Stack<i32> = Stack::new();
    /// ```
    pub fn new() -> Self {
        Stack(None)
    }

    /// Push a new value on the top of the [`Stack`]
    ///
    /// # Example
    /// ```
    /// # use linked_lists_rs::stack::Stack;
    /// let mut stack = Stack::new();
    ///
    /// stack.push(5);
    ///
    /// assert_eq!(Some(5), stack.pop());
    /// ```
    pub fn push(&mut self, value: T) {
        let new_node = Node {
            value,
            next: mem::take(&mut self.0),
        };

        self.0 = Some(Box::new(new_node))
    }

    /// Pops and return the value on the top of the [`Stack`]
    /// Returns `None` if the [`Stack`] is empty
    ///
    /// # Example
    /// ```
    /// # use linked_lists_rs::stack::Stack;
    /// let mut stack = Stack::new();
    ///
    /// stack.push(5);
    ///
    /// assert_eq!(Some(5), stack.pop());
    /// assert_eq!(None, stack.pop());
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        let link = mem::take(&mut self.0);
        link.map(|node| {
            self.0 = node.next;
            node.value
        })
    }

    /// Return a reference to the value on the top of the [`Stack`]
    /// Returns `None` if the [`Stack`] is empty
    ///
    /// # Example
    /// ```
    /// # use linked_lists_rs::stack::Stack;
    /// let mut stack = Stack::new();
    ///
    /// stack.push(5);
    ///
    /// assert_eq!(Some(&5), stack.peek());
    /// assert_eq!(Some(5), stack.pop());
    /// assert_eq!(None, stack.peek());
    /// ```
    pub fn peek(&self) -> Option<&T> {
        let link = self.0.as_ref();
        link.map(|node| &node.value)
    }

    /// Return a mutable reference to the value on the top of the [`Stack`]
    /// Returns `None` if the [`Stack`] is empty
    ///
    /// # Example
    /// ```
    /// # use linked_lists_rs::stack::Stack;
    /// let mut stack = Stack::new();
    ///
    /// stack.push(5);
    ///
    /// assert_eq!(Some(&mut 5), stack.peek_mut());
    /// stack.peek_mut().map(|mut v| *v *= 5);
    /// assert_eq!(Some(25), stack.pop());
    /// assert_eq!(None, stack.peek_mut());
    /// ```
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        let link = self.0.as_mut();
        link.map(|node| &mut node.value)
    }
}

// Custom code within the destructor.
impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        let mut link = mem::take(&mut self.0);
        while let Some(mut node) = link {
            link = mem::take(&mut node.next);
        }
    }
}

/// [`IntoIter`] struct for [`Stack`] consumed iteration
/// Iterate from top to bottom
pub struct IntoIter<T>(Stack<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<T> Stack<T> {
    /// Iterator to the [`Stack`]
    /// Consumes the data structure on iteration
    ///
    /// # Example
    /// ```
    /// # use linked_lists_rs::stack::Stack;
    /// let mut stack = Stack::new();
    ///
    /// // Insert values into the stack
    /// for x in [1, 2, 3] {
    ///     stack.push(x);
    /// }
    ///
    /// // Iterate the stack and verify its values
    /// for (i, x) in std::iter::zip(stack, [3, 2, 1]) {
    ///     assert_eq!(i, x);
    /// }
    ///
    /// ```
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> IntoIterator for Stack<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}

/// [`Iter`] struct for [`Stack`] referenced iteration
/// Iterate from top to bottom
pub struct Iter<'a, T>(Option<&'a Node<T>>);

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.take().map(|node| {
            self.0 = node.next.as_deref();
            &node.value
        })
    }
}

impl<'a, T> IntoIterator for &'a Stack<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T> Stack<T> {
    /// Reference Iterator to the [`Stack`]
    ///
    /// # Example
    /// ```
    /// # use linked_lists_rs::stack::Stack;
    /// let mut stack = Stack::new();
    ///
    /// // Insert values into the stack
    /// for x in [1, 2, 3] {
    ///     stack.push(x);
    /// }
    ///
    /// // Use iter to iterate the stack and verify its values
    /// for (i, x) in std::iter::zip(&stack, [3, 2, 1]) {
    ///     assert_eq!(i, &x);
    /// }
    ///
    /// // Stack is not consumed
    /// assert_eq!(Some(&3), stack.peek());
    /// ```
    pub fn iter(&self) -> Iter<'_, T> {
        let node = self.0.as_deref();
        Iter(node)
    }
}

/// [`IterMut`] struct for [`Stack`] mutable referenced iteration
/// Iterate from top to bottom
pub struct IterMut<'a, T>(Option<&'a mut Node<T>>);

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.take().map(|node| {
            self.0 = node.next.as_deref_mut();
            &mut node.value
        })
    }
}

impl<'a, T> IntoIterator for &'a mut Stack<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T> Stack<T> {
    /// Mutable Reference Iterator to the [`Stack`]
    ///
    /// # Example
    /// ```
    /// # use linked_lists_rs::stack::Stack;
    /// let mut stack = Stack::new();
    ///
    /// // Insert values into the stack
    /// for x in [1, 2, 3] {
    ///     stack.push(x);
    /// }
    ///
    /// // Use iter_mut to iterate the stack and mutate it's values
    /// for i in &mut stack {
    ///     *i *= 2;
    /// }
    ///
    /// // Assert values mutate as expected
    /// for x in [6, 4, 2] {
    ///     assert_eq!(Some(x), stack.pop());
    /// }
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        let node = self.0.as_deref_mut();
        IterMut(node)
    }
}

#[cfg(test)]
mod tests {
    use super::Stack;

    #[test]
    fn creates_an_empty_list() {
        let list: Stack<i32> = Stack::new();
        assert_eq!(None, list.peek())
    }

    #[test]
    fn pushes_element() {
        let mut list = Stack::new();
        let value = 5;
        list.push(value);
        assert_eq!(Some(&value), list.peek());
    }

    #[test]
    fn pops_empty() {
        let mut list: Stack<i32> = Stack::new();
        assert_eq!(None, list.pop())
    }

    #[test]
    fn pops() {
        let mut list = Stack::new();
        let value = 5;
        list.push(value);
        assert_eq!(Some(value), list.pop());
    }

    #[test]
    fn peek() {
        let mut list = Stack::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));
        list.peek_mut().map(|value| *value = 42);

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }

    #[test]
    fn into_iter() {
        let mut list = Stack::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = Stack::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn iter_mut() {
        let mut list = Stack::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }
}
