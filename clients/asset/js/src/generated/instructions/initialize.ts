/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import {
  Context,
  Option,
  OptionOrNullable,
  Pda,
  PublicKey,
  Signer,
  TransactionBuilder,
  none,
  transactionBuilder,
} from '@metaplex-foundation/umi';
import {
  Serializer,
  bytes,
  mapSerializer,
  option,
  struct,
  u32,
  u8,
} from '@metaplex-foundation/umi/serializers';
import {
  ResolvedAccount,
  ResolvedAccountsWithIndices,
  getAccountMetasAndSigners,
} from '../shared';
import {
  ExtensionType,
  ExtensionTypeArgs,
  getExtensionTypeSerializer,
} from '../types';

// Accounts.
export type InitializeInstructionAccounts = {
  /** Asset account */
  asset: Signer;
  /** The account paying for the storage fees */
  payer?: Signer;
  /** The system program */
  systemProgram?: PublicKey | Pda;
};

// Data.
export type InitializeInstructionData = {
  discriminator: number;
  extensionType: ExtensionType;
  length: number;
  data: Option<Uint8Array>;
};

export type InitializeInstructionDataArgs = {
  extensionType: ExtensionTypeArgs;
  length: number;
  data?: OptionOrNullable<Uint8Array>;
};

export function getInitializeInstructionDataSerializer(): Serializer<
  InitializeInstructionDataArgs,
  InitializeInstructionData
> {
  return mapSerializer<
    InitializeInstructionDataArgs,
    any,
    InitializeInstructionData
  >(
    struct<InitializeInstructionData>(
      [
        ['discriminator', u8()],
        ['extensionType', getExtensionTypeSerializer()],
        ['length', u32()],
        ['data', option(bytes({ size: u32() }))],
      ],
      { description: 'InitializeInstructionData' }
    ),
    (value) => ({ ...value, discriminator: 3, data: value.data ?? none() })
  ) as Serializer<InitializeInstructionDataArgs, InitializeInstructionData>;
}

// Args.
export type InitializeInstructionArgs = InitializeInstructionDataArgs;

// Instruction.
export function initialize(
  context: Pick<Context, 'programs'>,
  input: InitializeInstructionAccounts & InitializeInstructionArgs
): TransactionBuilder {
  // Program ID.
  const programId = context.programs.getPublicKey(
    'asset',
    'AssetGtQBTSgm5s91d1RAQod5JmaZiJDxqsgtqrZud73'
  );

  // Accounts.
  const resolvedAccounts = {
    asset: {
      index: 0,
      isWritable: true as boolean,
      value: input.asset ?? null,
    },
    payer: {
      index: 1,
      isWritable: true as boolean,
      value: input.payer ?? null,
    },
    systemProgram: {
      index: 2,
      isWritable: false as boolean,
      value: input.systemProgram ?? null,
    },
  } satisfies ResolvedAccountsWithIndices;

  // Arguments.
  const resolvedArgs: InitializeInstructionArgs = { ...input };

  // Default values.
  if (!resolvedAccounts.systemProgram.value) {
    if (resolvedAccounts.payer.value) {
      resolvedAccounts.systemProgram.value = context.programs.getPublicKey(
        'systemProgram',
        '11111111111111111111111111111111'
      );
      resolvedAccounts.systemProgram.isWritable = false;
    }
  }

  // Accounts in order.
  const orderedAccounts: ResolvedAccount[] = Object.values(
    resolvedAccounts
  ).sort((a, b) => a.index - b.index);

  // Keys and Signers.
  const [keys, signers] = getAccountMetasAndSigners(
    orderedAccounts,
    'programId',
    programId
  );

  // Data.
  const data = getInitializeInstructionDataSerializer().serialize(
    resolvedArgs as InitializeInstructionDataArgs
  );

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
