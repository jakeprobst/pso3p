import * as React from 'react';
import {Board, BoardState, PlayerBoardState} from './board';
import Hand from './hand';
import CardFull from './cardfull';
import ActionLog from './actionlog';
import StatusBar from './statusbar';
//import * as ReactDOM from 'react-dom';


//interface Card {
//}

export interface PSO3PState {
    socket: WebSocket;
    actions: string[];
    debug: string[];
    player: number
    boardstate: BoardState;
    activecard: any;
}

export interface PSO3PProps {
}


class PSO3P extends React.Component<PSO3PProps, PSO3PState> {
    constructor(props: PSO3PProps) {
        super(props);



        let ws = new WebSocket("ws://localhost:28305");
        console.log("hello!:" + ws);
        //ws.onmessage = m => this.addAction(m.data);
        ws.onmessage = m => this.onStateChange(m.data);

        /*ws.onopen = m => {
            ws.send("what is this");
            this.addAction("connected")
        };*/
        
        this.state = {
            socket: ws,
            actions: [],
            debug: [],
            player: 0,
            //state: new BoardState(),
            boardstate: {
                player1: {
                    character: undefined,
                    in_play: [],
                } as PlayerBoardState,
                player2: {
                    character: undefined,
                    in_play: [],
                } as PlayerBoardState,
            } as BoardState,
            activecard: "blank"
        };
    }

    addAction(a: string) {
        this.setState({
            actions: this.state.actions.concat(a),
        });
    }

    onStateChange(data: string) {
        this.setState({
            debug: this.state.debug.concat(JSON.stringify(JSON.parse(data), null, 2)),
        });

        let cmd = JSON.parse(data) as any;
        if ("SetPlayer" in cmd) {
            let cmd2 = cmd["SetPlayer"] as any;
            this.setState({
                player: cmd2.player,
            });
        }
        if ("SetCharacter" in cmd) {
            let cmd2 = cmd["SetCharacter"] as any;
            if (cmd2.card.player == 1) {
                let boardstate = this.state.boardstate;
                boardstate.player1.character = cmd2;
                
                this.setState({
                    boardstate: boardstate
                });
            }
            if (cmd2.card.player == 2) {
                let boardstate = this.state.boardstate;
                boardstate.player2.character = cmd2;
                
                this.setState({
                    boardstate: boardstate
                });
            }
        }
    }

    /*click1() {
        this.setState({
            a: this.state.a+1,
        });
        this.state.socket.send("atk " + this.state.a);
    }

    click2() {
        this.setState({
            b: this.state.b + 1,
        });
        this.state.socket.send("def " + this.state.b);
    }*/

    setCardThumbnail(card: any) {
        this.setState({
            activecard: card
        });
    }
    
    render() {
        return (
            <div id="playingfield">
              <StatusBar player={this.state.player} phase={this.state.socket.url} atk={0} def={0} />
              <div id="leftside">
                <Board width={8} height={8} boardstate={this.state.boardstate} onObjectClick={(card) => this.setCardThumbnail(card)}/>
                <Hand />
              </div>
              <div>
                <CardFull card={this.state.activecard}/>
              </div>
              <div>
                <ActionLog actions={this.state.actions}/>
              </div>
              <div>
                <ActionLog actions={this.state.debug}/>
              </div>
            </div>
        );
    }
}


export default PSO3P;




