# Simple Flecs 

Simple Rust bindings to the Flecs ECS library. 
Seeks to be as simple as possible to ensure inter DLL compatibility.

## Goals

- DLL consistency - you should be able to pass the world pointer without problems, while still use Rust's type system.
- Ease of use - this library should make working with Flecs a little bit easier

## Nice haves but not goals 

I. e. PRs are welcome, but no active efforts will be made in implementing these.

- Performance - since the bindings try to be as simple as possible, this comes at a cost of not using complex flecs's calls and stuff
- Addons - core is the main target, though some addons have been implemented
- Complete Safety - threat the library more as a C library and not like a Rust one, though the biggest (presumed) offenders are marked with unsafe

## Non-goals 

- Competition to the main `flecs_ecs` bindings, unless you need DLL consistency use them, instead of this one.
