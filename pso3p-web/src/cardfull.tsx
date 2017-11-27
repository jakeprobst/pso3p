import * as React from 'react';


interface CardFullData {
    card: any;
}


export default class CardFull extends React.Component<CardFullData, {}> {
    render() {
        return (
            <div id="cardfull">
              <span>{this.props.card.name}</span>
            </div>
        );
    }
}
