import React from 'react';
import { Box, CircularProgress, Tooltip, Typography } from '@mui/material';
import { DateTime } from 'luxon';
import { ConnectionStatusKind, GatewayPerformance } from '../types';
import { ServiceProvider } from '../types/directory';
import { ServiceProviderInfo } from './ServiceProviderInfo';

const FONT_SIZE = '10px';
const FONT_WEIGHT = '600';
const FONT_STYLE = 'normal';

const ConnectionStatusContent: FCWithChildren<{
  status: ConnectionStatusKind;
}> = ({ status }) => {
  switch (status) {
    case ConnectionStatusKind.connected:
      return (
        <Typography fontWeight={FONT_WEIGHT} fontStyle={FONT_STYLE} fontSize="14px">
          Connected to
        </Typography>
      );
    case ConnectionStatusKind.disconnecting:
      return (
        <Box display="flex" alignItems="center" justifyContent="center">
          <CircularProgress size={FONT_SIZE} color="inherit" />
          <Typography fontWeight={FONT_WEIGHT} fontStyle={FONT_STYLE} ml={1}>
            Disconnecting...
          </Typography>
        </Box>
      );
    case ConnectionStatusKind.connecting:
      return (
        <Box display="flex" alignItems="center" justifyContent="center">
          <CircularProgress size={FONT_SIZE} color="inherit" />
          <Typography fontWeight={FONT_WEIGHT} fontStyle={FONT_STYLE} ml={1}>
            Connecting...
          </Typography>
        </Box>
      );
    case ConnectionStatusKind.disconnected:
      return (
        <Typography
          fontWeight={FONT_WEIGHT}
          fontStyle={FONT_STYLE}
          ml={1}
          textTransform="uppercase"
          textAlign="center"
          fontSize={FONT_SIZE}
          sx={{ wordSpacing: 3, letterSpacing: 2 }}
        >
          You are not protected
        </Typography>
      );
    default:
      return null;
  }
};

export const ConnectionStatus: FCWithChildren<{
  status: ConnectionStatusKind;
  gatewayPerformance?: GatewayPerformance;
  connectedSince?: DateTime;
  serviceProvider?: ServiceProvider;
}> = ({ status, serviceProvider, gatewayPerformance }) => {
  const color =
    status === ConnectionStatusKind.connected || status === ConnectionStatusKind.disconnecting
      ? '#21D072'
      : 'warning.main';

  return (
    <>
      <Box color={color} fontSize={FONT_SIZE} sx={{ mb: 1 }}>
        {status === ConnectionStatusKind.connected && gatewayPerformance !== 'Good' ? (
          <Typography fontWeight={FONT_WEIGHT} fontStyle={FONT_STYLE} textAlign="left" color="primary">
            Gateway has issues
          </Typography>
        ) : (
          <ConnectionStatusContent status={status} />
        )}
      </Box>
      {serviceProvider ? (
        <Tooltip title={<ServiceProviderInfo serviceProvider={serviceProvider} />}>
          <Box sx={{ cursor: 'pointer' }}>
            {serviceProvider && <Typography>{serviceProvider.description}</Typography>}
          </Box>
        </Tooltip>
      ) : null}
    </>
  );
};
