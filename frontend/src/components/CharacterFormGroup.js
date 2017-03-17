// @flow
import React from "react";
import * as Immutable from "immutable";
import { FormGroup, ControlLabel, FormControl } from "react-bootstrap";

export default function CharacterFormGroup(
  props: {
    characters: Immutable.List<Immutable.Map<string, any>>,
    value: string,
    onChange: (string) => void
  }
) {
  return (
    <FormGroup>
      <ControlLabel>Character</ControlLabel>
      <FormControl
        componentClass="select"
        value={props.value}
        onChange={evt => {
          props.onChange(evt.target.value);
        }}
      >
        <option value="">Select Character</option>
        {props.characters.map(char => (
          <option value={char.get("uuid")} key={char.get("uuid")}>
            {char.get("name")}
          </option>
        ))}
      </FormControl>
    </FormGroup>
  );
}
