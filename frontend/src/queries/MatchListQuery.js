// @flow
import { gql } from "react-apollo";

export default gql`{
  allMatches {
    id
    createdAt
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
    character1 {
      name
    }
    character2 {
      name
    }
  }
}`;
