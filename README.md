# Rust Linked Lists

This crate aims to provide implementations in Rust of several data structures using linked lists as their underlying data structure

### Currently, the implemented data structures are:

* Stack
* Functional List
* Queue
* Double Linked List (TODO)

### Why Linked Lists?

I hate linked lists. With a passion. Linked lists are terrible data structures. 99% of the time you should just use a Vec (array stack), and 99% of the other 1% of the time you should be using a VecDeque (array deque). These are blatantly superior data structures for most workloads due to less frequent allocation, lower memory overhead, true random access, and cache locality.

Linked lists are as niche and vague of a data structure as a trie. Few would balk at me claiming a trie is a niche structure that your average programmer could happily never learn in an entire productive career -- and yet linked lists have some bizarre celebrity status. We teach every undergrad how to write a linked list. It's the list in C++!

We should all as a community say no to linked lists as a "standard" data structure. It's a fine data structure with several great use cases, but those use cases are exceptional, not common.

### TO-DO Linked List

- [x] Stack
- [x] Functional List
- [x] Queue
- [ ] Double-Linked List
- [x] Tests
- [ ] Examples
- [ ] Documentation
- [ ] Comply to [Rust Api Guidelines](https://rust-lang.github.io/api-guidelines/)