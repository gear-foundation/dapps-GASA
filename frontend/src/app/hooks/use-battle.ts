import { useApp, useLounch } from 'app/context';
import { useEffect } from 'react';
import type { LouncheStateResponse } from 'app/types/battles';
import { useAccount, useReadFullState } from '@gear-js/react-hooks';
import { useMetadata } from './use-metadata';
import metaBattle from 'assets/meta/meta.txt';
import { ENV } from 'app/consts';
import { useSendMessage } from './useSendMessage';

function useReadLouncheState<T>() {
  const { metadata } = useMetadata(metaBattle);
  return useReadFullState<T>(ENV.contract, metadata);
}

export function useInitLouncheData() {
  const { setIsAdmin } = useApp();
  const { setLaunch, setSessionIsOver } = useLounch();
  const { account } = useAccount();
  const { state } = useReadLouncheState<LouncheStateResponse>();

  useEffect(() => {
    if (state && account) {
      setIsAdmin(state.owner === account.decodedAddress);
      setLaunch(state);

      if (state.state === 'SessionIsOver') {
        setSessionIsOver(true);
      } else {
        setSessionIsOver(false);
      }
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [state, account]);
}

export function useLaunchMessage() {
  const { metadata } = useMetadata(metaBattle);
  return useSendMessage(ENV.contract, metadata);
}
