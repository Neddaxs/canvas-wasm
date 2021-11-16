import Head from 'next/head';

import type { Props } from './Head.types';

function HeadComponent({
  title = 'Rust Canvas Games',
  description = 'Rust Canvas games brought to you by Neddaxs',
}: Props) {
  return (
    <Head>
      <title>{title}</title>
      <meta name="description" content={description} />
      <link rel="icon" href="/favicon.ico" />
    </Head>
  );
}

export default HeadComponent;
