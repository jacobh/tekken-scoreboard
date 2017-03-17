// @flow
import React from "react";
import * as Immutable from "immutable";
import { connect } from "react-redux";
import { Form, Button } from "react-bootstrap";
import { createSet } from "../actions.js";
import PlayerFormGroup from "./PlayerFormGroup.js";
import CharacterFormGroup from "./CharacterFormGroup.js";
import WinnerFormGroup from "./WinnerFormGroup.js";

class CreateSetForm extends React.Component {
  props: {
    characters: Immutable.List<Immutable.Map<string, any>>,
    players: Immutable.List<Immutable.Map<string, any>>,
    dispatch: () => void
  };

  state: {
    player1Id: ?string,
    character1Id: ?string,
    player2Id: ?string,
    character2Id: ?string,
    winnerId: ?string
  };

  constructor(props) {
    super(props);

    this.state = {
      player1Id: null,
      character1Id: null,
      player2Id: null,
      character2Id: null,
      winnerId: null
    };
  }

  canSubmit(): boolean {
    for (let k in this.state) {
      let v = this.state[k];
      if (v === null) {
        return false;
      }
    }
    return true;
  }

  onSubmit(evt: Event) {
    evt.preventDefault();
    this.props.dispatch(createSet(this.state));
  }

  render() {
    const currentPlayers = this.props.players.filter(player => {
      const uuid = player.get("uuid");
      return uuid === this.state.player1Id || uuid === this.state.player2Id;
    });
    const canSubmit = this.canSubmit.bind(this)();

    return (
      <Form horizontal onSubmit={this.onSubmit.bind(this)}>
        <h2>Create Set</h2>
        <hr />
        <h3>Player 1</h3>
        <PlayerFormGroup
          players={this.props.players.filterNot(
            player => player.get("uuid") === this.state.player2Id
          )}
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
          players={this.props.players.filterNot(
            player => player.get("uuid") === this.state.player1Id
          )}
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
        <Button type="submit" disabled={!canSubmit}>
          Submit
        </Button>
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
