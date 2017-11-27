import * as React from 'react';


interface CardThumbData {
    card: any;
}


export default class CardThumb extends React.Component<CardThumbData, {}> {
    render() {
        return (
            <div className="cardthumb">{this.props.card}</div>
        );
    }
}

