// @flow
import * as Immutable from "immutable";
import { combineReducers } from "redux-immutable";
import { createStore } from "redux";

import players from "./reducers/players.js";
import characters from "./reducers/characters.js";
import sets from "./reducers/sets.js";

const rootReducer = combineReducers({ players, characters, sets });
const store = createStore(rootReducer, Immutable.Map());
export default store;
