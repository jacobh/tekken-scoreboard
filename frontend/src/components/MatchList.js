// @flow
import React from "react";
import classNames from "classnames";
import moment from "moment";
import { graphql } from "react-apollo";
import MatchListQuery from "../queries/MatchListQuery.js";

function sorted<T>(arr: T[], compareFn: (T, T) => number): T[] {
  return arr.slice(0).sort(compareFn);
}

function compareMatches(
  a: { createdAt: string },
  b: { createdAt: string }
): number {
  if (a.createdAt > b.createdAt) {
    return -1;
  } else {
    return 1;
  }
}

function MatchList(props) {
  let matches = [];
  if (props.data.allMatches != null) {
    matches = props.data.allMatches;
  }
  return (
    <div>
      <h2>Past Matches</h2>
      <table className="table">
        <thead>
          <tr>
            <th>Date</th>
            <th>Player 1</th>
            <th />
            <th>Player 2</th>
            <th />
          </tr>
        </thead>
        <tbody>
          {sorted(matches, compareMatches).map(match => (
            <tr key={match.id}>
              <td>{moment(match.createdAt).fromNow()}</td>
              <td
                className={classNames({
                  success: match.winner.id === match.player1.id
                })}
              >
                {match.player1.name}
              </td>
              <td>{match.character1.name}</td>
              <td
                className={classNames({
                  success: match.winner.id === match.player2.id
                })}
              >
                {match.player2.name}
              </td>
              <td>{match.character2.name}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

export default graphql(MatchListQuery)(MatchList);
