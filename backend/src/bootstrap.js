// @flow
import staticData from "../../data.json";
import { Player, Character, Match } from "./models.js";

// load initial data
for (let char of staticData.characters) {
  Character.findOrCreate({
    where: { id: char.uuid },
    defaults: { name: char.name }
  });
}

for (let player of staticData.players) {
  Player.findOrCreate({
    where: { id: player.uuid },
    defaults: { name: player.name }
  });
}

for (let match of staticData.matches) {
  Match.findOrCreate({
    where: { id: match.uuid },
    defaults: {
      winnerId: match.winnerId,
      player1Id: match.player1Id,
      player2Id: match.player2Id,
      character1Id: match.character1Id,
      character2Id: match.character2Id
    }
  });
}
