import React, { useRef, useState, useEffect } from 'react'
import { AppBar, Tooltip, Toolbar, Button, IconButton, Drawer, List, ListItem, ListItemText, Divider, Typography } from '@material-ui/core'
import { ThemeProvider } from '@material-ui/styles';
import MenuIcon from '@material-ui/icons/Menu';
import GitHubIcon from '@material-ui/icons/GitHub';
import PlayCircleFilledIcon from '@material-ui/icons/PlayCircleFilled';
import PauseCircleFilledIcon from '@material-ui/icons/PauseCircleFilled';
// import  from '@material-ui/icons';
import clsx from 'clsx';
import { useStyles, theme, buttonTheme, modalStyle} from './styles'
import './App.css'

import { WaveFile } from 'wavefile';
import sampleList from './samples.json';
import {exampleCode} from './example'

import AceEditor from "react-ace";
import "ace-builds/src-noconflict/mode-glicol";
import "ace-builds/src-noconflict/theme-glicol-night";

export default function App() {

  const classes = useStyles();

  const actx = useRef()
  const node = useRef()
  // const [url, setUrl] = useState('alex, 0')
  const [code, setCode] = useState(exampleCode)
  const codeRef = useRef(code)
  // const [isPlaying, setIsPlaying] = useState(false)
  const encoder = new TextEncoder('utf-8');
  const [height, setHeight] = useState(800)
  const [width, setWidth] = useState(600)

  const [sideOpen, setSideOpen] = useState(false)

  const loadModule = async () => {
    // Note the the path is from public folder
    actx.current = new window.AudioContext()
    await actx.current.audioWorklet.addModule('worklet/engine.js')

    node.current = new AudioWorkletNode(actx.current, 'glicol-engine')
    fetch('wasm/glicol_wasm.wasm')
    .then(response => response.arrayBuffer())
    .then(arrayBuffer => node.current.port.postMessage({type: "load", obj: arrayBuffer}))
    node.current.connect(actx.current.destination)

  };

  useEffect(() => {
    loadModule()
    setSize()
    console.log(sampleList)
  }, []);

  const mySamples = ["909", "ab", "insect", "bd", "jazz", "casio",
  "bass", "gtr", "sax", "can", "sf", "fm", "808ht", "808lt", "808hc"]

  const loadSamples = async () => {
      // var sample;
      // let tuple = url.split(",")
      // let key = tuple[0]
      // let sound = sampleList[key][parseInt(tuple[1])];
      // for (const key of Object.keys(sampleList)) {
      for (const key of mySamples) {
        try {
          let sound = sampleList[key][0];

          let u = 'https://raw.githubusercontent.com/chaosprint/Dirt-Samples/master/' + key + '/' + sound
          // let u = "./samples/" + key + '/' + sound
          let myRequest = new Request(u);
          await fetch(myRequest).then(response => response.arrayBuffer())
          .then(arrayBuffer => {
            console.log(arrayBuffer)
            let buffer = new Uint8Array(arrayBuffer)
            let wav = new WaveFile(buffer);
            let sample = wav.getSamples(true, Int16Array)

            // after loading, sent to audioworklet the sample array
            console.log("\\" + key)
            node.current.port.postMessage({type: "samples", sample: sample, name: encoder.encode("\\" + key)})
          });
        } catch(e) {
          console.log(e)
        }
      }
  }
  // const handlePause = () => {
  //   if (isPlaying) {
  //     actx.current.suspend();
  //     setIsPlaying(false)
  //   } else {
  //     actx.current.resume();
  //     setIsPlaying(true)
  //   }
  // }
  const change = (v) => {
    setCode(v)
    codeRef.current = v
  }

  const handleRun = () => {
    actx.current.resume()
    console.log(codeRef.current)
    try {
      node.current.port.postMessage({type: "run", value: encoder.encode(codeRef.current)})
    } catch (e) {
      console.log(e)
    }
  }

  const setSize = () => {
    try {
        let w = document.getElementById('AppBar').offsetWidth;
        let h = window.innerHeight;
        h -= document.getElementById('AppBar').offsetHeight
        setHeight(h)
        setWidth(w)
    } catch {}
  }
  window.onresize = setSize

  const handleUpdate = () => {
    actx.current.resume()
    // console.log(codeRef.current)
    try {
      // node.current.port.postMessage({type: "update", value: encoder.encode(code)})
      node.current.port.postMessage({type: "update", value: encoder.encode(codeRef.current)})
    } catch (e) {
      console.log(e)
    }
  }

  const handlePause = () => {
    actx.current.suspend()
    // console.log(codeRef.current)
  }

  const handleList = (code) => {
    setCode(code);
    setSideOpen(false);
    codeRef.current=code
  }

  
  return (
    <div className='App'>
      <div className="classes.root">
        <ThemeProvider theme={theme}>
        <AppBar position="static" id="AppBar">
        <Toolbar>
          <ThemeProvider theme={buttonTheme}>

          <Tooltip title="Run (cmd + enter / ctrl + enter)">
          {/* <Button
            variant="contained"
            style={{borderRadius:0, fontFamily: 'Inconsolata'}}
            color="primary"
            className={classes.button}
            onClick={handleRun}
          >Run</Button> */}
          <IconButton
            color="inherit"
            aria-label="Play"
            edge="end"
            onClick={handleRun}
            className={clsx(sideOpen && classes.hide)}
          >
            <PlayCircleFilledIcon  fontSize="large" />
          </IconButton>
          </Tooltip>

          <Tooltip title="Pause">

          {/* <Button
            variant="contained"
            style={{borderRadius:0, fontFamily: 'Inconsolata'}}
            color="secondary"
            className={classes.button}
            onClick={handlePause}
          >Pause</Button> */}
          <IconButton
            color="inherit"
            aria-label="Pause"
            edge="end"
            onClick={handleRun}
            className={clsx(sideOpen && classes.hide)}
          >
            <PauseCircleFilledIcon fontSize="large" />
          </IconButton>
          </Tooltip>

          {/* <Button
            variant="contained"
            style={{borderRadius:0, fontFamily: 'Inconsolata'}}
            color="primary"
            onClick={loadSamples}
          >Load</Button>*/}
          </ThemeProvider> 

          <div className={classes.menu}>
          <IconButton
            color="inherit"
            aria-label="open drawer"
            edge="end"
            onClick={()=>setSideOpen(true)}
            className={clsx(sideOpen && classes.hide)}
          >
            <MenuIcon />
          </IconButton>
          </div>
        </Toolbar> 
        </AppBar>
        </ThemeProvider>

                {/* <Button
          variant="contained"
          style={{borderRadius:0, fontFamily: 'Inconsolata'}}
          color={isPlaying? "secondary" : "primary"}
          onClick={handlePause}
        >{isPlaying? "Off": "On"}</Button> */}

        {/* <Button
          variant="contained"
          color="default"
          style={{borderRadius:0, fontFamily: 'Inconsolata'}}
          onClick={handleRun}
        >Run</Button> */}

        <AceEditor
          mode="glicol"
          theme="tomorrow-night"
          fontSize = {18}
          height = {height+"px"}
          width = {width+"px"}
          fontFamily = "Inconsolata"
          value = {code}
          onChange={change}
          name="UNIQUE_ID_OF_DIV"
          editorProps={{ $blockScrolling: true }}
          commands={[{   // commands is array of key bindings.
            name: 'Run', //name for the key binding.
            bindKey: {win: 'Ctrl-Enter', mac: 'Command-Enter'}, //key combination used for the command.
            exec: handleRun  //function to execute when keys are pressed.
          }, {   // commands is array of key bindings.
            name: 'Update', //name for the key binding.
            bindKey: {win: 'Shift-Enter', mac: 'Shift-Enter'}, //key combination used for the command.
            exec: handleUpdate  //function to execute when keys are pressed.
          }]}
        />

      <Drawer
        className={classes.drawer}
        // variant="persistent"
        anchor="right"
        open={sideOpen}
        onClose={()=>setSideOpen(false)}
        classes={{
          paper: classes.drawerPaper,
        }}
      >
        <Toolbar>
        <div className="classes.text">
          {/* <h3>GLiCoL</h3> */}
          <p>v0.1.0</p>
        </div>
        <div className={classes.menu}>
          <IconButton
            href="https://github.com/gilcol/"
            target="_blank"
            rel="noopener noreferrer"
            // data-show-count="true"
            aria-label="GitHub"
            color="inherit"
            // aria-label="open drawer"
            edge="end"
          >
            <GitHubIcon />
        </IconButton>
        </div>
        </Toolbar>
        <Divider />
        {/* <div className={classes.drawerHeader}>
          <IconButton onClick={()=>setSideOpen(false)}>
            <MenuIcon />
            {/* {theme.direction === 'rtl' ? <ChevronLeftIcon /> : <ChevronRightIcon />} */}
          {/* </IconButton>  */}
        {/* </div> */}
        {/* <Divider /> */}
        <div className="classes.text">
        
        <List>
          <ListItem
            button
            key="Hello"
            onClick={()=>{
              let code = "~hi: sin 440.0"
              setCode(code);
              setSideOpen(false);
              codeRef.current=exampleCode}}
          ><ListItemText
          primary={<Typography style={{ fontFamily: '\'Inconsolata\', monospace'}}>hello world.</Typography>}
          /></ListItem>
        </List>

        <List>
          <ListItem
            button
            key="Hello"
            onClick={()=>{
              let code = "~hi: sin 440.0 >> mul 0.1"
              setCode(code);
              setSideOpen(false);
              codeRef.current=code}}
          ><ListItemText
          primary={<Typography style={{ fontFamily: '\'Inconsolata\', monospace'}}>amp control.</Typography>}
          /></ListItem>
        </List>

        <List>
          <ListItem
            button
            key="Hello"
            onClick={()=>{
              let code = "~hi: sin 440.0 >> mul &am\n\n&am: sin 0.2 >> mul 0.3 >> add 0.5"
              setCode(code);
              setSideOpen(false);
              codeRef.current=code}}
          ><ListItemText
          primary={<Typography style={{ fontFamily: '\'Inconsolata\', monospace'}}>amp modulation.</Typography>}
          /></ListItem>
        </List>

        <Divider />
        <List>
          <ListItem
            button
            key="Hello"
            onClick={()=>{handleList("~hi: sin 440.0 >> mul &am\n\n&am: sin 0.2 >> mul 0.3 >> add 0.5")}}
          ><ListItemText
          primary={<Typography style={{ fontFamily: '\'Inconsolata\', monospace'}}>use samples.</Typography>}
          /></ListItem>
        </List>
        <Divider />
        <List>
          <ListItem
            button
            key="Hello"
            onClick={()=>{setCode(exampleCode);  setSideOpen(false); codeRef.current=exampleCode}}
          ><ListItemText
          primary={<Typography style={{ fontFamily: '\'Inconsolata\', monospace'}}>all together.</Typography>}
          /></ListItem>
        </List>
        <Divider />
        <List>
          <ListItem button key="sin" onClick={()=>{let c = "~sin: sin 110.0"; setCode(c);  setSideOpen(false); codeRef.current = c}}>
            <ListItemText className={{primary:classes.text}}
              primary={<Typography style={{ fontFamily: '\'Inconsolata\', monospace'}}>template - synthesis.</Typography>}
            ></ListItemText>
            </ListItem>
        </List>
        <List>
          <ListItem
          button key="sample"
          onClick={()=>{let c = "~bd: loop 60 >> sampler \\bd"; loadSamples(); setCode(c); setSideOpen(false); codeRef.current = c}}>
            <ListItemText
            className={{primary:classes.text}}
            primary={<Typography style={{ fontFamily: '\'Inconsolata\', monospace'}}>template - samples.</Typography>}

            />
            </ListItem>
        </List>
        </div>
      </Drawer>

      </div>
    </div>
  );
}