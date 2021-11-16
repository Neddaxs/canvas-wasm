import Head from 'components/common/layouts/Head';

import type { Props } from './CoreLayout.types';

function CoreLayout({ children, headProps = {} }: Props) {
  return (
    <div className="container w-screen h-screen">
      <Head {...headProps} />
      {children}
    </div>
  );
}

export default CoreLayout;
