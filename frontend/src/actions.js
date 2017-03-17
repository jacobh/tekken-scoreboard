// @flow
import { createAction } from "redux-actions";

export const createSet = createAction("CREATE_SET", set => set);
export const loadSet = createAction("LOAD_SET", set => set);
