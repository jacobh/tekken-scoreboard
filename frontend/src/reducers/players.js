// @flow
import * as Immutable from "immutable";
import type { FluxStandardAction } from "flux-standard-action";

export default function players(
  state: Immutable.Map<string, Immutable.Map<string, any>> = Immutable.Map(),
  action: FluxStandardAction
): Immutable.Map<string, Immutable.Map<string, any>> {
  return state;
}
