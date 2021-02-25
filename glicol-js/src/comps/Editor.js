import React, { useState, useEffect } from 'react'
import './Editor.css'
import { useParams } from "react-router-dom";
import firebaseConfig from './firebaseConfig'
import LockIcon from '@material-ui/icons/Lock';
import {Modal, TextField, Fab, ThemeProvider} from '@material-ui/core/'
import { useStyles, theme } from '../styles/styles';

export default function Editor(props) {
    const classes = useStyles();

    const [open, setOpen] = useState(false)
    const [pwd, setPwd] = useState("")
    const [locked, setLocked] = useState(true)
    
    const handleModalClose = () => {setOpen(false)}

    const getExampleRef = (roomID) => {
        var ref = window.firebase.database().ref();   
        ref = ref.child(roomID)
        return ref;
    }

    var { id } = useParams();

    const sumbitPassword = e => {
        e.preventDefault()
        window.firebase.auth().signInWithEmailAndPassword(id+"@glicol.web.app", pwd).then(cred => {
            setOpen(false)
            setLocked(false)
            window.editor.setOptions({
                readOnly: false
            })
            let c = document.getElementById("cover")
            window.editor.container.removeChild(c)
        }).catch( e=>{alert("Errors");console.log(e)})
    }
    
    const setSize = () => {
        let w = document.getElementById('AppBar').offsetWidth
        let border =  document.documentElement.clientWidth - w
        let h = document.documentElement.clientHeight
        h = h - document.getElementById('AppBar').offsetHeight - border
        window.editor.container.style.width = `${w}px`
        window.editor.container.style.height = `${h}px`
        window.editor.resize()
    }

    useEffect(()=>{
        try {
            window.firepad.dispose()
        } catch {}

        if (!window.firebase.apps.length) {
            window.firebase.initializeApp(firebaseConfig);

            // try {
            //     window.firebase.auth().signInAnonymously()
            //     .then(() => {
            //     })
            //     .catch((error) => {
            //         console.log(error.code);
            //         console.log(error.message);
            //     });
            // } catch {}
        }


        // console.log(id)
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
            bindKey: {win: 'Ctrl-Shift-/', mac: 'Command-Shift-/'},
            exec: ()=>{
                var pos = window.editor.getCursorPosition();
                // var sel = window.editor.getSelectedText();
                var token = window.editor.session.getTokenAt(pos.row, pos.column).value;

                window.help(token);
            }
        }]

        command.forEach(c => window.editor.commands.addCommand(c))


        try {
            window.firebase.auth().signInAnonymously()
            .then(() => {
                var firepadRef = getExampleRef(id);
                window.firepad = window.Firepad.fromACE(firepadRef, window.editor,
                    { richTextToolbar: false, richTextShortcuts: false});
                window.firepad.on('ready', () => {
                    try {
                        var cover = document.createElement("div")
                        cover.setAttribute("id", "cover");
                        window.editor.container.appendChild(cover)
                        cover.style.cssText = `position:absolute;
                            top:0;bottom:0;right:0;left:0;
                            background:rgba(150,150,150,0.1);
                            z-index:9`
                            cover.addEventListener("mousedown", e=>{
                                e.stopPropagation()}, true)
                        window.editor.setOptions({
                            readOnly: true,
                        })
                        if (window.opennew) {
                            window.editor.setValue(window.code)
                            window.opennew = false
                        }
                        setSize()
                    } catch(e) {console.log(e)}
                });
            })
            .catch((error) => {
                console.log(error.code);
                console.log(error.message);
            });
        } catch {}

    // eslint-disable-next-line
    }, [id])

    return(
        <div>
            <div id="firepad"></div>
            <ThemeProvider theme={theme}>
            {
                locked ? (<Fab aria-label="add"
                style={{
                    position: "absolute",
                    right: 50,
                    bottom: 50,
                    zIndex: 10,
                    color: "black"
                }}
                onClick={()=>{setOpen(true)}}>
                    <LockIcon />
                </Fab>) : (<></>)
            }

            
            <Modal
            id="modal"
            className={classes.modal}
            open={open}
            onClose={handleModalClose}
            onRendered={() => document.getElementById("unlockpwd").focus()}
            >
            <div className={classes.paper}>
                <p>Enter the password to edit:</p>
                <form onSubmit={sumbitPassword}>
                <TextField
                    id="unlockpwd"
                    label="Password"
                    type="password"
                    variant="filled"
                    margin="normal"
                    value={pwd}
                    onChange={e=>setPwd(e.target.value)}
                />

                </form>
            </div>
            </Modal>
            </ ThemeProvider>
        </div>
    )
}