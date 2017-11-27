import * as React from 'react';


export interface ActionLogProps {
    actions: string[];
}


export default class ActionLog extends React.Component<ActionLogProps, {}> {
    render() {

        let actions = this.props.actions.map((a) => <div className="actionitem"><pre>{a}</pre></div>);
        
        return (
            <div id="actionlog">{actions}</div>
        );
    }
}
