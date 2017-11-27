import * as React from 'react';


export interface StatusBarState {
    player: number;
    phase: string;
    atk: number;
    def: number;
}


export default class StatusBar extends React.Component<StatusBarState, {}> {
    render() {
        return (
            <div id="statusbar">
              <span id="player">Player {this.props.player}</span>
              <span id="phase">{this.props.phase}</span>
              <span id="atkroll">Atk: {this.props.atk}</span>
              <span id="defroll">Def: {this.props.def}</span>
            </div>
        );
    }
}
