// @flow
import React from "react";
import { FormGroup, ControlLabel, FormControl } from "react-bootstrap";
import type { PlayerMap } from "../models.js";

export default function WinnerFormGroup(
  props: {
    players: PlayerMap,
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
      <ControlLabel>Winner</ControlLabel>
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
        <option value="">Select Winner</option>
        {props.players.toIndexedSeq().map(player => (
          <option value={player.get("id")} key={player.get("id")}>
            {player.get("name")}
          </option>
        ))}
      </FormControl>
    </FormGroup>
  );
}
