import { USER_ROUTES } from 'constants/routes';

export enum GAME_OPTIONS {
  SNAKE = 'Snake',
  CHECKERS = 'Checkers',
  CHESS = 'Chess',
}

export const GAMES = Object.values(GAME_OPTIONS);

export const GAME_ROUTES = {
  [GAME_OPTIONS.SNAKE]: USER_ROUTES.SNAKE,
  [GAME_OPTIONS.CHECKERS]: USER_ROUTES.CHECKERS,
  [GAME_OPTIONS.CHESS]: USER_ROUTES.CHESS,
};
