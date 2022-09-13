## glicol-rs

You can use this crate to build audio apps with Glicol syntax.

```rust
use glicol::Engine; 
fn main() {
    let mut engine = Engine::<32>::new();
    engine.update_with_code(r#"o: sin 440"#);
    println!("next block {:?}", engine.next_block(vec![]));
}
```

More examples [here](https://github.com/chaosprint/glicol/tree/main/rs/main/examples).

Learn Glicol syntax [here](https://glicol.org).

It compiles to WebAssembly and runs in browsers.

It can also be used on VST and Bela, but these are all experimental.

See the GitHub repository for details.