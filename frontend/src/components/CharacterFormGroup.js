// @flow
import React from "react";
import { FormGroup, ControlLabel, FormControl } from "react-bootstrap";
import type { CharacterMap } from "../models.js";

export default function CharacterFormGroup(
  props: {
    characters: CharacterMap,
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
      <ControlLabel>Character</ControlLabel>
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
        <option value="">Select Character</option>
        {props.characters.toIndexedSeq().map(char => (
          <option value={char.get("id")} key={char.get("id")}>
            {char.get("name")}
          </option>
        ))}
      </FormControl>
    </FormGroup>
  );
}
