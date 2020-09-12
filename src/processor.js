export const engine = (actx) => {
    const node = new AudioWorkletNode(actx, 'quaver-engine')
    node.connect(actx.destination)

    fetch('wasm/quaverseries_rs.wasm')
      .then(response => response.arrayBuffer())
      .then(arrayBuffer => node.port.postMessage({type: "load", obj: arrayBuffer}))
    return node
}

// no need here