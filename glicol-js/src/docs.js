const audio = {
    range: {
        low: "-1.0",
        high: "1.0"
    }
}
const range =  {
    sin: audio,
    saw: audio,
    squ: audio,
    noiz: audio,
}

const para = (p) => {
    return {parameters: p}
}

const table = {
    sin: para(["freq"])
}

const about = {
    sin: "Outputs sine wave audio signal."
}

const example = {
    sin: ()=>{console.log("%ceg: %csin %c440.0", "color: #C99E00", "color: #8959A8", "color: #3E999F")}
}

export default { range, about, table, example }