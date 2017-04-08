// @flow
import { makeExecutableSchema } from "graphql-tools";

import { Player, Character, Match } from "./models.js";

const typeDefs = `
    type Player {
        id: String
        name: String
        matches: [Match]
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

        getPlayer(id: String): Player
        getCharacter(id: String): Character
        getMatch(id: String): Match
    }

    type Mutation {
        createMatch(
          winnerId: String!,
          player1Id: String!,
          player2Id: String!,
          character1Id: String!,
          character2Id: String!,
        ): Match
    }

    schema {
      query: Query
      mutation: Mutation
    }
`;

const resolvers = {
  Player: {
    matches: async player => {
      return Match.findAll({
        where: {
          $or: [{ player1Id: player.id }, { player2Id: player.id }]
        }
      });
    }
  },
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
    },
    getPlayer: async (obj, args) => {
      return Player.findById(args.id);
    },
    getCharacter: async (obj, args) => {
      return Character.findById(args.id);
    },
    getMatch: async (obj, args) => {
      return Match.findById(args.id);
    }
  },
  Mutation: {
    createMatch: async (
      _,
      { winnerId, player1Id, player2Id, character1Id, character2Id }
    ) => {
      return Match.create({
        winnerId,
        player1Id,
        player2Id,
        character1Id,
        character2Id
      });
    }
  }
};

export default makeExecutableSchema({ typeDefs, resolvers });
