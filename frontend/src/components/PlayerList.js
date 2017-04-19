//@flow
import React from "react";
import { graphql } from "react-apollo";
import PlayerListQuery from "../queries/PlayerListQuery.js";

function PlayerList(props) {
  let players = [];
  if (props.data.allPlayers != null) {
    players = props.data.allPlayers;
  }
  return (
    <div>
      <h2>Players</h2>
      <table className="table">
        <thead>
          <tr>
            <th>Name</th>
            <th>Played</th>
            <th>Won</th>
            <th>Lost</th>
          </tr>
        </thead>
        <tbody>
          {players.map(player => (
            <tr key={player.id}>
              <td>{player.name}</td>
              <td>{player.playedMatches}</td>
              <td>{player.wonMatches}</td>
              <td>{player.lostMatches}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

export default graphql(PlayerListQuery)(PlayerList);
