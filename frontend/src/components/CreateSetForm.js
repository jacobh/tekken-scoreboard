// @flow
import React from "react";
import * as Immutable from "immutable";
import { connect } from "react-redux";
import { Form } from "react-bootstrap";
import PlayerFormGroup from "./PlayerFormGroup.js";
import CharacterFormGroup from "./CharacterFormGroup.js";
import WinnerFormGroup from "./WinnerFormGroup.js";

class CreateSetForm extends React.Component {
  props: {
    characters: Immutable.List<Immutable.Map<string, any>>,
    players: Immutable.List<Immutable.Map<string, any>>
  };

  state: {
    player1Id: string,
    character1Id: string,
    player2Id: string,
    character2Id: string,
    winnerId: string
  };

  constructor(props) {
    super(props);
    this.state = {
      player1Id: "",
      character1Id: "",
      player2Id: "",
      character2Id: "",
      winnerId: ""
    };
  }

  render() {
    const currentPlayers = this.props.players.filter(player => {
      const uuid = player.get("uuid");
      return uuid === this.state.player1Id || uuid === this.state.player2Id;
    });
    return (
      <Form horizontal>
        <h2>Create Set</h2>
        <hr />
        <h3>Player 1</h3>
        <PlayerFormGroup
          players={this.props.players}
          value={this.state.player1Id}
          onChange={val => this.setState({ player1Id: val })}
        />
        <CharacterFormGroup
          characters={this.props.characters}
          value={this.state.character1Id}
          onChange={val => this.setState({ character1Id: val })}
        />
        <hr />
        <h3>Player 2</h3>
        <PlayerFormGroup
          players={this.props.players}
          value={this.state.player2Id}
          onChange={val => this.setState({ player2Id: val })}
        />
        <CharacterFormGroup
          characters={this.props.characters}
          value={this.state.character2Id}
          onChange={val => this.setState({ character2Id: val })}
        />
        <WinnerFormGroup
          players={currentPlayers}
          value={this.state.winnerId}
          onChange={val => this.setState({ winnerId: val })}
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
