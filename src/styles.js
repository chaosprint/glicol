import { makeStyles } from '@material-ui/core/styles';
import { createMuiTheme } from '@material-ui/core/styles';
import { red, green  } from '@material-ui/core/colors';
// grey
const theme = createMuiTheme({
  spacing: 0,
  margin: -5,
  palette: {
    primary: {
      // light: will be calculated from palette.primary.main,
      main: '#424242',
      // dark: will be calculated from palette.primary.main,
      // contrastText: will be calculated to contrast with palette.primary.main
    },
  },
  typography: {
    fontFamily: '\'Inconsolata\', monospace'
    // fontFamily: 'Inconsolata'
  }
});


const buttonTheme = createMuiTheme({
  borderRadius: 0,
  palette: {
    primary: green,
    secondary: red,
  },
});

const drawerWidth = 240;

const useStyles = makeStyles((theme) => ({
  root: {
    display: 'flex',
    flexGrow: 1,
  },
  appBar: {
      // position: 'absolote',
      margin: 0,
      padding: 0,
      transition: theme.transitions.create(['margin', 'width'], {
      easing: theme.transitions.easing.sharp,
      duration: theme.transitions.duration.leavingScreen,
    }),
  },
  appBarShift: {
    width: `calc(100% - ${drawerWidth}px)`,
    transition: theme.transitions.create(['margin', 'width'], {
      easing: theme.transitions.easing.easeOut,
      duration: theme.transitions.duration.enteringScreen,
    }),
    marginRight: drawerWidth,
  },
  title: {
    // flexGrow: 1,
  },
  hide: {
    display: 'none',
  },
  menu: {
    marginLeft: 'auto',
  },
  drawer: {
    width: drawerWidth,
    flexShrink: 0,
  },
  drawerPaper: {
    width: drawerWidth,
    color: '#fff',
    background: '#303030'
  },
  drawerHeader: {
    // display: 'flex',
    marginRight: 'auto',
    // margin: theme.spacing(1),
    // alignItems: 'center',
    // padding: theme.spacing(0, 1),
    // necessary for content to be below app bar
    // ...theme.mixins.toolbar,
    color: '#fff',
    // justifyContent: 'flex-start',
  },
  content: {
    // flexGrow: 1,
    // padding: theme.spacing(3),
    transition: theme.transitions.create('margin', {
      easing: theme.transitions.easing.sharp,
      duration: theme.transitions.duration.leavingScreen,
    }),
    marginRight: -drawerWidth,
  },
  contentShift: {
    transition: theme.transitions.create('margin', {
      easing: theme.transitions.easing.easeOut,
      duration: theme.transitions.duration.enteringScreen,
    }),
    marginRight: 0,
  },
  button: {
    width: 80,
    margin: theme.spacing(-1, -1),
  },
  text: {
    typography: {
      fontFamily: '\'Inconsolata\', monospace'
      // fontFamily: 'Inconsolata'
    }
  },
  editor: {
    margin: theme.spacing(0, 0),
    position: 'fixed',
    // width: "100%",
    // height: 700
  },
}));

// const useStyles = makeStyles(theme => ({
//   root: {
//     flexGrow: 1,
//   },
//   text: {
//     margin: theme.spacing(1),
//     [`& fieldset`]: {
//       borderRadius: 0,
//     },
//     // textAlign: "center",
//     width: 120
//   },
//   password: {
//     width: "50%",
//     position: 'relative'
//   },
//   button: {
//     width: 80,
//     margin: theme.spacing(1),
//   },
//   room: {
//     marginLeft: 'auto',
//     textAlign: "center",
//     [`& fieldset`]: {
//       borderRadius: 0,
//     },
//   },
//   paper: {
//     position: 'absolute',
//     backgroundColor: theme.palette.background.paper,
//     boxShadow: theme.shadows[5],
//     padding: theme.spacing(4),
//     outline: 'none',
//   },
//   fab: {
//     position: 'fixed',
//     zIndex: 2000,
//     // button: "10%",
//     // right: "5%"
//     bottom: theme.spacing(2),
//     right: theme.spacing(2),
//   },
//   firepad: {
//     position: 'absolote',
//     width: "100%",
//     height: 700
//   },
//   inside: {
//     padding: theme.spacing(3, 2),
//     [`& fieldset`]: {
//       borderRadius: 0,
//     },
//   },
//   link: {
//     margin: theme.spacing(1),
//   },
//   back: {
//     position: 'absolute',
//     left: '50%',
//     textAlign: "center",
//     [`& fieldset`]: {
//       borderRadius: 0,
//     },
//     top: '50%',
//     transform: 'translate(-50%, -50%)',
//     zIndex:"-2000",
//   }
// }));

const modalStyle = {
    top: "50%",
    left: "50%",
    transform: "translate(-50%, -50%)",
}

export {useStyles, theme, buttonTheme, modalStyle};