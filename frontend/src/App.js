import React, { Component } from "react";
import "./App.css";
import data from "../../data.json";

class App extends Component {
  render() {
    return (
      <div className="App"><pre>{JSON.stringify(data, null, 2)}</pre></div>
    );
  }
}

export default App;
