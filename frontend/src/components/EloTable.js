//@flow
import React from "react";
import classNames from "classnames";
import { graphql } from "react-apollo";
import EloTableQuery from "../queries/EloTableQuery.js";
import { calcNewElos } from "../utils/elo.js";
import { sorted } from "../utils/sort.js";

function compareMatches(
  a: { createdAt: string },
  b: { createdAt: string }
): number {
  if (a.createdAt < b.createdAt) {
    return -1;
  } else {
    return 1;
  }
}

type EloMatrixRow = {
  [playerId: string]: {
    score: number,
    isWin: ?boolean
  }
};

type EloMatrix = Array<EloMatrixRow>;

function EloTable(props) {
  let players = [];
  if (props.data.allPlayers != null) {
    players = props.data.allPlayers;
  }

  let matches = [];
  if (props.data.allMatches != null) {
    matches = sorted(props.data.allMatches, compareMatches);
  }

  let initialEloScores: EloMatrixRow = {};
  for (let player of players) {
    initialEloScores[player.id] = { score: 1000, isWin: null };
  }
  let eloMatrix: EloMatrix = [initialEloScores];

  for (let match of matches) {
    let winnerId = match.winner.id;
    let loserId = match.loser.id;

    let lastEloRow = eloMatrix.slice(-1)[0];

    let winnerCurrentElo = lastEloRow[winnerId].score;
    let loserCurrentElo = lastEloRow[loserId].score;

    let { winnerElo, loserElo } = calcNewElos(
      winnerCurrentElo,
      loserCurrentElo
    );

    let nextEloRow: EloMatrixRow = {};
    for (let player of players) {
      if (player.id === winnerId) {
        nextEloRow[player.id] = { score: winnerElo, isWin: true };
      } else if (player.id === loserId) {
        nextEloRow[player.id] = { score: loserElo, isWin: false };
      } else {
        nextEloRow[player.id] = {
          score: lastEloRow[player.id].score,
          isWin: null
        };
      }
    }
    eloMatrix.push(nextEloRow);
  }

  return (
    <div>
      <h2>Elo Scores</h2>
      <table className="table">
        <thead>
          <tr>
            {players.map(player => <th key={player.id}>{player.name}</th>)}
          </tr>
        </thead>
        <tbody>
          {eloMatrix.map((eloMatrixRow, i) => (
            <tr key={i}>
              {players.map(player => (
                <td
                  key={player.id}
                  className={classNames({
                    success: eloMatrixRow[player.id].isWin === true,
                    danger: eloMatrixRow[player.id].isWin === false
                  })}
                >
                  {Math.round(eloMatrixRow[player.id].score)}
                </td>
              ))}
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

export default graphql(EloTableQuery)(EloTable);
