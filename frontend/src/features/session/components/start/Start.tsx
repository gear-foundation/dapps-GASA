import { useEffect, useState } from 'react';
import clsx from 'clsx';
import {
  HexString,
  MessageQueued,
  MessageWaited,
  UserMessageRead,
  UserMessageSent,
  UserMessageSentData,
  decodeAddress,
  encodeAddress,
} from '@gear-js/api';
import { useAtomValue } from 'jotai';
import { CURRENT_CONTRACT_ADDRESS_ATOM } from 'atoms';
import { useAccount, useApi, withoutCommas } from '@gear-js/react-hooks';
import { UnsubscribePromise } from '@polkadot/api/types';
import src from 'assets/images/earth.gif';
import { Container } from 'components';
import { Participant, Session } from '../../types';
import { Traits } from '../traits';
import { Form } from '../form';
import styles from './Start.module.scss';
import { ParticipantsTable } from '../participants-table';
import { MaximumPlayersWarning } from '../maximum-players-warning';
import { SuccessfullyRegisteredInfo } from '../successfully-registered-info';

type Props = {
  participants: Participant[];
  session: Session;
  isUserAdmin: boolean;
  userAddress: string;
};

function Start({ participants, session, isUserAdmin, userAddress }: Props) {
  const currentContractAddress = useAtomValue(CURRENT_CONTRACT_ADDRESS_ATOM);
  const { api } = useApi();
  const { account } = useAccount();
  const { decodedAddress } = account || {};

  const { altitude, weather, fuelPrice, reward, sessionId } = session;
  const playersCount = participants?.length ? participants.length + 1 : 1;
  const isRegistered = decodedAddress ? !!participants.some((participant) => participant[0] === decodedAddress) : false;

  const containerClassName = clsx(styles.container, decodedAddress ? styles.smallMargin : styles.largeMargin);

  const [registrationStatus, setRegistrationStatus] = useState<'registration' | 'success' | 'error'>('registration');

  const handleEvents = ({ data }: UserMessageSent) => {};

  useEffect(() => {
    let unsub: UnsubscribePromise | undefined;

    if (api && decodedAddress) {
      unsub = api.gearEvents.subscribeToGearEvent('UserMessageSent', handleEvents);
    }

    return () => {
      if (unsub) unsub.then((unsubCallback) => unsubCallback());
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [api, decodedAddress]);

  return (
    <div className={styles.mainContainer}>
      <div>
        <header className={styles.header}>
          <h2 className={styles.heading}>Session #{sessionId}</h2>

          <div>
            <p className={styles.registration}>Registration</p>
            <p className={styles.subheading}>Rockets ({playersCount}/4). Waiting for other players...</p>
          </div>
        </header>

        <Container className={containerClassName}>
          {isUserAdmin && (
            <ParticipantsTable
              data={[
                {
                  id: userAddress,
                  playerAddress: encodeAddress(userAddress),
                },
                ...participants.map((item) => ({
                  id: item[0],
                  playerAddress: encodeAddress(item[0]),
                })),
              ]}
              userAddress={userAddress}
            />
          )}
          <Traits altitude={altitude} weather={weather} fuelPrice={fuelPrice} reward={reward} />

          <footer>
            {isRegistered && <SuccessfullyRegisteredInfo />}
            {!isRegistered && registrationStatus === 'error' && !isUserAdmin && <MaximumPlayersWarning />}
            {isRegistered && registrationStatus === 'registration' && (
              <Form
                weather={weather}
                defaultDeposit={withoutCommas('0')}
                isAdmin={isUserAdmin}
                setRegistrationStatus={setRegistrationStatus}
              />
            )}
          </footer>
        </Container>
      </div>

      <div className={styles.imageWrapper}>
        <img src={src} alt="" className={styles.image} />
      </div>
    </div>
  );
}

export { Start };
