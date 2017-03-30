// @flow
import React from "react";
import * as Immutable from "immutable";
import classNames from "classnames";
import { connect } from "react-redux";
import type { PlayerMap } from "../models.js";

function MatchList(
  props: {
    matches: Immutable.Map<string, Immutable.Map<string, any>>,
    players: PlayerMap
  }
) {
  const displayMatches = props.matches.toIndexedSeq().map(match => {
    const winnerId = match.get("winnerId");

    let winner;
    if (winnerId === match.get("player1Id")) {
      winner = "PLAYER_1";
    } else if (winnerId === match.get("player2Id")) {
      winner = "PLAYER_2";
    }

    return Immutable.Map({
      id: match.get("id"),
      winner: winner,
      player1Name: props.players.get(match.get("player1Id")).get("name"),
      player2Name: props.players.get(match.get("player2Id")).get("name")
    });
  });
  return (
    <div>
      <h2>Past Matches</h2>
      <table className="table">
        <thead>
          <tr>
            <th>Player 1</th>
            <th>Player 2</th>
          </tr>
        </thead>
        <tbody>
          {displayMatches.map(match => (
            <tr key={match.get("id")}>
              <td
                className={classNames({
                  success: match.get("winner") === "PLAYER_1"
                })}
              >
                {match.get("player1Name")}
              </td>
              <td
                className={classNames({
                  success: match.get("winner") === "PLAYER_2"
                })}
              >
                {match.get("player2Name")}
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
    matches: state.get("matches"),
    players: state.get("players")
  };
})(MatchList);
