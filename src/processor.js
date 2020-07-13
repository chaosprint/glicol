export const engine = (actx) => {
    const node = new AudioWorkletNode(actx, 'quaver-engine')
    node.connect(actx.destination)

    fetch('wasm/quaver.wasm')
      .then(response => response.arrayBuffer())
      .then(arrayBuffer => node.port.postMessage({type: "load", obj: arrayBuffer}))
    return node
}