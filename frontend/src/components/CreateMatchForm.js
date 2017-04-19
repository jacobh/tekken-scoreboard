// @flow
import React from "react";
import { compose } from "redux";
import { Form, Button } from "react-bootstrap";
import { gql, graphql } from "react-apollo";
import CreateMatchFormQuery from "../queries/CreateMatchFormQuery.js";
import MatchListQuery from "../queries/MatchListQuery.js";
import PlayerListQuery from "../queries/PlayerListQuery.js";
import PlayerFormGroup from "./PlayerFormGroup.js";
import CharacterFormGroup from "./CharacterFormGroup.js";
import WinnerFormGroup from "./WinnerFormGroup.js";

class CreateMatchForm extends React.Component {
  props: any;

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

  resetState() {
    this.setState({
      player1Id: null,
      character1Id: null,
      player2Id: null,
      character2Id: null,
      winnerId: null
    });
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
    this.props.mutate({
      variables: this.state,
      refetchQueries: [{ query: MatchListQuery }, { query: PlayerListQuery }]
    });
    this.resetState.bind(this)();
  }

  render() {
    let players = [];
    if (this.props.data.allPlayers != null) {
      players = this.props.data.allPlayers;
    }
    let characters = [];
    if (this.props.data.allCharacters != null) {
      characters = this.props.data.allCharacters;
    }

    const currentPlayers = players.filter(player => {
      return player.id === this.state.player1Id ||
        player.id === this.state.player2Id;
    });

    const canSubmit = this.canSubmit.bind(this)();

    return (
      <Form horizontal onSubmit={this.onSubmit.bind(this)}>
        <h2>Create Match</h2>
        <hr />
        <h3>Player 1</h3>
        <PlayerFormGroup
          players={players.filter(player => player.id !== this.state.player2Id)}
          value={this.state.player1Id}
          onChange={val => this.setState({ player1Id: val })}
        />
        <CharacterFormGroup
          characters={characters}
          value={this.state.character1Id}
          onChange={val => this.setState({ character1Id: val })}
        />
        <hr />
        <h3>Player 2</h3>
        <PlayerFormGroup
          players={players.filter(player => player.id !== this.state.player1Id)}
          value={this.state.player2Id}
          onChange={val => this.setState({ player2Id: val })}
        />
        <CharacterFormGroup
          characters={characters}
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
      </Form>
    );
  }
}

const mutation = gql`
mutation createMatch(
  $winnerId: ID!,
  $player1Id: ID!,
  $player2Id: ID!,
  $character1Id: ID!,
  $character2Id: ID!,
) {
  createMatch(
    winnerId: $winnerId,
    player1Id: $player1Id,
    player2Id: $player2Id,
    character1Id: $character1Id,
    character2Id: $character2Id,
  ) {
    id
  }
}`;

export default compose(graphql(CreateMatchFormQuery), graphql(mutation))(
  CreateMatchForm
);
