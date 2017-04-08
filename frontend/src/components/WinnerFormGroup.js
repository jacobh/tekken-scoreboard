// @flow
import React from "react";
import { FormGroup, ControlLabel, FormControl } from "react-bootstrap";

export default function WinnerFormGroup(
  props: {
    players: { id: string, name: string }[],
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
        {props.players.map(player => (
          <option value={player.id} key={player.id}>
            {player.name}
          </option>
        ))}
      </FormControl>
    </FormGroup>
  );
}
