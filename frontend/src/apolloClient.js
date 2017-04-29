// @flow
import { ApolloClient, createNetworkInterface } from "react-apollo";

var graphqlUri;

if (process.env.NODE_ENV === "production") {
  graphqlUri = "https://tekken-scorecard.herokuapp.com/graphql";
} else {
  graphqlUri = "/graphql";
}

const networkInterface = createNetworkInterface({
  uri: graphqlUri
});

export default new ApolloClient({
  networkInterface: networkInterface
});
