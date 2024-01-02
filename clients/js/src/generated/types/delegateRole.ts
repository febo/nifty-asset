/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import { Serializer, scalarEnum } from '@metaplex-foundation/umi/serializers';

export enum DelegateRole {
  None,
  Transfer,
  Lock,
  Burn,
}

export type DelegateRoleArgs = DelegateRole;

export function getDelegateRoleSerializer(): Serializer<
  DelegateRoleArgs,
  DelegateRole
> {
  return scalarEnum<DelegateRole>(DelegateRole, {
    description: 'DelegateRole',
  }) as Serializer<DelegateRoleArgs, DelegateRole>;
}
