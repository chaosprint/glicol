import  { Tooltip, IconButton } from '@material-ui/core'
import React from 'react';
import PlayCircleFilledIcon from '@material-ui/icons/PlayCircleFilled';
import PauseCircleFilledIcon from '@material-ui/icons/PauseCircleFilled';
import UpdateIcon from '@material-ui/icons/Update';
import RotateLeftIcon from '@material-ui/icons/RotateLeft';
import MenuIcon from '@material-ui/icons/Menu';

const ToolButton = ({title, onClick, icon}) => {
  // Declare a new state variable, which we'll call "count"
  return (
    <Tooltip title={title}>
    <IconButton
      color="inherit"
      edge="end"
      onClick={onClick}
    >
    {icon}
    </IconButton>
    </Tooltip>
  )
}

const Run = ({onClick, disabled}) => {
  return (
    <ToolButton
    // style = {{diasbled: true}}
      // disabled = {disabled}
      // // disabled = {true}
      // disableFocusRipple = {disabled}
      // disableRipple = {disabled}
      title = "run [ctrl + enter] (or cmd + enter on Mac)"
      onClick = {onClick}
      icon = {<PlayCircleFilledIcon
        // color= { !disabled ? "inherit" : "disabled"}
      fontSize="large" />}
    ></ToolButton>
  )
}

const Update = ({onClick}) => {
  return (
    <ToolButton
      title = "update [shift + enter]"
      onClick = {onClick}
      icon = {<UpdateIcon fontSize="large" />}
    ></ToolButton>
  )
}

const Pause = ({onClick}) => {
  return (
    <ToolButton
      title = "pause"
      onClick = {onClick}
      icon = {<PauseCircleFilledIcon fontSize="large" />}
    ></ToolButton>
  )
}


const Reset = ({onClick}) => {
  return (
    <ToolButton
      title = "reset [ctrl + shift + .] (cmd + shift + . on Mac)"
      onClick = {onClick}
      icon = {<RotateLeftIcon fontSize="large" />}
    ></ToolButton>
  )
}


const Menu = ({onClick}) => (
  // <div className={classes.menu}>
  <IconButton
    color="inherit"
    aria-label="open drawer"
    edge="end"
    onClick={onClick}
    style={{marginLeft: 'auto'}}
    // className={clsx(sideOpen && classes.hide)}
  >
  <MenuIcon />
  </IconButton>
  // </div>
)

export {Run, Pause, Reset, Menu, Update}