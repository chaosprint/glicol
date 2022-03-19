export default {
  // The start state contains the rules that are initially used
  start: [
    {regex: /(delayms|true|false|let|const|else|switch|do|loop|until|continue|break|fn|this|return|throw|try|catch|import|export|as|global|print|debug|eval|map|for|while|if|sin|exp|in|expr|seq|squsynth|sawsynth|trisynth|bd|hh|sn|speed|choose|mul|add|linrange|apfdecay|delayn|sin|saw|squ|imp|envperc|sampler|noiz|shape|tri|noise|noiz|rlpf|plate|onepole|rhpf|pha|buf|state|freeverb|pan|delay|apfgain|lpf|hpf|comb|mix|monosum|const_sig|sp|spd|amplfo|balance|meta|script|pad|in)(?![a-z])/,
     token: "string"},
    // The regex matches the token, the token property contains the type
    // {regex: /~([a-z]+(_)?)+/, token: "variable-3"},
    // You can match multiple tokens at once. Note that the captured
    // groups must span the whole string in this case
    // {regex:  /##([\S\n\t\v ]+?)#/, token: "error"},
    {regex: /[-+]?([0-9]{1,}[.][0-9]+)/, token: "variable"},
    {regex: /PI/, token: "variable"},
    // {regex: /\\(\S)*/, token: "number"},
    {regex: /([0-9]{1,3}|(~[a-z](?![a-z0-9\.]))|_)+/, token: "variable"},
    // {regex: /(~)([a-z])(?!([a-z]))/, token: "variable"},
    {regex: /^~[a-z][0-9a-z\_\.]+/, token: "variable-2"},
    

    {regex: /\/\/.*/, token: "comment"},
    // {regex: /`[\s\S\n\t]+`/, token: "meta"},
    {regex: /\}|\{|\;|\`|\-|\/|:|>>|,|\*|\+|\=|\||\(|\)/, token: "error"},
    {regex: /^\\[0-9a-z\_]+/, token: "variable"},
    {regex: /^[a-z][0-9a-z\_]*/, token: "keyword"},
    // A next property will cause the mode to move to a different state
    {regex: /\/\*/, token: "comment", next: "comment"},
    // {regex: /[-+\/*=<>!]+/, token: "operator"},
    // indent and dedent properties guide autoindentation
    {regex: /[\{\[\(]/, indent: true},
    {regex: /[\}\]\)]/, dedent: true},
    {regex: /##/, token: "error", next: "error"}, //js is error
    // {regex: /`/, token: "meta", next: "meta"}, //js is error
    // {regex:  /(?<=##)[^#]*(?=#)/, token: "meta"}
  ],
  // The multi-line comment state.
  comment: [
    {regex: /.*?\*\//, token: "comment", next: "start"},
    {regex: /.*/, token: "comment"}
  ],
  // meta: [
  //   {regex: /.*?`/, token: "meta", next: "start"},
  //   {regex: /.*/, token: "meta"}
  // ],
  error: [
    {regex: /.*?#/, token: "error", next: "start"},
    {regex: /.*/, token: "error"}
  ],
  // The meta property contains global information about the mode. It
  // can contain properties like lineComment, which are supported by
  // all modes, and also directives like dontIndentStates, which are
  // specific to simple modes.
  meta: {
    dontIndentStates: ["comment"],
    lineComment: "//"
  }
};