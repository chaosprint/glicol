import React, { useRef, useState, useEffect } from 'react'
import { AppBar, Tooltip, Toolbar, IconButton, Drawer, List, ListItem, ListItemText, Divider, Typography } from '@material-ui/core'
import { ThemeProvider } from '@material-ui/styles';
import MenuIcon from '@material-ui/icons/Menu';
import GitHubIcon from '@material-ui/icons/GitHub';
import PlayCircleFilledIcon from '@material-ui/icons/PlayCircleFilled';
import PauseCircleFilledIcon from '@material-ui/icons/PauseCircleFilled';
import PanoramaFishEyeIcon from '@material-ui/icons/PanoramaFishEye';
import clsx from 'clsx';
import { useStyles, theme } from './styles'

// import {readFile} from 'fs';
import { WaveFile } from 'wavefile';
import sampleList from './samples.json';
import {hello, am, fm, usesample, envelope, filter} from './examples'

import AceEditor from "react-ace";
import "ace-builds/src-noconflict/mode-glicol";
import "ace-builds/src-noconflict/theme-glicol-night";

export default function App() {

  const classes = useStyles();

  const actx = useRef()
  const loaded = useRef(false)
  const node = useRef()
  // const [url, setUrl] = useState('alex, 0')
  const [code, setCode] = useState(filter)
  const codeRef = useRef(code)
  // const [isPlaying, setIsPlaying] = useState(false)
  const encoder = new TextEncoder('utf-8');
  const [height, setHeight] = useState(800)
  const [width, setWidth] = useState(600)

  const [running, setRunning] = useState(false)
  const [sideOpen, setSideOpen] = useState(false)

  const loadModule = async () => {
    // Note the the path is from public folder
    actx.current = new window.AudioContext()

    // const processorPath = isDevMode ? 'public/worklet/engine.js' : `${global.__dirname}/worklet/engine.js`;
    // const processorPath = 'worklet/engine.js'
    // const processorSource = await readFile(processorPath); // just a promisified version of fs.readFile
    // const processorBlob = new Blob([processorSource.toString()], { type: 'text/javascript' });
    // const processorURL = URL.createObjectURL(processorBlob);
    // await actx.current.audioWorklet.addModule(processorURL);

    await actx.current.audioWorklet.addModule('./worklet/engine.js')
    node.current = new AudioWorkletNode(actx.current, 'glicol-engine')

    fetch('wasm/glicol_wasm.wasm')
    .then(response => response.arrayBuffer())
    .then(arrayBuffer => node.current.port.postMessage({type: "load", obj: arrayBuffer}))
    node.current.connect(actx.current.destination)
    loaded.current = true
    console.log("loaded")

  };

  useEffect(() => {
    setSize()
    try {
      loadModule()
    } catch (e) {
      console.log(e)
    }
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

  const change = (v) => {
    setCode(v)
    codeRef.current = v
  }


  const setSize = () => {
    try {
        // let w1 = document.getElementById('AppBar').offsetWidth;
        // let w1 = 0;
        let w = window.innerWidth;
        // let w = w1 < w2 ? w1 : w2
        let h = window.innerHeight;
        h -= document.getElementById('AppBar').offsetHeight
        setHeight(h)
        setWidth(w)
        console.log(w, h)
    } catch (e) {console.log(e)}
  }
  window.onresize = setSize

  const handleUpdate = () => {
    actx.current.resume()
    setRunning(true)
    // console.log(codeRef.current)
    try {
      // node.current.port.postMessage({type: "update", value: encoder.encode(code)})
      node.current.port.postMessage({type: "update", value: encoder.encode(codeRef.current)})
    } catch (e) {
      console.log(e)
    }
  }

  const handleRun = () => {
    try {
      actx.current.resume()
      setRunning(true)
    } catch (e) {
      console.log(e)
    }
    // console.log(codeRef.current)
    try {
      node.current.port.postMessage({type: "run", value: encoder.encode(codeRef.current)})
    } catch (e) {
      console.log(e)
    }
  }

  const handlePause = () => {
    actx.current.suspend()
    setRunning(false)
    // console.log(codeRef.current)
  }

  const handleStop = () => {
    try {
      actx.current.close();
      loadModule();
      setRunning(false)
    } catch (e) {
      console.log(e)
    }
    console.log("stop") 
  }

  const handleList = (code) => {
    setCode(code);
    setSideOpen(false);
    codeRef.current=code
  }

  
  return (
    <div className="App">
        <ThemeProvider theme={theme}>
        <AppBar
          className={classes.appBar}
          id="AppBar"
        >
        <Toolbar>

        { !running ? (

          <Tooltip title="Run (cmd + enter / ctrl + enter)">
          <IconButton
            color="inherit"
            aria-label="Run"
            edge="end"
            onClick={handleRun}
            className={clsx(sideOpen && classes.hide)}
          >
          <PlayCircleFilledIcon  fontSize="large" />
          </IconButton>
          </Tooltip>

        ) : (

          <Tooltip title="Pause">
          <IconButton
            color="inherit"
            aria-label="Pause"
            edge="end"
            onClick={handlePause}
            className={clsx(sideOpen && classes.hide)}
          >
          <PauseCircleFilledIcon fontSize="large" />
          </IconButton>
          </Tooltip>
          
        )}

        <Tooltip title="Stop">
        <IconButton
          color="inherit"
          aria-label="Stop (cmd + shift + . / ctrl + shift + .)"
          edge="end"
          onClick={handleStop}
          className={clsx(sideOpen && classes.hide)}
        >
          <PanoramaFishEyeIcon   fontSize="large" />
        </IconButton>
        </Tooltip>
        
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
        <div className={classes.menu}>
        <IconButton
          href="https://github.com/glicol/glicol"
          target="_blank"
          rel="noopener noreferrer"
          // data-show-count="true"
          aria-label="GitHub"
          color="inherit"
          // aria-label="open drawer"
          edge="end"
        ><GitHubIcon />
        </IconButton>
        </div>
        </Toolbar>

        <Divider />

        <List>
        <ListItem
          button
          onClick={()=>{handleList(hello)}}
        >
        <ListItemText
          primary={
          <Typography
            style={{ fontFamily: '\'Inconsolata\', monospace'}}
          >hello world.</Typography>
        }
        />
        </ListItem>
        </List>

        <List>
        <ListItem
          button
          onClick={()=>{handleList(am)}}
        >
        <ListItemText
          primary={
          <Typography
            style={{ fontFamily: '\'Inconsolata\', monospace'}}
          >am.</Typography>
        }
        />
        </ListItem>
        </List>

        <List>
        <ListItem
          button
          onClick={()=>{handleList(fm)}}
        >
        <ListItemText
          primary={
          <Typography
            style={{ fontFamily: '\'Inconsolata\', monospace'}}
          >fm.</Typography>
        }
        />
        </ListItem>
        </List>

        <Divider />

        <List>
        <ListItem
          button
          onClick={()=>{handleList(usesample)}}
        >
        <ListItemText
          primary={
          <Typography
            style={{ fontFamily: '\'Inconsolata\', monospace'}}
          >use samples.</Typography>
        }
        />
        </ListItem>
        </List>

        <List>
        <ListItem
          button
          onClick={()=>{handleList(envelope)}}
        >
        <ListItemText
          primary={
          <Typography
            style={{ fontFamily: '\'Inconsolata\', monospace'}}
          >envelope.</Typography>
        }
        />
        </ListItem>
        </List>

        <Divider />

        <List>
        <ListItem
          button
          onClick={()=>{handleList(filter)}}
        >
        <ListItemText
          primary={
          <Typography
            style={{ fontFamily: '\'Inconsolata\', monospace'}}
          >filter.</Typography>
        }
        />
        </ListItem>
        </List>

        <Divider />

        <List>
        <ListItem
          button
          onClick={()=>{handleList("~sin: sin 110.0")}}>
        <ListItemText
          primary={<Typography
          style={{ fontFamily: '\'Inconsolata\', monospace'}}
          >template - synthesis.
        </Typography>}
        ></ListItemText>
        </ListItem>
        </List>

        <List>
        <ListItem
          button
          onClick={()=>{handleList("~bd: loop 60 >> sampler \\bd"); loadSamples();}}>
        <ListItemText
          primary={<Typography
          style={{ fontFamily: '\'Inconsolata\', monospace'}}
          >template - use samples.
        </Typography>}
        ></ListItemText>
        </ListItem>
        </List>
        
        </Drawer>

        </Toolbar> 
        </AppBar>
        <Toolbar />

        <AceEditor
          className={classes.editor}
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
          }, {   // commands is array of key bindings.
            name: 'Stop', //name for the key binding.
            bindKey: {win: 'Ctrl-Shift-.', mac: 'Command-Shift-.'}, //key combination used for the command.
            exec: handleStop //function to execute when keys are pressed.
          }]}
        />
        </ThemeProvider>
        
     </div>

  );
}