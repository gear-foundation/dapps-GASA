import { HexString } from '@polkadot/util/types';

export type TamagotchiColor = 'Green' | 'Red' | 'Blue' | 'Purple' | 'Orange' | 'Yellow';

export type SessionData = {
  altitude: string;
  fuelPrice: string;
  payloadValue: string;
  registered: Participant;
  weather: string;
};

export type EventData = {
  alive: boolean;
  fuelLeft: string;
  halt: any;
  lastAltitude: string;
  participant: HexString;
  payload: string;
};

export type ParticipantDataType = {
  fuel: string;
  payload: string;
};

export type Participant = {
  [key: HexString]: ParticipantDataType;
};

type ParticipantMain = {
  [key: HexString]: {
    name: string;
    balance: string;
  };
};

export enum SessionStatus {
  SESSION_IS_OVER = 'SessionIsOver',
  REGISTRATION = 'Registration',
  INIT = 'Init',
}

export type LouncheStateResponse = {
  currentSession: SessionData | null;
  events: {
    [key: string]: EventData[];
  };
  name: string;
  owner: HexString;
  participants: ParticipantMain;
  sessionId: string;
  state: SessionStatus;
};
