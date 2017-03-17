// @flow
import React from "react";
import * as Immutable from "immutable";
import { connect } from "react-redux";
import { Form } from "react-bootstrap";
import PlayerFormGroup from "./PlayerFormGroup.js";
import CharacterFormGroup from "./CharacterFormGroup.js";

type Props = {
  characters: Immutable.List<Immutable.Map<string, any>>,
  players: Immutable.List<Immutable.Map<string, any>>
};

class CreateSetForm extends React.Component {
  props: Props;

  render() {
    return (
      <Form horizontal>
        <h2>Create Set</h2>
        <hr />
        <h3>Player 1</h3>
        <PlayerFormGroup players={this.props.players} />
        <CharacterFormGroup characters={this.props.characters} />
        <hr />
        <h3>Player 2</h3>
        <PlayerFormGroup players={this.props.players} />
        <CharacterFormGroup characters={this.props.characters} />
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
