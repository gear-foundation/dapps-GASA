import { HexString } from '@polkadot/util/types';
import { useEffect, useState } from 'react';
import { useAtomValue, useSetAtom } from 'jotai';
import { useAccount } from '@gear-js/react-hooks';
import { useReadState } from 'hooks';
import { FT_BALANCE, FT_BALANCE_READY } from './atoms';
import { IFTLogic, IFTMain, IFTStorage } from './types';
import { getAccountBalanceById, getFTStorageIdByAccount } from './utils';
import { ADDRESS } from './consts';
import meta from './assets/ft_main.meta.txt';
import metaFTLogic from './assets/ft_logic.meta.txt';
import metaFTStorage from './assets/ft_storage.meta.txt';

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

function useFTStorage() {
  const { account } = useAccount();
  const { state: stateMain, error: errorMain } = useReadState<IFTMain>({
    programId: ADDRESS.SFT,
    meta,
  });

  const { state: stateLogic, error: errorLogic } = useReadState<IFTLogic>({
    programId: stateMain?.ftLogicId,
    meta: metaFTLogic,
  });
  const [storageId, setStorageId] = useState<HexString | undefined | null>(null);
  const [isIdExist, setIsIdExist] = useState<boolean | null>(null);

  useEffect(() => {
    if (stateLogic) {
      setStorageId(
        getFTStorageIdByAccount({
          ids: stateLogic?.idToStorage,
          accountAddress: account?.decodedAddress,
        }),
      );
    }
  }, [account, stateLogic]);

  const { state, error } = useReadState<IFTStorage>({
    programId: storageId !== null ? storageId : undefined,
    meta: metaFTStorage,
  });

  useEffect(() => {
    if (storageId !== null && stateLogic) {
      setIsIdExist(!!storageId);
    }
  }, [storageId, stateLogic, account]);

  return { state, error: error || errorLogic || errorMain, isIdExist };
}

export function useFTBalanceSync() {
  const { account } = useAccount();
  const { setBalance, setFTBalanceReady, isFTBalanceReady } = useFTBalance();
  const { state: stateStorage, isIdExist, error } = useFTStorage();

  useEffect(() => {
    if (isIdExist !== null) {
      setBalance(
        getAccountBalanceById({
          accountAddress: account?.decodedAddress,
          balances: stateStorage?.balances,
        }),
      );

      const getStorageReadState = () => {
        if (isIdExist !== null) {
          return isIdExist ? isIdExist && !!stateStorage?.balances : true;
        }
        return false;
      };

      if (!isFTBalanceReady && getStorageReadState()) {
        setFTBalanceReady(true);
      }
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [account, isFTBalanceReady, isIdExist, stateStorage?.balances]);

  return {
    errorFT: error,
  };
}
