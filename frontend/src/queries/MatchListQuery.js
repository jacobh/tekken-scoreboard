// @flow
import { gql } from "react-apollo";

export default gql`{
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
