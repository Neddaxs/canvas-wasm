import CoreLayout from 'components/common/layouts/CoreLayout';
import Snake from 'components/games/Snake';

export default function SnakeRoute() {
  return (
    <>
      <CoreLayout>
        <Snake />
      </CoreLayout>
    </>
  );
}
