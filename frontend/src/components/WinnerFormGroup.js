// @flow
import React from "react";
import * as Immutable from "immutable";
import { FormGroup, ControlLabel, FormControl } from "react-bootstrap";

export default function WinnerFormGroup(
  props: {
    players: Immutable.List<Immutable.Map<string, any>>,
    value: string,
    onChange: (string) => void
  }
) {
  return (
    <FormGroup>
      <ControlLabel>Winner</ControlLabel>
      <FormControl
        componentClass="select"
        value={props.value}
        onChange={evt => {
          props.onChange(evt.target.value);
        }}
      >
        <option value="">Select Winner</option>
        {props.players.map(player => (
          <option value={player.get("uuid")} key={player.get("uuid")}>
            {player.get("name")}
          </option>
        ))}
      </FormControl>
    </FormGroup>
  );
}
