// @flow
import React from "react";
import * as Immutable from "immutable";
import { connect } from "react-redux";
import { Form } from "react-bootstrap";
import PlayerFormGroup from "./PlayerFormGroup.js";
import CharacterFormGroup from "./CharacterFormGroup.js";

class CreateSetForm extends React.Component {
  props: {
    characters: Immutable.List<Immutable.Map<string, any>>,
    players: Immutable.List<Immutable.Map<string, any>>
  };

  state: {
    player1Player: ?string,
    player1Character: ?string,
    player2Player: ?string,
    player2Character: ?string
  };

  constructor(props) {
    super(props);
    this.state = {
      player1Player: null,
      player1Character: null,
      player2Player: null,
      player2Character: null
    };
  }

  render() {
    return (
      <Form horizontal>
        <h2>Create Set</h2>
        <hr />
        <h3>Player 1</h3>
        <PlayerFormGroup
          players={this.props.players}
          value={this.state.player1Player}
        />
        <CharacterFormGroup
          characters={this.props.characters}
          value={this.state.player1Character}
        />
        <hr />
        <h3>Player 2</h3>
        <PlayerFormGroup
          players={this.props.players}
          value={this.state.player2Player}
        />
        <CharacterFormGroup
          characters={this.props.characters}
          value={this.state.player2Character}
        />
        <pre>{JSON.stringify(this.state, null, 2)}</pre>
      </Form>
    );
  }
}

export default connect(state => {
  return {
    characters: state.get("characters"),
    players: state.get("players")
  };
})(CreateSetForm);
