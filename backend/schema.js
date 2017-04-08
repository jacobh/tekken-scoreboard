// @flow
import { makeExecutableSchema } from "graphql-tools";

import { Player, Character, Match } from "./models.js";

const typeDefs = `
    type Player {
        id: String
        name: String
    }

    type Character {
        id: String
        name: String
    }

    type Match {
        id: String
        winner: Player
        player1: Player
        player2: Player
        character1: Character
        character2: Character
    }

    type Query {
        allPlayers: [Player]
        allCharacters: [Character]
        allMatches: [Match]
    }
`;

const resolvers = {
  Match: {
    winner: async match => {
      return Player.findById(match.winnerId);
    },
    player1: async match => {
      return Player.findById(match.player1Id);
    },
    player2: async match => {
      return Player.findById(match.player2Id);
    },
    character1: async match => {
      return Character.findById(match.character1Id);
    },
    character2: async match => {
      return Character.findById(match.character2Id);
    }
  },
  Query: {
    allPlayers: async () => {
      return Player.findAll();
    },
    allCharacters: async () => {
      return Character.findAll();
    },
    allMatches: async () => {
      return Match.findAll();
    }
  }
};

export default makeExecutableSchema({ typeDefs, resolvers });
