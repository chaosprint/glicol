import React, { useEffect } from 'react'
import firebaseConfig from './firebaseConfig'
import './Editor.css'
import { useParams } from "react-router-dom";
// import { sound, control, calc } from './helpers'

export default function Editor(props) {

    const getExampleRef = (roomID) => {
        // console.log(roomID, "called")
        var ref = window.firebase.database().ref();   
        ref = ref.child(roomID)
        return ref;
    }

    var { id } = useParams();
    
    const setSize = () => {
        let w = document.getElementById('AppBar').offsetWidth
        let border =  document.documentElement.clientWidth - w
        let h = document.documentElement.clientHeight
        h = h - document.getElementById('AppBar').offsetHeight - border
        window.editor.container.style.width = `${w}px`
        window.editor.container.style.height = `${h}px`
        window.editor.resize()
    }

    // window.onresize = setSize

    useEffect(()=>{
            try {
                window.firepad.dispose()
            } catch {}
            // console.log(id)
            if (!window.firebase.apps.length) {
                window.firebase.initializeApp(firebaseConfig);
            }
            //// Create ACE
            window.editor = window.ace.edit("firepad");
            window.editor.setValue("")// has to be here, else cause error
            window.editor.setFontSize("20px");
            window.editor.setTheme("ace/theme/tomorrow-night")
            window.editor.setOptions({
                fontFamily: "B612 Mono",
                // readOnly: true,
                "highlightActiveLine": false
            })
            window.editor.resize()
            // window.editor.onChange(()=>{console.log("change")})
            // editor.setTheme("ace/theme/textmate");
            var session =  window.editor.getSession();
            session.setUseWrapMode(true);
            session.setUseWorker(true);
            session.setMode("ace/mode/glicol");
            session.on("change", () => window.code = window.editor.getValue())

            const command = [{
                name: 'run',
                bindKey: {win: 'Ctrl-Enter', mac: 'Command-Enter'},
                exec: props.handleRun
            }, {
                name: 'update',
                bindKey: {win: 'Shift-Enter', mac: 'Shift-Enter'},
                exec: props.handleUpdate
            }, {
                name: 'stop',
                bindKey: {win: 'Ctrl-Alt-.', mac: 'Command-Option-.'},
                exec: props.handleStop
            }, {
                name: 'pause',
                bindKey: {win: 'Ctrl-\'', mac: 'Command-\''},
                exec: props.handlePause
            }, {
                name: 'help',
                bindKey: {win: 'Ctrl-\\', mac: 'Command-\\'},
                exec: ()=>{
                    var pos = window.editor.getCursorPosition();
                    // var sel = window.editor.getSelectedText();
                    var token = window.editor.session.getTokenAt(pos.row, pos.column).value;

                    if (token in window.docs.about) {
                        console.log(`%c${window.docs.about[token]}`, "background: blue")
                        console.table(window.docs.table[token])
                        console.table(window.docs.range[token])
                        console.log("%cEXAMPLE", "background: green; color: white")
                        window.docs.example[token]()
                    } else {
                        console.error(`Move your cursor to an non-empty place where you wish to search.
                        \nFor example, if you wish to search "sin", your cursor should be inside "sin" like this: s|in`)
                    }
                }
            }]

            command.forEach(c => window.editor.commands.addCommand(c))

            var firepadRef = getExampleRef(id);
            try {
                // window.code = window.editor.getValue()
                // console.log( window.code)
                // window.editor.setValue("")
                window.firepad = window.Firepad.fromACE(firepadRef, window.editor,
                    { richTextToolbar: false, richTextShortcuts: false});
            } catch (e) {
                // console.log(e)
                console.warn("please refresh the page")
            }
            setSize()
            // console.log("editor loaded")
    // eslint-disable-next-line
    }, [id])

    return(
        <div>
            <div id="firepad"></div> 
        </div>
    )
}