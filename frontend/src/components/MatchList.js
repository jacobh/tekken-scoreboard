// @flow
import React from "react";
import classNames from "classnames";
import { gql, graphql } from "react-apollo";

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
            <th>Player 1</th>
            <th>Player 2</th>
          </tr>
        </thead>
        <tbody>
          {matches.map(match => (
            <tr key={match.id}>
              <td
                className={classNames({
                  success: match.winner.id === match.player1.id
                })}
              >
                {match.player1.name}
              </td>
              <td
                className={classNames({
                  success: match.winner.id === match.player2.id
                })}
              >
                {match.player2.name}
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

const query = gql`{
  allMatches {
    id
    winner {
      id
    }
    player1 {
      id
      name
    }
    player2 {
      id
      name
    }
  }
}`;

export default graphql(query)(MatchList);
