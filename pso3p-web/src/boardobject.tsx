import * as React from 'react';
//import {FieldObject} from './board';


interface BoardObjectData {
    //object?: FieldObject;
    object?: any;
    onClick: () => void;
}


export default class BoardObject extends React.Component<BoardObjectData, {}> {
    render() {
        let board = "";
        if (this.props.object != undefined) {
            board = this.props.object.name;
            //board = JSON.stringify(this.props.object.name);
            //board = "look at me!";
        }

        
        return (
            //<div className="boardsquare">{board}</div>
            <td className="boardsquare" onClick={this.props.onClick}>{board}</td>
        );
    }
}
