/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import {
  ClusterFilter,
  Context,
  Program,
  PublicKey,
} from '@metaplex-foundation/umi';
import { getProxyErrorFromCode, getProxyErrorFromName } from '../errors';

export const PROXY_PROGRAM_ID =
  'Proxy11111111111111111111111111111111111111' as PublicKey<'Proxy11111111111111111111111111111111111111'>;

export function createProxyProgram(): Program {
  return {
    name: 'proxy',
    publicKey: PROXY_PROGRAM_ID,
    getErrorFromCode(code: number, cause?: Error) {
      return getProxyErrorFromCode(code, this, cause);
    },
    getErrorFromName(name: string, cause?: Error) {
      return getProxyErrorFromName(name, this, cause);
    },
    isOnCluster() {
      return true;
    },
  };
}

export function getProxyProgram<T extends Program = Program>(
  context: Pick<Context, 'programs'>,
  clusterFilter?: ClusterFilter
): T {
  return context.programs.get<T>('proxy', clusterFilter);
}

export function getProxyProgramId(
  context: Pick<Context, 'programs'>,
  clusterFilter?: ClusterFilter
): PublicKey {
  return context.programs.getPublicKey(
    'proxy',
    PROXY_PROGRAM_ID,
    clusterFilter
  );
}
