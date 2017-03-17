// @flow
import React from "react";
import * as Immutable from "immutable";
import classNames from "classnames";
import { connect } from "react-redux";

function SetList(
  props: {
    sets: Immutable.Map<string, Immutable.Map<string, any>>,
    players: Immutable.Map<string, Immutable.Map<string, any>>
  }
) {
  const displaySets = props.sets.toIndexedSeq().map(set => {
    const winnerId = set.get("winnerId");

    let winner;
    if (winnerId === set.get("player1Id")) {
      winner = "PLAYER_1";
    } else if (winnerId === set.get("player2Id")) {
      winner = "PLAYER_2";
    }

    return Immutable.Map({
      uuid: set.get("uuid"),
      winner: winner,
      player1Name: props.players.get(set.get("player1Id")).get("name"),
      player2Name: props.players.get(set.get("player2Id")).get("name")
    });
  });
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
          {displaySets.map(set => (
            <tr key={set.get("uuid")}>
              <td
                className={classNames({
                  success: set.get("winner") === "PLAYER_1"
                })}
              >
                {set.get("player1Name")}
              </td>
              <td
                className={classNames({
                  success: set.get("winner") === "PLAYER_2"
                })}
              >
                {set.get("player2Name")}
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

export default connect(state => {
  return {
    sets: state.get("sets"),
    players: state.get("players")
  };
})(SetList);
