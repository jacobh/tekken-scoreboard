// @flow
import { ApolloClient, createNetworkInterface } from "react-apollo";

const networkInterface = createNetworkInterface({
  uri: "/graphql"
});

export default new ApolloClient({
  networkInterface: networkInterface
});
