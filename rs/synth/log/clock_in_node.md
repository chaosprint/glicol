Since this `glicol_synth` crate is now design to consider standalone usage as a Rust audio library, in each node there are some changes to note.

The most important thing is to differ two cases: if it has clock, if it has sidechain.

The clock is important in Glicol. In the original `dasp_graph` crate, there is no way to stop double processing.

Example:
```
~source: sin 60;
out_a: ~source >> mul 0.5;
out_b: ~source >> delayn 30 >> mul 0.3;
```

Without a clock, when processing `out_b`, the source `sin` node will already pass the current phase, because the `process()` method inside `sin` has been called once in `out_a`. This calling will move its internal phase forward already.

The solution is to create a dummy clock to each node.