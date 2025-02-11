/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as beet from '@metaplex-foundation/beet'
import * as web3 from '@solana/web3.js'
import {
  InitBridgeParams,
  initBridgeParamsBeet,
} from '../types/InitBridgeParams'

/**
 * @category Instructions
 * @category InitBridge
 * @category generated
 */
export type InitBridgeInstructionArgs = {
  params: InitBridgeParams
}
/**
 * @category Instructions
 * @category InitBridge
 * @category generated
 */
export const initBridgeStruct = new beet.BeetArgsStruct<
  InitBridgeInstructionArgs & {
    instructionDiscriminator: number[] /* size: 8 */
  }
>(
  [
    ['instructionDiscriminator', beet.uniformFixedSizeArray(beet.u8, 8)],
    ['params', initBridgeParamsBeet],
  ],
  'InitBridgeInstructionArgs'
)
/**
 * Accounts required by the _initBridge_ instruction
 *
 * @property [_writable_, **signer**] authority
 * @property [_writable_] bridge
 * @property [_writable_] solVault
 * @category Instructions
 * @category InitBridge
 * @category generated
 */
export type InitBridgeInstructionAccounts = {
  authority: web3.PublicKey
  bridge: web3.PublicKey
  solVault: web3.PublicKey
  systemProgram?: web3.PublicKey
  anchorRemainingAccounts?: web3.AccountMeta[]
}

export const initBridgeInstructionDiscriminator = [
  94, 43, 213, 33, 108, 54, 253, 169,
]

/**
 * Creates a _InitBridge_ instruction.
 *
 * @param accounts that will be accessed while the instruction is processed
 * @param args to provide as instruction data to the program
 *
 * @category Instructions
 * @category InitBridge
 * @category generated
 */
export function createInitBridgeInstruction(
  accounts: InitBridgeInstructionAccounts,
  args: InitBridgeInstructionArgs,
  programId = new web3.PublicKey('DD12vMyLdwszDCAzLhsUPwBmzJXv611dUCPhqwpZQYG4')
) {
  const [data] = initBridgeStruct.serialize({
    instructionDiscriminator: initBridgeInstructionDiscriminator,
    ...args,
  })
  const keys: web3.AccountMeta[] = [
    {
      pubkey: accounts.authority,
      isWritable: true,
      isSigner: true,
    },
    {
      pubkey: accounts.bridge,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.solVault,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.systemProgram ?? web3.SystemProgram.programId,
      isWritable: false,
      isSigner: false,
    },
  ]

  if (accounts.anchorRemainingAccounts != null) {
    for (const acc of accounts.anchorRemainingAccounts) {
      keys.push(acc)
    }
  }

  const ix = new web3.TransactionInstruction({
    programId,
    keys,
    data,
  })
  return ix
}
