import { ReactNode } from 'react';

import type { Props as HeadProps } from 'components/common/layouts/Head/Head.types';

export type Props = {
  children: ReactNode;
  headProps?: HeadProps;
};
