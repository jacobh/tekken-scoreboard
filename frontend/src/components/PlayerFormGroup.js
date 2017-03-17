// @flow
import React from "react";
import * as Immutable from "immutable";
import { FormGroup, ControlLabel, FormControl } from "react-bootstrap";

export default function PlayerFormGroup(
  props: {
    players: Immutable.Map<string, Immutable.Map<string, any>>,
    value: ?string,
    onChange: (?string) => void
  }
) {
  let value = props.value;
  if (props.value === null) {
    value = "";
  }
  return (
    <FormGroup>
      <ControlLabel>Player</ControlLabel>
      <FormControl
        componentClass="select"
        value={value}
        onChange={evt => {
          let changedValue = evt.target.value;
          if (changedValue === "") {
            changedValue = null;
          }
          props.onChange(changedValue);
        }}
      >
        <option value="">Select Player</option>
        {props.players.toIndexedSeq().map(player => (
          <option value={player.get("uuid")} key={player.get("uuid")}>
            {player.get("name")}
          </option>
        ))}
      </FormControl>
    </FormGroup>
  );
}
