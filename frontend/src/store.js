// @flow
import * as Immutable from "immutable";
import { combineReducers } from "redux-immutable";
import { createStore } from "redux";
import data from "../../data.json";

const initialState = Immutable.Map({
  characters: Immutable.fromJS(data.characters)
});
const rootReducer = combineReducers({});
const store = createStore(rootReducer, initialState);
export default store;
