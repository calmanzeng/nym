import React, { useEffect, useState } from 'react';
import { Box, FormHelperText, Stack, TextField } from '@mui/material';
import { CurrencyFormField } from '@nymproject/react/currency/CurrencyFormField';
import { ModalListItem } from 'src/components/Modals/ModalListItem';
import { SimpleModal } from 'src/components/Modals/SimpleModal';
import { DecCoin } from '@nymproject/types';
import { TokenPoolSelector, TPoolOption } from 'src/components/TokenPoolSelector';
import { ConfirmTx } from 'src/components/ConfirmTX';
import { useGetFee } from 'src/hooks/useGetFee';
import { validateAmount, validateKey } from 'src/utils';
import { simulateBondMore, simulateVestingBondMore } from 'src/requests';
import { TBondMoreArgs } from 'src/types';

export const BondMoreModal = ({
  currentBond,
  userBalance,
  hasVestingTokens,
  onConfirm,
  onClose,
  onError,
}: {
  currentBond: DecCoin;
  userBalance?: string;
  hasVestingTokens: boolean;
  onConfirm: (data: TBondMoreArgs, tokenPool: TPoolOption) => Promise<void>;
  onClose: () => void;
  onError: (e: string) => void;
}) => {
  const { fee, getFee, resetFeeState, feeError } = useGetFee();
  const [additionalBond, setAdditionalBond] = useState<DecCoin>({ amount: '0', denom: currentBond.denom });
  // const [signature, setSignature] = useState<string>('');
  const [tokenPool, setTokenPool] = useState<TPoolOption>('balance');
  const [errorAmount, setErrorAmount] = useState(false);
  const [errorSignature, setErrorSignature] = useState(false);

  useEffect(() => {
    if (feeError) {
      onError(feeError);
    }
  }, [feeError]);

  const handleOnOk = async () => {
    const errors = {
      amount: false,
      signature: false,
    };

    // if (!validateKey(signature || '', 64)) {
    //   errors.signature = true;
    // }

    if (!additionalBond?.amount) {
      errors.amount = true;
    }

    if (additionalBond && !(await validateAmount(additionalBond.amount, '1'))) {
      errors.amount = true;
    }

    if (!errors.amount) {
      const data = { additionalPledge: additionalBond };
      onConfirm(data, tokenPool);
    } else {
      setErrorAmount(errors.amount);
      setErrorSignature(errors.signature);
    }

    if (tokenPool === 'balance') {
      await getFee<TBondMoreArgs>(simulateBondMore, { additionalPledge: additionalBond });
    } else {
      await getFee<TBondMoreArgs>(simulateVestingBondMore, { additionalPledge: additionalBond });
    }
  };

  useEffect(() => {
    setErrorAmount(false);
  }, [additionalBond]);

  if (fee)
    return (
      <ConfirmTx
        header="Bond more details"
        open
        fee={fee}
        onConfirm={async () => onConfirm({ additionalPledge: additionalBond }, tokenPool)}
        onPrev={resetFeeState}
      >
        <ModalListItem label="Current bond" value={`${currentBond.amount} ${currentBond.denom}`} divider />
        <ModalListItem label="Additional bond" value={`${additionalBond?.amount} ${additionalBond?.denom}`} divider />
      </ConfirmTx>
    );

  return (
    <SimpleModal
      open
      header="Bond more"
      subHeader="Bond more tokens on your node and receive more rewards"
      okLabel="Next"
      onOk={handleOnOk}
      okDisabled={errorAmount || errorSignature}
      onClose={onClose}
    >
      <Stack gap={3}>
        <Box display="flex" gap={1}>
          {hasVestingTokens && <TokenPoolSelector disabled={false} onSelect={(pool) => setTokenPool(pool)} />}
          <CurrencyFormField
            autoFocus
            label="Bond amount"
            denom={currentBond.denom}
            onChanged={(value) => {
              setAdditionalBond(value);
              setErrorSignature(false);
            }}
            fullWidth
            validationError={errorAmount ? 'Please enter a valid amount' : undefined}
          />
        </Box>
        {/* 
        <Box>
          <TextField
            fullWidth
            label="Signature"
            value={signature}
            onChange={(e) => setSignature(e.target.value)}
            InputLabelProps={{ shrink: true }}
          />
          {errorSignature && <FormHelperText sx={{ color: 'error.main' }}>Invalid signature</FormHelperText>}
        </Box> */}

        <Box>
          <ModalListItem label="Account balance" value={userBalance?.toUpperCase() || '-'} divider />
          <ModalListItem label="Current bond" value={`${currentBond.amount} ${currentBond.denom}`} divider />
          <ModalListItem label="Est. fee for this operation will be calculated in the next page" value="" divider />
        </Box>
      </Stack>
    </SimpleModal>
  );
};
