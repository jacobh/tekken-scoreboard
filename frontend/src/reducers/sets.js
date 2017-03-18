// @flow
import * as Immutable from "immutable";
import uuidV4 from "uuid/v4";
import type { FluxStandardAction } from "flux-standard-action";

export default function sets(
  state: Immutable.Map<string, Immutable.Map<string, any>> = Immutable.Map(),
  action: FluxStandardAction
): Immutable.Map<string, Immutable.Map<string, any>> {
  switch (action.type) {
    case "CREATE_SET":
      const uuid = uuidV4();
      const newSet = Immutable.Map(action.payload);
      const newSetWithUuid = newSet.set("uuid", uuid);
      return state.set(uuid, newSetWithUuid);
    case "LOAD_SET":
      return (() => {
        if (action.payload !== undefined) {
          const newSet = Immutable.Map(action.payload);
          const newSetWithUuid = newSet.set("uuid", action.payload.id);
          return state.set(action.payload.id, newSetWithUuid);
        }
        return state;
      })();
    default:
      return state;
  }
}
