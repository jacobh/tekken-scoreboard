// @flow
import { gql } from "react-apollo";

export default gql`{
  allPlayers {
    id
    name
    playedMatches
    wonMatches
    lostMatches
  }
}`;
