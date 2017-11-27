import * as React from 'react';
import BoardObject from './boardobject';


export interface Position {
    x: number;
    y: number;
}

export interface FieldObject {
    id: number;
    type: string;
    //player: number;
    player: number;
    //card: Card;
    card: any;
    pos: Position;
    hp: number;
}

// TODO: fieldobjectinstance
export interface PlayerBoardState {
    character?: FieldObject;
    in_play: Object[];
}

export interface BoardState {
    player1: PlayerBoardState;
    player2: PlayerBoardState;
}

export interface BoardData {
    width: number;
    height: number;
    boardstate: BoardState;
    onObjectClick: (card: any) => void;
}

export class Board extends React.Component<BoardData, {}> {
    render() {
        /*let board = [];
        for(var y = 0; y < this.props.height; y++) {
            let col = [];
            for(var x = 0; x < this.props.width; x++) {
                console.log("player1: " + JSON.stringify(this.props.boardstate.player1.character));
                if (this.props.boardstate.player1.character != undefined &&
                    this.props.boardstate.player1.character.card.pos.x == x && this.props.boardstate.player1.character.card.pos.y == y) {
                    col.push(<BoardObject object={this.props.boardstate.player1.character.card}/>);
                }
                else {
                    col.push(<BoardObject object={undefined}/>);
                }
            }
            board.push(<div className="boardrow">{col}</div>);
            }*/
        
        let board = [];
        //board.push(<table>);
        for(var y = 0; y < this.props.height; y++) {
            let col = [];
            for(var x = 0; x < this.props.width; x++) {
                if (this.props.boardstate.player1.character != undefined &&
                    this.props.boardstate.player1.character.card.pos.x == x && this.props.boardstate.player1.character.card.pos.y == y) {
                    col.push(<BoardObject object={this.props.boardstate.player1.character.card.card.Character}
                             onClick={() => this.props.onObjectClick(this.props.boardstate.player1.character!.card.card.Character)}/>);
                }
                else {
                    col.push(<BoardObject object={undefined} onClick={() => this.props.onObjectClick(x + y)} />);
                }
            }
            //board.push(<div className="boardrow">{col}</div>);
            board.push(<tr>{col}</tr>);
        }
        //board.push(</table>);
        
        return (
            <div id="board"><table>{board}</table></div>
        );
    }
}
