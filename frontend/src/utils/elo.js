// @flow

type EloPair = {
  winnerElo: number,
  loserElo: number
};

const K = 32;

function expected(a: number, b: number): number {
  return 1 / (1 + 10 ** ((b - a) / 400));
}

function elo(old: number, exp: number, score: number): number {
  return old + K * (score - exp);
}

export function calcNewElos(
  winnerOriginalElo: number,
  loserOriginalElo: number
): EloPair {
  const winnerExp = expected(winnerOriginalElo, loserOriginalElo);
  const loserExp = expected(loserOriginalElo, winnerOriginalElo);

  return {
    winnerElo: elo(winnerOriginalElo, winnerExp, 1),
    loserElo: elo(loserOriginalElo, loserExp, 0)
  };
}
