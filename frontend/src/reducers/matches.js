// @flow
import * as Immutable from "immutable";
import uuidV4 from "uuid/v4";
import type { FluxStandardAction } from "flux-standard-action";

export default function matches(
  state: Immutable.Map<string, Immutable.Map<string, any>> = Immutable.Map(),
  action: FluxStandardAction
): Immutable.Map<string, Immutable.Map<string, any>> {
  switch (action.type) {
    case "CREATE_MATCH":
      const uuid = uuidV4();
      const newMatch = Immutable.Map(action.payload);
      const newMatchWithUuid = newMatch.set("uuid", uuid);
      return state.set(uuid, newMatchWithUuid);
    case "LOAD_MATCH":
      return (() => {
        if (action.payload !== undefined) {
          const newMatch = Immutable.Map(action.payload);
          const newMatchWithUuid = newMatch.set("uuid", action.payload.id);
          return state.set(action.payload.id, newMatchWithUuid);
        }
        return state;
      })();
    default:
      return state;
  }
}
