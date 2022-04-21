const isRef = s => String(s).includes('~')

export function sin(freq) {
  if (!isNaN(freq) || isRef(freq)) {
    return new NodeChain(`sin ${freq}`)
  }
}

export function saw(freq) {
  if (!isNaN(freq) || isRef(freq)) {
    return new NodeChain(`saw ${freq}`)
  }
}
export function tri(freq) {
  if (!isNaN(freq) || isRef(freq)) {
    return new NodeChain(`tri ${freq}`)
  }
}

export function squ(freq) {
  if (!isNaN(freq) || isRef(freq)) {
    return new NodeChain(`squ ${freq}`)
  }
}

export function imp(freq) {
  if (!isNaN(freq)) {
    return new NodeChain(`imp ${freq}`)
  }
}

export function noise(seed) {
  if (!isNaN(seed)) {
    return new NodeChain(`noise ${seed}`)
  }
}

export function speed(val) {
  if (!isNaN(val)) {
    return new NodeChain(`speed ${val}`)
  }
}

export function seq(str) {
  // if (!isNaN(seed)) {
  return new NodeChain(`seq ${str}`)
  // }
}

export function psynth(str, span) {
  if (!isNaN(span)) {
    return new NodeChain(`p_synth \`${str} ${span}`)
  }
}

export function psampler(str) {
  return new NodeChain(`psampler ${str}`)
}

export function sig(param) {
    return new NodeChain(`constsig ${param}`)
}

export function mix(str) {
  // var result;
  // if (typeof str === "Array") {
  //   result = str.join(" ")
  // } else if (typeof str === "String") {
  //   result = str
  // }
  return new NodeChain(`mix ${str}`)
}

export class NodeChain {
  constructor(code) {
    this.code = code
  }

  toString() {
    return `${this.code}`;
  };

  mul(val) {
    if (!isNaN(val) || isRef(val)) {
      this.code += ` >> mul ${val}`
    }
    return this
  }
  add(val) {
    if (!isNaN(val) || isRef(val)) {
      this.code += ` >> add ${val}`
    }
    return this
  }

  delayms(val) {
    if (!isNaN(val) || isRef(val)) {
      this.code += ` >> delayms ${val}`
    }
    return this
  }

  delayn(val) {
    if (!isNaN(val)) {
      this.code += ` >> delayn ${parseInt(val)}`
    }
    return this
  }

  lpf(cutoff, qvalue) {
    // if ( (!isNaN(cutoff) || isRef(cutoff)) &&  (!isNaN(qvalue))) {
    this.code += ` >> lpf ${cutoff} ${qvalue}`
    // }
    return this
  }

  hpf(cutoff, qvalue) {
    if ( (!isNaN(cutoff) || isRef(cutoff)) &&  (!isNaN(qvalue))) {
      this.code += ` >> hpf ${cutoff} ${qvalue}`
    }
    return this
  }

  plate(val) {
    if (!isNaN(val)) {
      this.code += ` >> plate ${val}`
    }
    return this
  }

  bd(val) {
    if (!isNaN(val)) {
      this.code += ` >> bd ${val}`
    }
    return this
  }

  sn(val) {
    if (!isNaN(val)) {
      this.code += ` >> sn ${val}`
    }
    return this
  }

  hh(val) {
    if (!isNaN(val)) {
      this.code += ` >> hh ${val}`
    }
    return this
  }

  sawsynth(att, dec) {
    if (!isNaN(att) && !isNaN(dec)) {
      this.code += ` >> sawsynth ${att} ${dec}`
    }
    return this
  }

  squsynth(att, dec) {
    if (!isNaN(att) && !isNaN(dec)) {
      this.code += ` >> squsynth ${att} ${dec}`
    }
    return this
  }

  trisynth(att, dec) {
    if (!isNaN(att) && !isNaN(dec)) {
      this.code += ` >> trisynth ${att} ${dec}`
    }
    return this
  }

  seq(str) {
    // if (!isNaN(str)) {
    this.code += ` >> seq ${str}`
    // }
    return this
  }

  adsr(a, d, s, r) {
    // if (!isNaN(str)) {
    this.code += ` >> adsr ${a} ${d} ${s} ${r}`
    // }
    return this
  }

  sp(sampleName) {
    // if (!isNaN(str)) {
    this.code += ` >> sp \\${sampleName}`
    // }
    return this
  }

  envperc(attack, decay) {
    // if (!isNaN(str)) {
    this.code += ` >> envperc ${attack} ${decay}`
    // }
    return this
  }

}