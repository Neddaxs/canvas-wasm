import Head from 'components/common/layouts/Head';

import AppBar from './AppBar';

import type { Props } from './CoreLayout.types';

function CoreLayout({ children, headProps = {} }: Props) {
  return (
    <div className="w-screen h-screen overflow-hidden flex flex-col">
      <Head {...headProps} />
      <AppBar />
      {children}
    </div>
  );
}

export default CoreLayout;
