# 0.1.4

* Added `Packed*::from_bytes` to be able to actually create packed integers. I
  didn't think about it before because I don't actually need such a method,
  given my whole reason for writing this crate is to cast tranches of bytes to
  tranches of structs that implement the three traits.

# 0.1.3

* Added the `packed` module.

# 0.1.2

* Fixed a shameful bug where I forgot all trait bounds on the implementations
  for the few generic types this crate support, I'm very smart. 🙃
* Fixed the soundness checks in `passive_derive` which also didn't include
  trait bounds.
* Added a changelog to document my shame.

# 0.1.1

Introduced the `derive` feature and the `passive_derive` companion crate.

# 0.1.0

Initial release!
