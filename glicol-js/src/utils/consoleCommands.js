import docs from './docs'
import { intro } from './examples'
import { WaveFile } from 'wavefile';

const encoder = new TextEncoder("utf-8")
window.docs = docs
window.code = intro

window.help = (token) => {
    console.clear()
    if (token in window.docs.about) {
        console.log("%c"+window.art, "color: grey")
        console.log("\n\n%c About: ", "background: black; color:white; font-weight: bold")
        // console.log("about")
        console.log("%c"+token, `color: yellow`, `${window.docs.about[token]}`)
    }  else {
        console.error(`Move your cursor to an non-empty place where you wish to search.
        \nFor example, if you wish to search "sin", your cursor should be inside "sin" like this: s|in`)
        // return `--------------------------`
    }

    if (token in window.docs.params) {
        console.log("\n\n%c Parameters: ", "background: black; color:white; font-weight: bold")
        // console.log("para")${window.docs.color[token]}
        let p = window.docs.params[token];
        
        p.forEach(a=>{
            let c = a[2] === "modulable" ? "green" : "red"
            let name = `%c(${a[2]}) ${a[0]}`;
            let des = `- ${a[1]}`
            console.log(name, `color: ${c}`, des)
        })
    }

    if (token in window.docs.example) {
        console.log("\n\n%c Example: ", "background: black; color:white; font-weight: bold")
        console.log(...window.docs.example[token])
    }

    if (token in window.docs.note) {
        console.log("\n\n%c Note: ", "background: black; color:white; font-weight: bold")
        console.log(window.docs.note[token])
    }
        // console.log("\n\nNote:")
        // console.log("Some important things")
        // if (token in window.docs.range) {
        //     console.log("%cABOUT", "background: green; font-weight: bold");
        //     // console.log("%cABOUT:", "background: purple; color: white; font-weight: bold")
        //     console.log(`%c${window.docs.about[token]}`) //, "background: green; color: white")
        // } else {
        //     console.log(`%cstill under development...`, "background: red")
        // }
        // if (token in window.docs.params) {
        // // console.table(window.docs.params[token])
        // console.log("%cPARAMETERS", "background: green; font-weight: bold");
        // // console.log("%cPARAMETERS:", "background: purple; color: white; font-weight: bold")
        // window.docs.params[token].forEach(e=>{
        //     console.log(`${e[0]}: ${e[1]}`) //, "background: green; color: white");
        //     console.log(`${e[2]}`) //, "background: yellow; color: white");
        // })
        // }
        // if (token in window.docs.range) {
        
        // console.log("%cRANGE", "background: green; font-weight: bold");
        // console.table(window.docs.range[token])
        // }
        // if (token in window.docs.example) {
        // // console.log("example:")
        // console.log("%cEXAMPLE", "background: green; font-weight: bold");
        // window.docs.example[token]()
        // }

}


window.bpm = (beats_per_minute) => {
const t0 = performance.now();
if (typeof beats_per_minute === "number") {
    window.node.port.postMessage({
    type: "bpm", value: beats_per_minute})
    console.log(`%cBPM set to: ${beats_per_minute}`, "background: green");
    console.log("%c This will be effective when you make some changes to the code.", "background: yellow");
} else {
    console.warn("BPM should be a number.")
}
return `Execution time: ${(performance.now()-t0).toFixed(4)} ms`
}

window.trackAmp = (amp) => {
const t0 = performance.now();
if (typeof amp === "number") {
    if (amp <= 1.0) {
    window.node.port.postMessage({
        type: "amp", value: amp})
    console.log(`%cThe amplitude of each track is set to: ${amp}`,"background: green");
    } else {
    console.warn("Amplitude should not exceed 1.0.")
    }
} else {
    console.warn("Amplitude should be a number.")
}
return `Execution time: ${(performance.now()-t0).toFixed(4)} ms`
}


window.addArray = async (name, arr) => {
window.actx.suspend()
console.log(arr)
let f32 = new Float32Array(arr)
console.log(f32)
let i16 = new Int16Array(f32.buffer)
console.log(i16)
window.node.port.postMessage({
    type: "samples",
    sample: i16,
    name: encoder.encode("\\" + name)
})
}

window.addJSON = async (url, key) => {
window.actx.suspend()
let req = new Request(url)
await fetch(req).then(res=>res.json()).then(a=>{
    let arr = a[key]
    console.log(arr)
    let f32 = new Float32Array(arr)
    console.log(f32)
    let i16 = new Int16Array(f32.buffer)
    console.log(i16)
    window.node.port.postMessage({
    type: "samples",
    sample: i16,
    name: encoder.encode("\\" + key)
    })
})
}

window.addSample = async (name, url) => {
window.actx.suspend()
let myRequest = new Request(url);
await fetch(myRequest).then(response => response.arrayBuffer())
.then(arrayBuffer => {
    // console.log("downloaded", arrayBuffer)
    let buffer = new Uint8Array(arrayBuffer)
    let wav = new WaveFile(buffer);
    let sample = wav.getSamples(true, Int16Array)

    // after loading, sent to audioworklet the sample array
    // console.log("sampler \\" + key)
    window.node.port.postMessage({
    type: "samples",
    sample: sample,
    name: encoder.encode("\\" + name)
    })
});
}