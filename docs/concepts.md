# Rust Concepts

## Module System in Rust

### Understanding Module Visibility and Imports

In Rust, modules are units of code organization that help you structure your code. The module system changed significantly in Rust 2018 edition.

#### Module Declaration and Visibility

- `mod module_name;` - Declares a module but keeps it private to the current scope
- `pub mod module_name;` - Declares a public module that can be accessed from other modules

#### Path Resolution in Rust 2018+

In Rust 2018 edition, paths in `use` statements must be explicit:

- `use crate::module_name` - Reference modules from the crate root
- `use self::module_name` - Reference modules relative to the current module
- `use super::module_name` - Reference modules from the parent module

#### Common Error: Unresolved Import

The error:
```rust 
error[E0432]: unresolved import model
--> src/handlers/handlers.rs:1:5
|
1 | use model::User;
| ^^^^^ help: a similar path exists: crate::model

```


This occurs when you try to import a module without specifying the full path. The module system can't find `model` because it doesn't know where to look for it.

#### Solution

Instead of:
```rust
use model::User;
```

Use the full path:
```rust
use crate::model::model::User;
```

### Module Organization Best Practices

1. **Root Module Declaration**: Declare modules at the crate root (main.rs or lib.rs)
   ```rust
   pub mod model;
   pub mod handlers;
   ```

2. **Module Export**: In mod.rs files, re-export submodules
   ```rust
   pub mod model; // In mod.rs
   ```

3. **Re-exporting for Convenience**: You can re-export types to simplify imports
   ```rust
   // In mod.rs
   pub use self::model::User;
   ```
   This allows other modules to use `use crate::model::User` instead of `use crate::model::model::User`

   ```