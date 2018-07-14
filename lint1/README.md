



Basically follow [how-to in The Unstable Book](https://github.com/rust-lang/rust/blob/1e4269cb83a14f53851460c4780ff06d0a9f1d50/src/doc/unstable-book/src/language-features/plugin.md#lint-plugins).

I think you should use nightly compiler.

For now, you need two more special flags to build due to compiler
update.

    #![feature(macro_vis_matcher)]
    #![feature(macro_at_most_once_rep)]

Lint implementation must be in a separate crate, and added as a dependency.

    [dependency]
    impl2 = { path = "impl2" }

Lint implementation crate must provide dynamic library build.

    [lib]
    crate-type = ["dylib"]


