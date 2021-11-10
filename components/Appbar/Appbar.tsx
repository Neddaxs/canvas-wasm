import { memo } from "react";
import { GAME_VALUES } from "../../types/GAME.enum";
import type AppbarProps from "./utilities/Appbar.props";

export default memo(function Appbar({
  currentGame,
  setCurrentGame,
}: AppbarProps): JSX.Element {
  return (
    <ul
      style={{
        width: "100%",
        display: "flex",
        flexDirection: "row",
        listStyle: "none",
      }}
    >
      {GAME_VALUES.map(
        (game): JSX.Element => (
          <li
            key={game}
            onClick={(): void => {
              if (currentGame !== game) {
                setCurrentGame(game);
              }
            }}
            style={{
              padding: "1em 1.5em",
              ...(currentGame === game
                ? {
                    background: "cyan",
                  }
                : {
                    background: "gray",
                    cursor: "pointer",
                  }),
            }}
          >
            {game}
          </li>
        )
      )}
    </ul>
  );
});
