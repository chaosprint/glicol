const exampleCode = 

// `~aa: sin 220.0 >> perc_env 0.1 0.1`
// perc_env 0.01 0.9

`&aa: imp 1.0 >> env_perc 0.01 0.1

~aa: sin 100.0 >> mul &aa`


// `~aa: loop 60 >> sampler \\bd

// ~bb: loop 60 64 67 72 >> sampler \\bass`

// `~aa: loop 60 60 60 60 >> sampler \\bd

// ~bb: loop 60 _67 _62 _65 >> sampler \\bass

// ~cc: loop _ _75 80 60 70 ___80 __75 >> sampler \\can

// &dd: sin 1.0 >> mul 0.3 >> add 0.5

// ~dd: loop 62 67 _58 64 62 _67 _58 64 >> sampler \\808hc >> mul &dd

// ~ee: loop _ 60 _ 60 >> sampler \\jazz`


// `~aa: sin 220.0`
// `~aa: loop 60 >> sampler \\bd

// ~bb: loop _ 60 >> sampler \\jazz

// ~cc: loop _ 80 _70 75__70 >> sampler \\can`
// `~aa: imp 1.0 >> mul 1.0 >> sampler \\bd`

// `&cc: sin 1.0 >> mul 0.5 >> add 0.5

// ~aa: sin 200.0 >> mul &cc`

// "~bd: loop 60 >> sampler \\bd"
// `~bd: loop 60 >> sampler \\bd

// ~hook: loop 40 _80_34 73__65 42 >> sampler \\808hc

// ~jazz: loop _60 >> sampler \\jazz`
export {exampleCode}