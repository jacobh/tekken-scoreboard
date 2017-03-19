// @flow
import * as Immutable from "immutable";
import type { FluxStandardAction } from "flux-standard-action";
import { Player } from "../models.js";
import type { PlayerMap } from "../models.js";
import data from "../../../data.json";

const initialState = Immutable.Map(
  data.players.map(p => [
    p.uuid,
    new Player({ id: p.uuid, name: p.name, foo: "" })
  ])
);

export default function players(
  state: PlayerMap = initialState,
  action: FluxStandardAction
): PlayerMap {
  return state;
}
