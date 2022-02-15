The node def:

```
out: sin ~pitch >> mul ~envb >> mul 0.8; // here the sink, if there are morethanone out, they will be added to the sink

~envb: ~triggerb >> envperc 0.01 PARA_0;

~env_pitch: ~triggerb >> envperc 0.01 0.1;

~pitch: ~env_pitch >> mul 50 >> add 60;

~triggerb: SOURCE;
```

Our code is 

```
a1: seq 60 >> bd 0.3 >> mul 0.1;
```

We want an output like this:

```
to_sink: ~out1 // >> add ~out2

~envb: ~triggerb >> envperc 0.01 PARA_0;

~env_pitch: ~triggerb >> envperc 0.01 0.1;

~pitch: ~env_pitch >> mul 50 >> add 60;

~triggerb: [[seq 60]];

~out1: [[sin ~pitch >> mul ~envb >> mul 0.8]]

```