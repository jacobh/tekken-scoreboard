// @flow
import * as Immutable from "immutable";
import { combineReducers } from "redux-immutable";
import { createStore } from "redux";
import data from "../../data.json";

import players from "./reducers/players.js";
import characters from "./reducers/characters.js";
import sets from "./reducers/sets.js";

const initialState = Immutable.fromJS(data);
const rootReducer = combineReducers({ players, characters, sets });
const store = createStore(rootReducer, initialState);
export default store;
