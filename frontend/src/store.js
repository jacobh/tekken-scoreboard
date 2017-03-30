// @flow
import * as Immutable from "immutable";
import { combineReducers } from "redux-immutable";
import { createStore } from "redux";

import players from "./reducers/players.js";
import characters from "./reducers/characters.js";
import matches from "./reducers/matches.js";

const rootReducer = combineReducers({ players, characters, matches });
const store = createStore(rootReducer, Immutable.Map());
export default store;
