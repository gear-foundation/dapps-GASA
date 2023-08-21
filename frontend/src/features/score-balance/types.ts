import { HexString } from '@polkadot/util/types';

export type IFTMain = {
  admins: HexString[];
  allowances: HexString[];
  balances: Array<[HexString, string]>;
  decimals: string;
  name: string;
  symbol: string;
  totalSupply: string;
};

export type IFTLogic = {
  admin: HexString;
  ftokenId: HexString;
  idToStorage: Array<[string, HexString]>;
  instructions: [];
  storageCodeHash: HexString;
  transactionStatus: [HexString, 'Failure' | 'Success'][];
};

export type IFTStorage = {
  approvals: [];
  balances: Array<[HexString, string]>;
  ftLogicId: HexString;
  transactionStatus: [];
};
