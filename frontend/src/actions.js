// @flow
import { createAction } from "redux-actions";

export const createMatch = createAction("CREATE_MATCH", match => match);
export const loadMatch = createAction("LOAD_MATCH", match => match);
