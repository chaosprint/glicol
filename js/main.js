import mode from './src/glicol-mode'
import './neo.css'
import './style.css'
let myTextarea =  document.getElementById("code");
CodeMirror.defineSimpleMode("simplemode", mode);
window.editor = CodeMirror.fromTextArea(myTextarea, {
    lineNumbers: true,
    theme: "neo",
    extraKeys: {
        "Ctrl-Enter": function(cm) {
            window.run(cm.getValue())
        },
        "Ctrl-Shift-Enter": function(cm) {
            window.run(cm.getValue())
            cm.setValue(window.code)
        },
        "Cmd-Enter": function(cm) {
            window.run(cm.getValue())
        },
        "Cmd-Shift-Enter": function(cm) {
            window.run(cm.getValue())
            cm.setValue(window.code)
        },
        "Alt-D": function(editor) {
            let A1 = editor.getCursor().line;
            let A2 = editor.getCursor().ch;
            let B1 = editor.findWordAt({line: A1, ch: A2}).anchor.ch;
            let B2 = editor.findWordAt({line: A1, ch: A2}).head.ch;
            window.help(editor.getRange({line: A1,ch: B1}, {line: A1,ch: B2}))
        },
        "Ctrl-Alt-.": function() {
            window.stop()
        },
        "Cmd-Alt-.": function() {
            window.stop()
        },
        'Ctrl-/': cm => {cm.execCommand('toggleComment')},
        'Cmd-/': cm => {cm.execCommand('toggleComment')}
    }

});
editor.setValue(window.code)
document.getElementById("run").addEventListener("click", ()=>{
    // let currentCode = document.getElementById("code").value;
    window.run(window.editor.getValue())
})
document.getElementById("stop").addEventListener("click", ()=>{
    window.stop(); // stop function binding to window
})