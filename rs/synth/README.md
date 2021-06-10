
Template for developing node:

```rust
impl Node<128> for MyNode {
    fn process(&mut self, inputs: &[Input<128>], output: &mut [Buffer<128>]) {
        let max_user_input = 1;
        let min_user_input = 0;
        let l = inputs.len();
        if l < 1 { return ()};
        let has_clock = inputs[l-1].buffers()[0][0] % 128. == 0. && inputs[l-1].buffers()[0][1] == 0.;

        if l - has_clock as usize > 1 {
            // has mod
            
        } else {               
            // no mod
        }
    }
}
```