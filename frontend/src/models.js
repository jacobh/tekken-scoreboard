// @flow
import * as Immutable from "immutable";

export const Character = Immutable.Record({ id: "", name: "" });
export const Player = Immutable.Record({ id: "", name: "" });

export type CharacterMap = Immutable.Map<string, Character>;
export type PlayerMap = Immutable.Map<string, Character>;
