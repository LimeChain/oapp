/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as beet from '@metaplex-foundation/beet'
import { Tx, txBeet } from './Tx'
import { XChainMsgType, xChainMsgTypeBeet } from './XChainMsgType'
export type XFER = {
  nonce: beet.bignum
  transaction: Tx
  trader: number[] /* size: 20 */
  symbol: number[] /* size: 32 */
  quantity: beet.bignum[] /* size: 4 */
  timestamp: number
  customdata: number[] /* size: 28 */
  messageType: XChainMsgType
}

/**
 * @category userTypes
 * @category generated
 */
export const xFERBeet = new beet.BeetArgsStruct<XFER>(
  [
    ['nonce', beet.u64],
    ['transaction', txBeet],
    ['trader', beet.uniformFixedSizeArray(beet.u8, 20)],
    ['symbol', beet.uniformFixedSizeArray(beet.u8, 32)],
    ['quantity', beet.uniformFixedSizeArray(beet.u64, 4)],
    ['timestamp', beet.u32],
    ['customdata', beet.uniformFixedSizeArray(beet.u8, 28)],
    ['messageType', xChainMsgTypeBeet],
  ],
  'XFER'
)
