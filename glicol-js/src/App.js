import './App.css'
import { BrowserRouter as Router, Switch, Route } from "react-router-dom";
import React, { useRef, useState, useEffect } from 'react'
import { AppBar, Toolbar, IconButton } from '@material-ui/core'
import { Drawer, Divider, Typography, Modal, Tooltip } from '@material-ui/core'
import { FormGroup, FormControlLabel, Switch as IO} from '@material-ui/core'
import { ThemeProvider } from '@material-ui/styles';
import GitHubIcon from '@material-ui/icons/GitHub';
import SettingsIcon from '@material-ui/icons/Settings';

// import clsx from 'clsx';
import { useStyles, theme } from './styles/styles'
import {Run, Reset, Pause, Menu, Update, Fork } from './comps/ToolButton'
import MyList from "./comps/MyList"

import handleError from './utils/handleError'
import { WaveFile } from 'wavefile';
import sampleDict from './utils/samples.json';
import {sampleList} from './utils/samples.js';
import {hello, am, fm, usesample, envelope, filter, demo2, demo1, intro} from './utils/examples'
import './utils/consoleCommands'
// import {loadModule} from './utils/audioLoader'

import Editor from './comps/Editor'
import ForkStepper from './comps/Fork'
import { CodeContext } from './Context'

import AceEditor from "react-ace";
import "./styles/mode-glicol";
import "./styles/theme-glicol-night";

