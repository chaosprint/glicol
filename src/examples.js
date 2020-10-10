const welcome =

`~a: noiz 0 >> mul 1 >> add 40

~b: choose 35 45 0

~trigger: speed 8.0 >> loop ~a ~b

~env: ~trigger >> envperc 0.01 0.1 >> mul 0.5

~pitch: ~trigger >> mul 261.626

_lead: saw ~pitch >> mul ~env >> lpf ~cut 6.0

~cut: squ 0.5 >> linrange 300.0 3000.0`

const hello = 
`_hi: sin 440.0

// if this doesn't play, check your browser console
// chrome or firefox are recommended

// this is a comment
// uncomment the line below, and click on the update button to update the sound
// _another: sin 441.0

// try to control the volume by adding another node function
// _another: sin 441.0 >> mul 0.5

// this example shows the basic usage of nodes
// a node can have several inputting signals but only one output signal
// here "sin" is a node that outputs sine wave signal based on its argument frequency
// in this example, "sin" has no input signal
// "mul" has one input from its left hand side
// "mul" processes the input signal by multiplying the input signal with its first argument

// everything before the colon, e.g. "_hi" or "_another", is called [reference]
// this will be explained in the next page (am)`

const am = 
`// you can modulate a parameter using the following syntax.

_hi: sin 440.0 >> mul ~am

~am: sin 1.5 >> mul 0.3 >> add 0.5

// it doesn't matter you write ~am line before or after using it, which is called lazy evaluation
// however, remember that only those references beginning with "_" will be played
// while the ones beginning with "~" with be processed as a control signal`

const fm = 
`_hi: sin ~fm >> mul ~am

~am: sin 0.2 >> mul 0.3 >> add 0.5

// this linrange node map -1.0 to 1.0 the range you give
~fm: sin ~more >> linrange 100.0 1000.0

~more: sin 0.1 >> linrange 1.0 100.0`

const usesample = 
`// "imp" is used to trigger the sampler
// the default output amp of "imp" is 1.0
// you can multiply a float to change the pitch

// "sampler" node triggers the sample everytime it gets a non-zero signal from its left
// the pitch is determined by the value of this input signal
// the default pitch is 1.0; try to change the argument in "mul" node;
// for example, mul 2.0 will make the sample octave higher

_imp: imp 1.0 >> mul 1.0 >> sampler \\bd

// an easier way to handle sampler is to use the "seq" node
// it is a good way to work with midi pitches
// the default pitch is midi 60, so to make it one octave higher, you should change it to 72

// _tt: seq 60 >> sampler \\casio

// "seq" node also handles time and rhythm algorithmically
// all its arguments will occupy one bar with the default bar length to be 2.0 second (equivalent to bpm 120, 4/4)

// this one bar length will be first divided by space
// uncomment the following codes to see the difference
// _tt: seq 48 49 >> sampler \\casio
// _tt: seq 48 49 50 >> sampler \\casio
// _tt: seq 48 49 50 51 >> sampler \\casio
// ... 
// you can add more notets for the "seq" node by yourself to see

// try to replace some midi notes with underscore "_"
// "_" means rest
// _tt: seq 48 _ 50 51 >> sampler \\casio

// rest and midi notes can form compound notes, which will further divided that part
// _tt: loop 48 49 >> sampler \\casio
// _tt: loop 48 _49_ >> sampler \\casio
// _tt: loop 48 _49__ >> sampler \\casio
// _tt: loop 48 ___49 >> sampler \\casio

// you can use a speed node to control the speed of "seq" node
// _tt: speed 2.0 >> loop 48 _ _50 _ >> sampler \\casio

// use "choose" node to choose notes in the seq
// the convention is to use single-letter references
// zeroes means rest while the number of zeroes can influence the probability
// _tt: seq 48 ~c >> sampler \\casio

// ~c: choose 50 62 74 0 0`


const envelope = 

`// the envelope can also triggered by "imp" and "seq" but slightly different

// in that it resets to beginning phase everytime it receives a non-zero input

~tri: imp 1.0 >> envperc 0.01 0.5

_lead: sin 100.0 >> mul ~tri

// use "seq" to set both env and the pitch
// ~lp: seq 60 _48 _72 _60

// ~pitch: ~lp >> mul 261.626

// ~tri: ~lp >> envperc 0.01 0.5

// _lead: sin ~pitch >> mul ~tri`


const filter = 
`// there are several more nodes we haven't covered yet
// "squ" "saw" "noiz" "lpf "hpf"
// see how they are used in this example` + welcome

