import Link from 'next/link';

import { GAMES, GAME_ROUTES } from 'constants/games';

function AppBar() {
  return (
    <nav className="bg-gray-300 w-full absolute z-50 py-2 flex items-center pl-3 mb-2">
      <ul className="flex">
        {GAMES.map((game) => (
          <li className="mr-6" key={game}>
            <Link href={GAME_ROUTES[game]}>{game}</Link>
          </li>
        ))}
      </ul>
    </nav>
  );
}

export default AppBar;
