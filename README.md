# Passive

This crate defines a trio of marker traits to classify types that fit some
definitions of "plain old data", also known as "passive data structures".

## The Trio

* `AlwaysAligned` represents types that are always aligned, in other words
  types whose alignment is of 1.
* `AlwaysValid` represents types for which all bit patterns are valid, e.g.
  `u8` but not `bool`.
* `Immutable` represents types that don't provide any inner mutability.

## Licensing

This crate is licensed under both the Apache 2.0 and MIT licenses, so you are
free to do whatever you want with it as long as you respect the terms from
these two.

If you are a highly paid worker at Google, Facebook, Apple, Amazon, Microsoft,
Palantir, Uber, Airbnb, Deliveroo, or any other company that prioritises profit
over people as strongly as they do, you can still use this crate. I simply wish
you will unionise and push back against the obsession for growth, control, and
power that is rampant in your workplace. Please take a stand against the
horrible working conditions they inflict on your lesser paid colleagues, and
more generally their gross disrespect for the very human rights they claim to
fight for.