const demo1 = `_bd: speed 1.375 >> loop 60 >> sampler \\breaks165

~a: choose 63 62 58 53 0 0 0

_bass: speed 2.75 >> loop ~a ~a >> sampler \\jvbass >> mul 0.6

~c: noiz 0 >> mul 2 >> add 40

~b: choose 63 62 58 53 0

~trigger: speed 5.5 >> loop ~c ~b

~env: ~trigger >> envperc 0.01 0.1 >> mul 0.5

~pitch: ~trigger >> mul 261.626 >> mul 0.5

_lead: saw ~pitch >> mul ~env >> lpf ~cut 3.0 >> mul 0.8

~cut: sin 0.3 >> linrange 300.0 3000.0`


const demo2 = `_bd: speed 4.0 >> loop 60 >> sampler \\909

~a: choose 60 48 55

_aa: speed 2.0 >> loop ~a >> sampler \\arp >> mul 0.1 >> lpf 400.0 1.0

_xx: speed 2.0 >> seq 60 60 _60 _60 >> sampler \\stomp >> lpf 500.0 10.0

~b: choose 48 50

_ff: speed 2.0 >> seq _ ~b >> sampler \\v

~mod: sin 0.2 >> linrange 300.0 3000.0

_rm: seq _ 60 >> sampler \\voodoo >> lpf ~mod 3.0

_tok: seq _ 60 _ _ >> sampler \\tok`

export {hello, am, fm, envelope, usesample, filter, demo1, demo2, welcome}

// `~a: noiz 0 >> mul 10 >> add 60

// ~trigger: loop ~a

// ~env: ~trigger >> envperc 0.01 0.1 >> mul 0.5

// ~pitch: ~trigger >> mul 261.626

// _lead: saw ~pitch >> mul ~env`

// `~trigger: speed 3.0 >> loop 30 32 33 35 37 39 40 41

// ~env: ~trigger >> env_perc 0.01 0.1 >> mul 0.5

// ~pitch: ~trigger >> mul 261.626

// _lead: saw ~pitch >> mul ~env >> lpf ~mod 1.0

// ~mod: sin 0.2 >> mul 3000.0 >> add 5000.0`
// "_aa: loop 60 >> sampler \\bd"
// `~mod: sin 3.0 >> mul 2000.0 >> add 3000.0

// _aa: noiz \\raw >> lpf ~mod 1.0`

// `~mod: sin 4.9 >> mul 2000.0 >> add 3000.0

// ~am: sin 5.0 >> mul 0.3 >> add 0.5

// _aa: loop 60 >> sampler \\fm >> rlpf ~mod >> mul ~am`

// `_aa: loop 60 >> sampler \\bd`

// `~trigger: loop 60 58 _67 _62

// ~env: ~trigger >> env_perc 0.01 0.1 >> mul 0.5

// ~pitch: ~trigger >> mul 200.0

// _lead: sin ~pitch >> mul ~env`

// `~part: sin 440 >> mul 0.5

// _aa: ~part >> mul 0.1`

// `~fm: sin 30.0 >> mul 100.0 >> add 200.0

// ~am: sin 1.0 >> mul 0.3 >> add 0.5

// _aa: sin ~fm >> mul ~am`

// `_aa: sin 220.0 >> perc_env 0.1 0.1`
// perc_env 0.01 0.9

// `~aa: imp 0.5 >> env_perc 0.01 1.0

// _aa: sin 1000.0 >> mul ~aa`

// `_aa: loop 60 >> sampler \\bd

// _bb: loop 60 64 67 72 >> sampler \\bass`

// `_aa: loop 60 60 60 60 >> sampler \\bd

// _bb: loop 60 _67 _62 _65 >> sampler \\bass

// _cc: loop _ _75 80 60 70 ___80 __75 >> sampler \\can

// ~dd: sin 1.0 >> mul 0.3 >> add 0.5

// _dd: loop 62 67 _58 64 62 _67 _58 64 >> sampler \\808hc >> mul ~dd

// _ee: loop _ 60 _ 60 >> sampler \\jazz`

// `_aa: sin 220.0`
// `_aa: loop 60 >> sampler \\bd

// _bb: loop _ 60 >> sampler \\jazz

// _cc: loop _ 80 _70 75__70 >> sampler \\can`

// `_aa: imp 1.0 >> mul 1.0 >> sampler \\bd`

// `~cc: sin 1.0 >> mul 0.5 >> add 0.5

// _aa: sin 200.0 >> mul ~cc`

// "_bd: loop 60 >> sampler \\bd"
// `_bd: loop 60 >> sampler \\bd

// _hook: loop 40 _80_34 73__65 42 >> sampler \\808hc

// _jazz: loop _60 >> sampler \\jazz`