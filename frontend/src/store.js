// @flow
import * as Immutable from "immutable";
import { combineReducers } from "redux-immutable";
import { createStore } from "redux";
import _ from "lodash";
import data from "../../data.json";

import players from "./reducers/players.js";
import characters from "./reducers/characters.js";
import sets from "./reducers/sets.js";

const initialState = Immutable.Map({
  players: Immutable.fromJS(_.keyBy(data.players, "uuid")),
  characters: Immutable.fromJS(_.keyBy(data.characters, "uuid")),
  sets: Immutable.fromJS(_.keyBy(data.sets, "uuid"))
});
const rootReducer = combineReducers({ players, characters, sets });
const store = createStore(rootReducer, initialState);
export default store;
