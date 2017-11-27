import * as React from 'react';
import CardThumb from './cardthumb';


export default class Hand extends React.Component {
    render() {
        let hand = [];
        for(var i = 0; i < 5; i++) {
            hand.push(<CardThumb card="asdf" />);
        }
        
        return (
            <div id="hand">{hand}</div>
        );
    }
}
