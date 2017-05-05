//@flow
import React from "react";
import classNames from "classnames";
import { graphql } from "react-apollo";
import moment from "moment";
import EloTableQuery from "../queries/EloTableQuery.js";
import { sorted } from "../utils/sort.js";

function comparePlayers(a: { name: string }, b: { name: string }): number {
  if (a.name < b.name) {
    return -1;
  } else {
    return 1;
  }
}

function EloTable(props) {
  let players = [];
  if (props.data.allPlayers != null) {
    players = sorted(props.data.allPlayers, comparePlayers);
  }

  let eloRows = [];
  if (props.data.allEloRows != null) {
    eloRows = props.data.allEloRows;
  }

  return (
    <div>
      <h2>Elo Scores</h2>
      <table className="table">
        <thead>
          <tr>
            <th style={{ width: "120px" }} />
            {players.map(player => <th key={player.id}>{player.name}</th>)}
          </tr>
        </thead>
        <tbody>
          {eloRows.map((eloRow, i) => (
            <tr key={i}>
              <td>
                {eloRow.date != null
                  ? moment(eloRow.date).format("ddd Do MMM")
                  : ""}
              </td>
              {players.map(player => {
                for (let cell of eloRow.cells) {
                  if (cell.player.id === player.id) {
                    return (
                      <td
                        key={player.id}
                        className={classNames({
                          success: cell.scoreChange > 0,
                          danger: cell.scoreChange < 0
                        })}
                      >
                        {Math.round(cell.score)}
                      </td>
                    );
                  }
                }
              })}
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

export default graphql(EloTableQuery)(EloTable);
