const decoder = new TextDecoder('utf-8');
const errors = [
    "trying to use a non-existent sample.",
    "trying to connect to an invalid reference.",
    "this node parameter only accepts a number.",
    "unable to build the node.",
]
// console.log("%cAt line "+String(result[1]+1)+".", "color: white; background: green")
const handler = e => {
    console.log(`%cError: ${errors[e.data[0]-1]}`, "color: white; background: red")

    if (e.data[0] === 2) {
        let name = decoder.decode(e.data.slice(2).filter(v => v !== 0.0))
        let index = window.code.indexOf(name)
        let code = window.code.slice(0, index)

        let line = code.split("\n").length;
        console.log("%cAt line "+String(line)+".", "color: white; background: green")
    } else {
        console.log("%cAt line "+String(e.data[1]+1)+".", "color: white; background: green")
    }
    console.log("%cError element: "+decoder.decode(e.data.slice(2)), "color:white;background:pink");
  };

export default handler;