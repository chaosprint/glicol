const hello = 
`~hi: sin 440.0

// if this doesn't play, check your browser console
// chrome or firefox are recommended


// this is a comment
// uncomment the line below, and click on the play button again
// ~another: sin 441.0`

const am = 
`// you can modulate a parameter using the following syntax.
// it doesn't matter you write &am line before or after using it
// however, remember that reference beginning with "~" will be played
// while the ones beginning with "&" with be processed as a control signal

~hi: sin 440.0 >> mul &am

&am: sin 0.2 >> mul 0.3 >> add 0.5`

const fm = 
`~hi: sin &fm >> mul &am

&am: sin 0.2 >> mul 0.3 >> add 0.5

// this linrange node map -1.0 to 1.0 the range you give
&fm: sin &more >> linrange 100.0 1000.0

&more: sin 0.1 >> linrange 1.0 100.0"
`

const usesample = 
`// "imp" is used to trigger the sampler
// the default output amp of "imp" is 1.0
// you can multiply a float to change the pitch

~imp: imp 1.0 >> mul 1.0 >> sampler \\bd

// loop is a good way to work with midi pitches
// try to uncomment the following lines to see the difference

// ~tt: loop 60 >> sampler \\casio

// ~tt: loop 48 50 >> sampler \\casio

// ~tt: loop 48 _50 >> sampler \\casio

// ~tt: loop 48 _ _50 _ >> sampler \\casio

// ~tt: speed 2.0 >> loop 48 _ _50 _ >> sampler \\casio`

const filter = 
`// if there is no sound, check the console for errors
// make sure you are using Chrome or FireFox
// if you do, one solution can be to use the incog mode (cmd + shift + n)

&a: noiz 0 >> mul 2 >> add 40

&b: choose 35 47

&trigger: speed 8.0 >> loop &a &b

&env: &trigger >> envperc 0.01 0.1 >> mul 0.5

&pitch: &trigger >> mul 261.626

~lead: saw &pitch >> mul &env >> lpf &cut 3.0

&cut: sin 0.3 >> linrange 300.0 3000.0`

const envelope = 

`// use 'imp' to trigger an envelop

&tri: imp 1.0 >> envperc 0.01 0.5

~lead: sin 100.0 >> mul &tri

// use the loop to give it a pitch

// &lp: loop 60 _48 _72 _60

// &pitch: &lp >> mul 261.626

// &tri: &lp >> envperc 0.01 0.5

// ~lead: sin &pitch >> mul &tri`

// `&a: noiz 0 >> mul 10 >> add 60

// &trigger: loop &a

// &env: &trigger >> envperc 0.01 0.1 >> mul 0.5

// &pitch: &trigger >> mul 261.626

// ~lead: saw &pitch >> mul &env`

// `&trigger: speed 3.0 >> loop 30 32 33 35 37 39 40 41

// &env: &trigger >> env_perc 0.01 0.1 >> mul 0.5

// &pitch: &trigger >> mul 261.626

// ~lead: saw &pitch >> mul &env >> lpf &mod 1.0

// &mod: sin 0.2 >> mul 3000.0 >> add 5000.0`
// "~aa: loop 60 >> sampler \\bd"
// `&mod: sin 3.0 >> mul 2000.0 >> add 3000.0

// ~aa: noiz \\raw >> lpf &mod 1.0`

// `&mod: sin 4.9 >> mul 2000.0 >> add 3000.0

// &am: sin 5.0 >> mul 0.3 >> add 0.5

// ~aa: loop 60 >> sampler \\fm >> rlpf &mod >> mul &am`

// `~aa: loop 60 >> sampler \\bd`

// `&trigger: loop 60 58 _67 _62

// &env: &trigger >> env_perc 0.01 0.1 >> mul 0.5

// &pitch: &trigger >> mul 200.0

// ~lead: sin &pitch >> mul &env`

// `&part: sin 440 >> mul 0.5

// ~aa: &part >> mul 0.1`

// `&fm: sin 30.0 >> mul 100.0 >> add 200.0

// &am: sin 1.0 >> mul 0.3 >> add 0.5

// ~aa: sin &fm >> mul &am`

// `~aa: sin 220.0 >> perc_env 0.1 0.1`
// perc_env 0.01 0.9

// `&aa: imp 0.5 >> env_perc 0.01 1.0

// ~aa: sin 1000.0 >> mul &aa`

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
export {hello, am, fm, envelope, usesample, filter}