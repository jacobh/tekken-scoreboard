// @flow
import * as Immutable from "immutable";
import type { FluxStandardAction } from "flux-standard-action";
import { Character } from "../models.js";
import type { CharacterMap } from "../models.js";
import data from "../../../data.json";

const initialState = Immutable.Map(
  data.characters.map(char => [
    char.uuid,
    new Character({ id: char.uuid, name: char.name, foo: "" })
  ])
);

export default function characters(
  state: CharacterMap = initialState,
  action: FluxStandardAction
): CharacterMap {
  return state;
}
