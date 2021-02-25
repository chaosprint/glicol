import React from 'react';
import { useStyles, theme } from '../styles/styles';
import {Stepper,Step,  TextField, StepLabel, Button, ThemeProvider} from '@material-ui/core/'
import firebaseConfig from './firebaseConfig'
import { useHistory } from "react-router-dom";

export default function HorizontalLabelPositionBelowStepper() {
  let history = useHistory();
  const classes = useStyles()
  const [activeStep, setActiveStep] = React.useState(0)
  const [password, setPassword] = React.useState("")
  const steps = ['Create a password', 'Get a sharable link']

  const handleNext = ()=> {

    if (password.length >= 6 ) {
      if (!window.firebase.apps.length) {
        window.firebase.initializeApp(firebaseConfig);
      }
      var ref = window.firebase.database().ref()
      ref = ref.push()
      window.key = ref.key
      window.opennew = true
      history.push("/"+ref.key);
      window.firebase.auth().createUserWithEmailAndPassword(
        ref.key+"@glicol.web.app", password).catch( e => console.log(e) );
      setActiveStep((prevActiveStep) => prevActiveStep + 1)
    } else {
      alert("Password shoud be longer than 6 digits.")
    }
    
  }

  return (
    <div className={classes.fork}>
      <ThemeProvider theme={theme}>
      <div className="fork">
      <Stepper className={classes.forkpaper} activeStep={activeStep} alternativeLabel>
        {steps.map((label) => (
          <Step key={label}>
            <StepLabel>{label}</StepLabel>
          </Step>
        ))}
      </Stepper>
      <div className={classes.forkpaper}>
        {activeStep !== 1 ? (
        <form onSubmit={e=>{e.preventDefault(); handleNext()}}>
        <TextField
          id="password-input"
          label="Password"
          type="password"
          variant="filled"
          value={password}
          onChange={e=>{setPassword(e.target.value)}}
        />
        </form>): <></>} <br /><br />
        {activeStep !== 1 ? (
        <Button variant="contained" className={classes.button}
          onClick={handleNext}
        >Next
        </Button>) : (
        <div>
          <p>Current address can be shared to others.</p><br />
          <p>People with the password can edit.</p></div>)}
      </div>
      </div>
      </ThemeProvider>
    </div>
  )
}