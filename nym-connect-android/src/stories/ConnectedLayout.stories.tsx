import React from 'react';
import { ComponentMeta, ComponentStory } from '@storybook/react';
import { Box } from '@mui/material';
import { DateTime } from 'luxon';
import { ConnectedLayout } from '../layouts/ConnectedLayout';
import { ConnectionStatusKind } from '../types';

export default {
  title: 'Layouts/ConnectedLayout',
  component: ConnectedLayout,
} as ComponentMeta<typeof ConnectedLayout>;

export const Default: ComponentStory<typeof ConnectedLayout> = () => (
  <Box p={2} width={242} sx={{ bgcolor: 'nym.background.dark' }}>
    <ConnectedLayout
      showInfoModal={false}
      gatewayPerformance="Good"
      handleCloseInfoModal={() => undefined}
      status={ConnectionStatusKind.connected}
      connectedSince={DateTime.now()}
      ipAddress="127.0.0.1"
      port={1080}
      stats={[
        {
          label: 'in:',
          totalBytes: 1024,
          rateBytesPerSecond: 1024 * 1024 * 1024 + 10,
        },
        {
          label: 'out:',
          totalBytes: 1024 * 1024 * 1024 * 1024 * 20,
          rateBytesPerSecond: 1024 * 1024 + 10,
        },
      ]}
    />
  </Box>
);
