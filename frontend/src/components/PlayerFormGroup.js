import React from "react";
import * as Immutable from "immutable";
import { FormGroup, ControlLabel, FormControl } from "react-bootstrap";

export default function PlayerFormGroup(
  props: { players: Immutable.List<Immutable.Map<string, any>> }
) {
  return (
    <FormGroup>
      <ControlLabel>Player</ControlLabel>
      <FormControl componentClass="select">
        <option value="">Select Player</option>
        {props.players.map(player => (
          <option value={player.get("uuid")} key={player.get("uuid")}>
            {player.get("name")}
          </option>
        ))}
      </FormControl>
    </FormGroup>
  );
}
