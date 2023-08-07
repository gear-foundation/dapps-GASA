import { useState } from 'react';
import clsx from 'clsx';
import { encodeAddress } from '@gear-js/api';
import { useAccount, withoutCommas } from '@gear-js/react-hooks';
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
  const { account } = useAccount();
  const { decodedAddress } = account || {};

  const { altitude, weather, fuelPrice, reward, sessionId } = session;
  const playersCount = participants?.length ? participants.length + 1 : 1;
  const isRegistered = decodedAddress ? !!participants.some((participant) => participant[0] === decodedAddress) : false;

  const containerClassName = clsx(styles.container, decodedAddress ? styles.smallMargin : styles.largeMargin);

  const [registrationStatus, setRegistrationStatus] = useState<'registration' | 'success' | 'error'>('registration');

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
            {!isRegistered && registrationStatus === 'registration' && (
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
