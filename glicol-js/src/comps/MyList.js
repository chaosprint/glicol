import React from 'react';
import { Link } from "react-router-dom";
import { List, ListItem } from '@material-ui/core'

const MyList = ({onClick, title}) => (
    <List>
    <ListItem
        button
        onClick={onClick}
    >
    <Link
        to="/"
        style={{ color:"white", fontFamily: '\'Inconsolata\', monospace'}}
    >{title}</Link>
    {/* <ListItemText
    primary={
    <Typography
        style={{ fontFamily: '\'Inconsolata\', monospace'}}
    >{title}</Typography>
    }
    /> */}
    </ListItem>
    </List>
)

export default MyList