export default function App() {

  const classes = useStyles();
  const encoder = new TextEncoder('utf-8');
  const codeRef = useRef(intro)
  const [code, setCode] = useState(intro)
  const [height, setHeight] = useState(800)
  const [width, setWidth] = useState(600)
  const [running, setRunning] = useState(false)
  const [prog, setProg] = useState(0)
  const [loading, setLoading] = useState(false)
  const [loaded, setLoaded] = useState(false)
  const [sideOpen, setSideOpen] = useState(false)
  const [settingsOpen, setSettingsOpen] = useState(false)
  const [forkOpen, setForkOpen] = useState(false)
  const [useSamples, setUseSamples] = useState(false)

  useEffect(() => {
    resize()
    try {
      // loadModule()
    } catch (e) {console.log(e)}
    try {
      window.firebase.auth().signInAnonymously()
      .then(() => {
      })
      .catch((error) => {
          console.log(error.code);
          console.log(error.message);
      });
    } catch {}
  }, []);

  const loadSamples = async (list) => {
    console.clear()
    console.log("\n\navailable samples:")
    console.log(list.sort())
    let tempcode = window.code
    setLoading(true)
    window.actx.suspend()
    let l = list.length
    let count = l
    for (const key of list) {
      setProg((l-count)/l*100)
      count -= 1
      try {
        let sound = sampleDict[key][0];
        let u =
        'https://raw.githubusercontent.com/chaosprint/Dirt-Samples/master/' 
        + key + '/' + sound
        let myRequest = new Request(u);
        await fetch(myRequest).then(response => response.arrayBuffer())
        .then(arrayBuffer => {
          // console.log("downloaded", arrayBuffer)
          let buffer = new Uint8Array(arrayBuffer)
          let wav = new WaveFile(buffer);
          let sample = wav.getSamples(true, Int16Array)
          window.node.port.postMessage({
            type: "samples",
            sample: sample,
            name: encoder.encode("\\" + key)
          })
        });
      } catch(e) {}
    }
    setLoading(false)
    setLoaded(true)
    window.code = tempcode
  }

  const change = (v) => {
    setCode(v)
    window.code = v
  }

  const resize = () => {
    // console.log("set size")
    // if (window.state === "coding") {
      try {
        let w = document.getElementById('AppBar').offsetWidth
        let border =  document.documentElement.clientWidth - w
        let h = document.documentElement.clientHeight
        h = h - document.getElementById('AppBar').offsetHeight - border
        window.editor.container.style.width = `${w}px`
        window.editor.container.style.height = `${h}px`
        window.editor.resize()
      } catch (e) {}
    // } else {
      try {
        let w = window.innerWidth;
        let h = window.innerHeight;
        h -= document.getElementById('AppBar').offsetHeight
        setHeight(h)
        setWidth(w)
      } catch (e) {}
    // }
  }
  window.onresize = resize

  const handleUpdate = () => {
    
    setRunning(true)
    try {
      window.actx.resume()
      window.node.port.postMessage({
        type: "update",
        value: encoder.encode(window.code?window.code:"")
      })

      console.log(encoder.encode(window.code?window.code:"").length)
      window.paramWriter.enqueue_change(0, encoder.encode(window.code?window.code:"")[0])

      window.node.onmessage = (event) => {
        // Handling data from the processor.
        console.log(event);
      };
    } catch (e) {
      console.log(e)
    }
  }

  const handleRun = () => {

    window.node.port.onmessage = handleError;
    try {
      window.actx.suspend()
      window.actx.resume()
      setRunning(true)
    } catch (e) {
      console.log(e)
    }
    try {
      window.node.port.postMessage({
        type: "run",
        value: encoder.encode(window.code?window.code:"")
      })
    } catch (e) {
      console.log(e)
    }
  }

  const handlePause = () => {
    window.actx.suspend()
    setRunning(false)
  }

  const handleStop = () => {
    console.log("%cRestarting Glicol server...", "background: pink; font-weight: bold")
    let codetemp = window.code
    try {
      window.actx.close();
      window.loadModule();
      setRunning(false)
      setLoaded(false)
      setUseSamples(false)
    } catch (e) {
      console.log(e)
    }
    // console.log("stop") 
    window.code = codetemp
  }

  const handleSettings = () => { setSettingsOpen(true) }

  const handleSettingsClose = () => {setSettingsOpen(false)}

  const handleFork = () => {setForkOpen(true)}

  const handleForkClose = () => { setForkOpen(false) }

  const handleUseSamples = (e) => {
    setUseSamples(e.target.checked)
    // console.log(e.target.checked)
    if (e.target.checked && !loaded) {
      loadSamples(sampleList.selected)
    }
  }

  const handleList = async (code) => {
    // setShowTutorial(true)
    // history.push("/")
    // window.editor.destroy();
    // console.log("should go to turorial")
    setCode(code);
    window.code = code
    resize()
    setSideOpen(false);
    codeRef.current=code
    setRunning(false)
    // handleStop();
    // window.actx.close();
    // await loadModule();
    // loadSamples(list)
  }

  return (
    <Router>
    <CodeContext.Provider value={{code, setCode}}>
    <div className="App">
        <ThemeProvider theme={theme}>
        <AppBar
          className={classes.appBar}
          id="AppBar"
        >
        <Toolbar>
        <div className={classes.menu} ><Menu onClick = {()=>setSideOpen(true)} /></div>
        <div id="logo"><h2><a href="/">GLICOL</a></h2></div>
        
        <div id="control">
        {loading ?
        <Typography className={classes.text}
        >loading samples...[
          {Math.floor(prog)}%] </Typography>
         : <div>
        {!running ? <Run onClick={handleRun}/> :
        (<Pause onClick={handlePause}/> )}
        <Update onClick={handleUpdate} />
        <Reset onClick={handleStop} />
        

        <Tooltip title="settings">
        <IconButton
          // color="inherit"
          edge="end"
          onClick={handleSettings}
        >
        <SettingsIcon fontSize="large" />
        </IconButton>
        </Tooltip>
        <Fork onClick={handleFork} />
        {/* <Help onClick={handleHelp} /> */}
        
       </div>}
       </div>
      
        <Drawer
          className={classes.drawer}
          // variant="persistent"
          anchor="left"
          open={sideOpen}
          onClose={()=>setSideOpen(false)}
          classes={{
            paper: classes.drawerPaper,
          }}
        >
        <Toolbar>
        <Typography>v0.1.0</Typography>
        <IconButton
          href="https://github.com/chaosprint/glicol/"
          target="_blank"
          rel="noopener noreferrer"
          // color="inherit"
          style={{marginLeft: 'auto'}}
        ><GitHubIcon /></IconButton>
        </Toolbar>

        <Divider />

        <MyList onClick={()=>handleList(hello)} title="hello world." />
        <MyList onClick={()=>handleList(am)} title="am." />
        <MyList onClick={()=>handleList(fm)} title="fm." />
        <Divider />
        <MyList onClick={()=>{handleList(usesample)}}
          title="use samples." />
        <MyList onClick={()=>handleList(envelope)} title="envelope." />
        <Divider />
        <MyList onClick={()=>handleList(filter)} title="filter." />

        <Divider />
        <MyList onClick={()=>{handleList(demo1)}} title="demo 1." />
        <MyList onClick={()=>{handleList(demo2)}} title="demo 2." />
        </Drawer>

        </Toolbar> 
        </AppBar>
        <Toolbar />

        <Modal
          className={classes.modal}
          open={forkOpen}
          onClose={handleForkClose}
          
          onRendered={() => {document.getElementById("password-input").focus()}}
        >
        <div className={classes.paper}>
          <ForkStepper />
        </div>
        </Modal>
        
        <Modal
          className={classes.modal}
          open={settingsOpen}
          onClose={handleSettingsClose}
          // onRendered={() => modalRef.current.children[1].children[0].focus()}
          // BackdropComponent={Backdrop}
        >
        <div className={classes.paper}>
        <FormGroup>
        <FormControlLabel
          control={
            <IO
              checked={useSamples}
              onChange={handleUseSamples}
              name="useSamples"
              // color="inherit"
            />
          }
          label="use samples?"
          labelPlacement="start"
        />
        </FormGroup>
        </div>
        </Modal>
        </ThemeProvider>
     </div>

      <Switch>
        <Route exact path="/" children={
            <div>
              <AceEditor
                className={classes.editor}
                mode="glicol"
                theme="tomorrow-night"
                fontSize = {20}
                height = {height+"px"}
                width = {width+"px"}
                // style={{ height: "100%", width: "100%"}}
                fontFamily = "B612 Mono"
                value = {code}
                onChange={change}
                name="UNIQUE_ID_OF_DIV"
                highlightActiveLine={false}
                editorProps={{ $blockScrolling: true }}
                setOptions={{
                  useWorker: false // <<----- USE THIS OPTION TO DISABLE THE SYNTAX CHECKER
                }}
                commands={[{   // commands is array of key bindings.
                  name: 'Run', //name for the key binding.
                  bindKey: {win: 'Ctrl-Enter', mac: 'Command-Enter'},
                  exec: handleRun  //function to execute when keys are pressed.
                }, {
                  name: 'Update',
                  bindKey: {win: 'Shift-Enter', mac: 'Shift-Enter'},
                  exec: handleUpdate
                }, {
                  name: 'Stop',
                  bindKey: {win: 'Ctrl-Shift-.', mac: 'Command-Shift-.'},
                  exec: handleStop
                }, {
                  name: 'Help',
                  bindKey: {win: 'Ctrl-Shift-/', mac: 'Command-Shift-/'},
                  exec: (e)=>{
                    let pos = e.getCursorPosition()
                    let token = e.session.getTokenAt(pos.row, pos.column).value;
                    window.help(token);
                  }
                }]}
              />
            </div> }
        />
        <Route path="/:id" children={<Editor
          handleRun={handleRun}
          handleUpdate={handleUpdate}
          handleStop={handleStop}
          handlePause={handlePause}
        />} />
      </Switch>
    </CodeContext.Provider>
     </Router>
  )
}