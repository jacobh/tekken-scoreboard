// @flow
import { gql } from "react-apollo";

export default gql`{
  allEloRows {
    createdAt
    cells {
      player {
        id
      }
      score
      scoreChange
    }
  }
  allPlayers {
    name
    id
  }
}
`;
