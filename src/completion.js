import {sampleList} from './samples'


let c = "sin|saw|squ|lpf|hpf|imp|seq|loop|speed|choose|mul|add|envperc|linrange".split("|").concat(sampleList.selected).map(
    i => {
        return {
            caption: i,
            snippet: i,
            type: i
        }
    }
)

let d = [
    {
        caption: "sampler",
        snippet: "sampler \\",
        type: "sampler",
    }, {
        caption: "noiz",
        snippet: "noiz 0",
        type: "noiz",  
    }
]


export default c.concat(d)