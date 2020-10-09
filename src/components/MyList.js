import React from 'react';
import { List, ListItem, ListItemText, Typography } from '@material-ui/core'

const MyList = ({onClick, title}) => (
    <List>
    <ListItem
    button
    onClick={onClick}
    >
    <ListItemText
    primary={
    <Typography
        style={{ fontFamily: '\'Inconsolata\', monospace'}}
    >{title}</Typography>
    }
    />
    </ListItem>
    </List>
)

export default MyList