import CoreLayout from 'components/common/layouts/CoreLayout';
import Snake from 'components/games/Snake';

export default function Home() {
  return (
    <>
      <CoreLayout>
        <Snake />
      </CoreLayout>
    </>
  );
}
