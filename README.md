# Sorted Collections

This is an experiment in implementing datastructures in safe Rust, following
the description given in Grant Jenks' [talk][py-talk] and the
[documentation][py-docs] for the Python library.

If you're interested in seeing the Python code, you can find that on
[github][py-git] (I haven't read it).

## Safe Datastructures?

Rust is exciting for its ability to provide memory safety without garbage
collection. This is accomplished with an ownership and lifetime system that
xplicitly disallows multiply-aliased mutable references in "safe" code.

This means that container types that have multiply-aliased data (that is, two
pointers to the same object) *must* use `unsafe` somewhere. There are ways to
avoid using `unsafe` in one's own code -- for example, one can use something
like [`Option<Rc<RefCell<Node<T>>>>`][too-many-rc] (which has a runtime cost),
or have immutable collections.

However, types that don't have multiple aliasing (such as a binary search tree,
or a list) shouldn't need to use `unsafe` code. The last time I checked, the
source for [`BTreeMap`][btree-src] used `unsafe` 41 times. I wanted to see for
myself how a data structure that didn't use `unsafe` would work, and to
implement a sorted list with better-than-`Vec` insert time for large lists.

## Benchmarks

Benchmarks are pretty spare at this point, and require a nightly Cargo to run.
Assuming you have a current nightly installed, you can run them with
```bash
rustup run nightly cargo bench
```
or
```bash
cargo +nightly bench
```

[py-git]: https://github.com/grantjenks/python-sortedcontainers
[py-talk]: https://www.youtube.com/watch?v=7z2Ki44Vs4E
[py-docs]: http://www.grantjenks.com/docs/sortedcontainers/
[too-many-rc]: https://rust-unofficial.github.io/too-many-lists/fourth-final.html
[btree-src]: https://doc.rust-lang.org/src/alloc/collections/btree/map.rs.html#123-126

