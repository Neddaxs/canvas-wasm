import type GAME from '../../../types/GAME.enum';

type AppbarProps = {
  currentGame: GAME;
  setCurrentGame(game: GAME): void;
};

export default AppbarProps;
