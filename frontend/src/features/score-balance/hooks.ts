import { useEffect } from 'react';
import { useAtomValue, useSetAtom } from 'jotai';
import { useAccount } from '@gear-js/react-hooks';
import { useReadState } from 'hooks';
import { FT_BALANCE, FT_BALANCE_READY } from './atoms';
import { IFTMain } from './types';
import { ADDRESS } from './consts';
import meta from './assets/fungible_token.meta.txt';

export function useFTBalance() {
  const setBalance = useSetAtom(FT_BALANCE);
  const setFTBalanceReady = useSetAtom(FT_BALANCE_READY);
  const balance = useAtomValue(FT_BALANCE);
  const isFTBalanceReady = useAtomValue(FT_BALANCE_READY);
  return {
    balance,
    setBalance,
    isFTBalanceReady,
    setFTBalanceReady,
  };
}

export function useFTBalanceSync() {
  const { account } = useAccount();
  const { setBalance, setFTBalanceReady, isFTBalanceReady } = useFTBalance();
  const { state, error, isStateRead } = useReadState<IFTMain>({
    programId: ADDRESS.SFT,
    meta,
  });

  useEffect(() => {
    if (isStateRead && !isFTBalanceReady) setFTBalanceReady(isStateRead);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isFTBalanceReady, isStateRead]);

  useEffect(() => {
    if (state?.balances && account?.decodedAddress) {
      const userBalance = state.balances.find(([address]) => address === account.decodedAddress);
      setBalance(userBalance ? userBalance[1] : '0');
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [account?.decodedAddress, state?.balances]);

  return {
    errorFT: error,
  };
}
