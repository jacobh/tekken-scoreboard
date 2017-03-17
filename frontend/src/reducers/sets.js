// @flow
import * as Immutable from "immutable";
import uuidV4 from "uuid/v4";

export default function sets(
  state: Immutable.Map<string, Immutable.Map<string, any>> = Immutable.Map(),
  action: any
) {
  switch (action.type) {
    case "CREATE_SET":
      const uuid = uuidV4();
      const newSet = Immutable.Map(action.payload);
      const newSetWithUuid = newSet.set("uuid", uuid);
      return state.set(uuid, newSetWithUuid);
    default:
      return state;
  }
}
