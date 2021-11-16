import Head from 'components/common/layouts/Head';

import type { Props } from './CoreLayout.types';

function CoreLayout({ children, headProps = {} }: Props) {
  return (
    <>
      <Head {...headProps} />
      <div className="container w-screen h-screen">{children}</div>
    </>
  );
}

export default CoreLayout;
