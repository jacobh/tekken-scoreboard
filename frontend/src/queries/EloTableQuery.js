// @flow
import { gql } from "react-apollo";

export default gql`{
  allEloRows {
    date
    cells {
      player {
        id
      }
      score
      scoreChange
      matchesWon
      matchesLost
    }
  }
  allPlayers {
    name
    id
  }
}
`;
