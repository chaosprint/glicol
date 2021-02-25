const audio = {
    range: {
        low: "-1.0",
        high: "1.0"
    }
}
const range = {
    sin: audio,
    saw: audio,
    squ: audio,
    noiz: audio,
    sampler: {
        range: {
            low: "depends on the sample",
            high: "depends on the sample"
        }
    },
    imp: {
        range: {
            low: "0.0",
            high: "1.0"
        }
    }
}

const params = {
    // sin: para(["freq"])
    sin: [
        ["freq", "determine the frequency of the sine wave", "modulable"]
    ],
    saw: [
        ["freq", "determine the frequency of the sawtooth wave", "modulable"]
    ],
    squ: [
        ["freq", "determine the frequency of the square wave", "modulable"]
    ],
    mul: [
        ["mul", "determine how much the input signal is multiplied/amplified", "modulable"]
    ],
    add: [
        ["add", "determine how much the input signal is added/shifted", "modulable"]
    ],
    imp: [
        ["freq", "determine the frequency of the impluse signal", "not modulable"]
    ],
    sampler: [
        ["sample_name", "determine which sample to use", "not modulable"]
    ],
}

const about = {
    sin: "outputs sine wave audio signal",
    saw: "outputs sawtooth wave audio signal",
    squ: "outputs sawtooth wave audio signal",
    mul: "multiply the input signal by a constant value",
    add: "add the input signal by a constant value",
    imp: "outputs an impulse signal",
    sampler: "play back the sample based on the value of its input. 1.0 triggers the default pitch. a trigger of value 2.0 will make the playback speed double. note: every non-zero value will trigger the playback once.",
    sp: "play back the sample based on the value of its input. 1.0 triggers the default pitch. a trigger of value 2.0 will make the playback speed double. note: every non-zero value will trigger the playback once.",
    buf: "holds a buffer; the input signal should be within the range from 0.0 to 1.0, and the output depends on the input value.",
    seq: "outouts the trigger based on the given pattern.",
    linrange: "maps the input from (-1.0, 1.0) to the given range",
    lpf: "is a low pass filter.",
    hpf: "is a high pass filter.",
    spd: "controls the speed of a sequencer; should be used with seq",
    speed: "controls the speed of a sequencer; should be used with seq",
    noiz: "outputs noise signal.",
    choose: "can be used to select values from its arguments randomly.",
    envperc: "generates a percusive shape envelope.",
    pha: "generates phosor signal.",
    state: "is an experimental node which generated states.",
    pan: "determined the Left Right audio panning.",
    delay: "delays the signal based on the given parameter (milesecond).",
    apf: "is an all pass filter.",
    comb: "is a comb fiter.",
    mix: "mix stereo signals.",
    plate: "is the Dattoro reverb.",
    onepole: "is a one pole filter.",
    allpass: "is another all pass filter based on sample size.",
    delayn: "is the delay based on the sample numbers.",
    monosum: "sums up mono signals.",
    const: "generates a constance value.",
}

const example = {
    sin: ["%ca_ref%c: %csin %c440", "color: #C99E00", "color: #808080", "color: #a84275", "color: #5a9c54"],
    saw: ["%ca_ref%c: %csaw %c110", "color: #C99E00", "color: #808080", "color: #a84275", "color: #5a9c54"],
    squ: ["%ca_ref%c: %csqu %c110", "color: #C99E00", "color: #808080", "color: #a84275", "color: #5a9c54"],
    noiz: ["%ca_ref%c: %cnoiz %c42", "color: #C99E00", "color: #808080", "color: #a84275", "color: #5a9c54"],
    mul: ["%ca_ref%c: %csin %c110 %c>> %cmul %c0.1", "color: #C99E00", "color: #808080", "color: #a84275", 
    "color: #5a9c54", "color: #808080", "color: #8959A8", "color: #3E999F"],
    choose: ["// loadModules() first!\n~a: choose 60 72 0 0\n\nlead: speed 4.0 >> seq ~a >> sp \\blip", ]
    // add: "a_ref: sin 440 >> mul ~am\n\n~am: sin 0.2 >> mul 0.3 >> add 0.5"
    // sin: () => { console.log("%csome_ref: %csin %c440.0", "color: #C99E00", "color: #8959A8", "color: #3E999F") }
}

const note = {
    sin: "The default output range is from 0.0 to 1.0.",
    saw: "The default output range is from 0.0 to 1.0.",
    squ: "The default output range is from 0.0 to 1.0.",
    noiz: "The default output range is from 0.0 to 1.0.",
    choose: "The 0 means rests and the number of zeros can influence the probability."
}

const all = { about, params, range, example, note }

export default all;