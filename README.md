# next next next

Welcome to these community Rustlings exercises 😃 A collection of excercises to learn about iterators.

First, [install Rustlings using the official instructions](https://github.com/rust-lang/rustlings) ✅

Then, clone this repository, open a terminal in this directory and run `rustlings` to get started with the exercises 🚀

You need to download the May 2026 dataset from https://www.rijdendetreinen.nl/en/open-data/train-archive#downloads

## General structure

Have one or two intro tasks with artifical data like "Sum up `[1,2,3]`". Followed by a practical example with the train data.


Three parts to an iterator

1. Creating an iterator (`1..5`, `.iter()`, `.keys()`)
2. (Optional) modifying the iterator with combinators (`map`, `filter`)
3. Consuming the iterator (`for`, `collect`, etc.)

## Outline

- Iterators in `for` loops
  - Iterate a vec and sum up its values (explain implicit `.into_iterator`)
  - Iterate over a HashMap to look for a key matching a pattern (using explicit `.keys()` iterator)
  - Practical example: Sum up all the arrival delays

- Introduce combinators
  - Aggregating values
    - `sum`
      - redo summing vec (from intro)
    - `find`
      - redo finding a key (from intro)
  - Changing values
    - `map`
      - redo sum up arrival delays

- Collecting items
  - `Vec`
  - `HashMap` / `BTreeMap`
- Filtering
  - `filter`
  - `filter_map`
