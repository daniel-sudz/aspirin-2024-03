# find_in_string

By default the compiler assumes that the lifetime of a returned reference is the maximum of the lifetimes of all input parameters to the function. Therefore the compiler gives an error as it sees that after the call to `find_in_string(sentence,&word)` a `drop(word)` is made before the return is consumed by the `assert_eq!(found, Some("fox"))`. We fix this problem by anotating the lifetime of the return as equal to the lifetime of `sentence` only. Then the compiler will perform a local analysis of `find_in_string` and indeed verify that the returned slice only dependends on the lifetime of `sentence` and not `&word`.

# doubly_linked_list

Observe the code below:

```rust
let mut next_head = Some(Box::new(Node {
    val,
    next: None,
    prev: Some(prev_head)
}));
// here we encounter an issue as we have a circular ownership problem
// the compiler returns "value partially assigned here after move"
prev_head.next = next_head.take();
```

When we construct the `next_head` we have to move the `prev_head` into it. However, `prev_head` also needs a link in the other direction so it also consumes `next_head`. We have now created a borrow cycle that is impossible to resolve. 

Any kind of graph traversal where we need to dynamically modify edges/nodes would also be impossible to implement with our traditional borrow primatives that we have learned so far. This is because the borrow checker operates at compile time so static borrow checking is impossible on a dynamic structure. 