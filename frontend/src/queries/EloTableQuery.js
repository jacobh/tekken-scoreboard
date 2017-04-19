// @flow
import { gql } from "react-apollo";

export default gql`{
  allPlayers {
    id
    name
  }
  allMatches {
    id
    winner {
      id
      name
    }
    loser {
      id
      name
    }
    
  }
}
`;
