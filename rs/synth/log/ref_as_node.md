# Feature
In Glicol, refs can be used as an independent node. There are three cases:
(1) at the head as source;
(2) appear in the middle as an effect;
(3) show in the end of a chain.

Example 1:
```r
~source: seq 60;
out_a: ~source >> sp \bd;
out_b: ~source >> mul 0.5 >> sp \bass;
```

Example 2:
```r
~fx: lpf 300 1.0;
out_a: seq 60 >> sp \bass >> ~fx >> mul 0.1;
out_b: seq 60 _60 >> sp \bass >> ~fx;
```

Previously, the parser will first detect this pattern by judging its name and paras.

Using ref as a node, the name is an empty "", and the para is the ref itself only.

Thus, a `Pass` node can be created and the ref is passed as its parameter.

This can be seen as a syntax sugar.

# Changes
In standalone mode, the `Pass` node no longer handles the sidechain info.