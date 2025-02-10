/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as beet from '@metaplex-foundation/beet'
import * as web3 from '@solana/web3.js'
import { SendParams, sendParamsBeet } from '../types/SendParams'

/**
 * @category Instructions
 * @category Send
 * @category generated
 */
export type SendInstructionArgs = {
  params: SendParams
}
/**
 * @category Instructions
 * @category Send
 * @category generated
 */
export const sendStruct = new beet.FixableBeetArgsStruct<
  SendInstructionArgs & {
    instructionDiscriminator: number[] /* size: 8 */
  }
>(
  [
    ['instructionDiscriminator', beet.uniformFixedSizeArray(beet.u8, 8)],
    ['params', sendParamsBeet],
  ],
  'SendInstructionArgs'
)
/**
 * Accounts required by the _send_ instruction
 *
 * @property [**signer**] sender
 * @property [] endpointProgram
 * @category Instructions
 * @category Send
 * @category generated
 */
export type SendInstructionAccounts = {
  sender: web3.PublicKey
  endpointProgram: web3.PublicKey
  anchorRemainingAccounts?: web3.AccountMeta[]
}

export const sendInstructionDiscriminator = [102, 251, 20, 187, 65, 75, 12, 69]

/**
 * Creates a _Send_ instruction.
 *
 * @param accounts that will be accessed while the instruction is processed
 * @param args to provide as instruction data to the program
 *
 * @category Instructions
 * @category Send
 * @category generated
 */
export function createSendInstruction(
  accounts: SendInstructionAccounts,
  args: SendInstructionArgs,
  programId = new web3.PublicKey('9Fmenbf7Qti4sG3hQWwifpAvGArtqtK9N96jdN19MX3u')
) {
  const [data] = sendStruct.serialize({
    instructionDiscriminator: sendInstructionDiscriminator,
    ...args,
  })
  const keys: web3.AccountMeta[] = [
    {
      pubkey: accounts.sender,
      isWritable: false,
      isSigner: true,
    },
    {
      pubkey: accounts.endpointProgram,
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
