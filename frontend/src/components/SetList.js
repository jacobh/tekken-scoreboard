// @flow
import React from "react";
import * as Immutable from "immutable";
import { connect } from "react-redux";

function SetList(
  props: {
    sets: Immutable.Map<string, Immutable.Map<string, any>>
  }
) {
  return (
    <div>
      <h2>Past Sets</h2>
      <table className="table">
        <thead>
          <tr>
            <th>Player 1</th>
            <th>Player 2</th>
          </tr>
        </thead>
        <tbody>
          {props.sets.toIndexedSeq().map(set => (
            <tr key={set.get("uuid")}>
              <td>{set.get("player1Id")}</td>
              <td>{set.get("player2Id")}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

export default connect(state => {
  return {
    sets: state.get("sets")
  };
})(SetList);
