# mypl-spec

## features

### type system

* strongly, statically typed
* mutability control
* no nulls

### abstraction

* ADT
    - can be composed into other - why no one did this? probably there is some issue with this...

* no object methods
* type classes
* static dispatch, monomorphizing at compile time
    - no dynamic dispatch as of now

### memory

* manual de/ allocations
* no implicit allocators

### general

* no exceptions
* defer like [zig](https://ziglang.org/documentation/master/#defer)


## not features

* be fast as C
* be terse (but not redundant)
* be easy