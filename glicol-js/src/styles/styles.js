import { makeStyles } from '@material-ui/core/styles';
import { createMuiTheme } from '@material-ui/core/styles';

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

const drawerWidth = 240;

const dark = '#202020'

const useStyles = makeStyles((theme) => ({
  modal: {
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
  },
  paper: {
    backgroundColor: dark,
    boxShadow: theme.shadows[5],
    padding: theme.spacing(2, 4, 3),
  },
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
    marginLeft: drawerWidth,
  },
  title: {
    // flexGrow: 1,
  },
  hide: {
    display: 'none',
  },
  menu: {
    marginRight: 'auto',
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
    marginLeft: 'auto',
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
    marginLeft: -drawerWidth,
  },
  contentShift: {
    transition: theme.transitions.create('margin', {
      easing: theme.transitions.easing.easeOut,
      duration: theme.transitions.duration.enteringScreen,
    }),
    marginLeft: 0,
  },
  button: {
    width: 80,
    margin: theme.spacing(-1, -1),
  },
  text: {
    typography: {
      fontFamily: 'Inconsolata'
    }
  },
  editor: {
    margin: theme.spacing(0, 0),
    position: 'fixed',
    fontFamily: 'B612 Mono',
  },
  fork: {
    minHeight: 200,
  },
  forkpaper: {
    textAlign: "center",
    alignContent: "center",
    backgroundColor: dark,
  }
}));

export {useStyles, theme};