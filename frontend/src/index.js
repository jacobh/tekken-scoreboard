// @flow
import React from "react";
import ReactDOM from "react-dom";
import { ApolloProvider } from "react-apollo";
import { Provider } from "react-redux";
import App from "./App";
import store from "./store.js";
import client from "./apolloClient.js";
import "./index.css";

ReactDOM.render(
  <ApolloProvider client={client}>
    <Provider store={store}>
      <App />
    </Provider>
  </ApolloProvider>,
  document.getElementById("root")
);
