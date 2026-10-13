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

- See excercises for first chapters

- No idea how to introduce map in between, since you almost never use it with `for` but we did not yet introduce `sum`, `collect`, etc.
  - Practical example extract only the `service_maximum_delay` field and find the maximum
  - maybe this could share a chapter with enumerate? As a `combinator` chapter

- Consuming iterators (maybe this could be one chapter?)
  - Goal: See that you can consume an iterator by more than `for` loops
  - Using [sum](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.sum) to sum up values instead of a for loop
    - Sum up all the `stop_departure_delay`
  - Using [find](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.find) to find the first Intercity to depart from in Nijmegen

- Filtering items
  - Goal: Learn that you can select specific items from an iterator
  - https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.filter

- Collecting items
  - Goal: Learn that we can create a new collection from an iterator, show [FromIterator](https://doc.rust-lang.org/stable/std/iter/trait.FromIterator.html) trait and all the things that implement it
  - Collect a `Vec` of all Nightjet train numbers
  - Collect a `HashMap` of departure delays per stop name for the NightJet 420 on the 2026-05-01
  - Collect a Result? (Is really handy, but kinda obscure?)
  - Use [fold](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.fold) to build a HashMap that counts how many trains stopped at a given station

- Final boss
  - Goal: Learn to split up an iterator taks into chunks that fit into combinators
  - List all the stops the most delayed train had an how much it was delayed at each stop

## Useful combinators not yet covered:

See: [Iterator trait](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html)

* `all`, `any`, `count`
* `chain`, `zip`
* Using `map` to lookup extra data in a HashMap
* `cloned`, `by_ref`
* `filter_map`
* `flat_map`, `flatten`
* `max*`, `min*`
* `take`, `nth`, `skip`, `step_by`
* `take_while`, `skip_while`
* `peekable`
* `rev`
* `scan`

## Things that are not content that need doing later

* Add a CI pipeline based on the one in Rustlings
* Add CI for the libs part
* Add hints
* Add solutions to excercises
* Add Contributions, License, CoC from karel
