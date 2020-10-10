// this file should be copied to node_modules/brace/mode

ace.define("ace/mode/glicol_highlight_rules", ["require", "exports", "module", "ace/lib/oop", "ace/mode/text_highlight_rules"], function (acequire, exports, module) {
  "use strict";

  var oop = acequire("../lib/oop");
  
  // var DocCommentHighlightRules = acequire("./doc_comment_highlight_rules").DocCommentHighlightRules;

  var TextHighlightRules = acequire("./text_highlight_rules").TextHighlightRules;

  var GlicolHighlightRules = function GlicolHighlightRules() {

    var keywordControls = "seq|loop|bpm|line|shift|every|speed|choose|range|play|set_gate|set_gate_all|midi_out|mul|add|envperc|linrange";

    var storageType = "sin|saw|squ|imp|pwm|brown|white|pink|noiz|membrane|sin_synth|saw_synth|squ_synth|tri_synth|sampler|pluck|metalphone|fm_synth|lfo|sin_lfo|tri_lfo|saw_lfo|squ_lfo|pink_noise|brown_noise|white_noise"

    var storageModifiers = "";
    var keywordOperators = ">>|->|="
    var builtinConstants = "amp|pan|bpf|bnf|rlpf|rhpf|lpf|hpf|reverb|pingpong|jcreverb|freeverb|delay|"

    var keywordMapper = this.$keywords = this.createKeywordMapper({
        "keyword.control" : keywordControls,
        "storage.type" : storageType,
        "storage.modifier" : storageModifiers,
        "keyword.operator" : keywordOperators,
        // "variable.language": "this",
        "constant.language": builtinConstants
    }, "identifier");
    this.$rules = {
      "start": [{
        token: "comment",
        regex: "//$",
        next: "start"
      }, {
        token: "invisible",
        regex: ">>|:"
      }, {
        token: "comment",
        regex: "//",
        next: "singleLineComment"
      }, {
        token: "variable.parameter", // compound note
        regex: "(((((_)+)?((~|&)[a-z])((_)+)?)+)|(_))(\\s|\\n|(~|&))?\\b"
      }, {
        token : "support.constant",
        regex : ","
      }, {
        token: "meta.tag", // float
        regex: "[-+]?([0-9]{1,}[\.][0-9]+)"
        // regex : "[+-]?\\d+(?:(?:\\.\\d*)?(?:[eE][+-]?\\d+)?)?(?:L|l|UL|ul|u|U|F|f|ll|LL|ull|ULL)?\\b"
      }, {
        token : "constant.character",
        regex : "(((((_)+)?([0-9]+)((_)+)?)+)|(_))(\\s|\\n|~)?\\b"
      }, {
        token : "support.type", // "\8n" now "everything" 
        regex : "\\\\(([0-9]+)?([a-z]+)?(_)?([0-9]+)?)+\\b"
        // regex : "\\\\([0-9]{1,2})([a-z]+)\\b"
      }, {
        token : "constant.character", // symbal?
        regex : "\\\\(([0-9]+)?([a-z]+)(_)?([0-9]+)?)+\\b"
      }, {
        token: "string", // ref
        regex: "((~)([a-z]+(_)?)+)\\b"
      }, {
        token: "audio", // ref with _
        regex: "((_|&)([a-z]+(_)?)+)\\b",
      }, {
        token : keywordMapper,
        regex : "[a-zA-Z_$][a-zA-Z0-9_$]*\\b"
      }, {
        token: "text",
        regex: "\\\\s+"
      }],
      
      "singleLineComment": [{
        token: "comment",
        regex: /\\\\$/,
        next: "singleLineComment"
      }, {
        token: "comment",
        regex: /$/,
        next: "start"
      }, {
        defaultToken: "comment"
      }]
    };
  };

  oop.inherits(GlicolHighlightRules, TextHighlightRules);
  exports.GlicolHighlightRules = GlicolHighlightRules;
});
ace.define("ace/mode/glicol", ["require", "exports", "module", "ace/lib/oop", "ace/mode/text", "ace/mode/glicol_highlight_rules"], function (acequire, exports, module) {
  "use strict";

  var oop = acequire("../lib/oop");
  var TextMode = acequire("./text").Mode;
  var GlicolHighlightRules = acequire("./glicol_highlight_rules").GlicolHighlightRules;

  var Mode = function Mode() {
    this.HighlightRules = GlicolHighlightRules;
    this.$behaviour = this.$defaultBehaviour;
  };

  oop.inherits(Mode, TextMode);
  (function () {
    this.lineCommentStart = "//"; // this.blockComment = {start:"//", end:""}

    this.$id = "ace/mode/glicol";
  }).call(Mode.prototype);
  exports.Mode = Mode;
});