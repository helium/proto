const proto = require('../build')

const bob = Buffer.from(
  'ATEzTThkVWJ4eW1FM3h0aUFYc3pSa0dNbWV6TWhCUzhMaTd3RXNNb2pMZGI0U2R4YzR3Yw==',
  'base64',
)

const tokenType = proto.helium.blockchain_token_type_v1

describe('payment_v2', () => {
  it('can be constructed', () => {
    const RedeemTxn = proto.helium.blockchain_txn_token_redeem_v1

    const txn = RedeemTxn.create({
      tokenType: tokenType.mobile,
      account: bob,
      amount: 100,
      signature: 'some signature',
      nonce: 2,
      fee: 1,
    })

    expect(txn.tokenType).toBe(tokenType.mobile)
    expect(txn.account).toBe(bob)
    expect(txn.amount).toBe(100)
    expect(txn.signature).toBe('some signature')
    expect(txn.nonce).toBe(2)
    expect(txn.fee).toBe(1)
  })

  it('can be serialized', () => {
    const RedeemTxn = proto.helium.blockchain_txn_token_redeem_v1
    const BlockchainTxn = proto.helium.blockchain_txn

    const tokenRedeem = RedeemTxn.create({
      tokenType: tokenType.mobile,
      account: bob,
      amount: 100,
      signature: 'some signature',
      nonce: 2,
      fee: 1,
    })

    const txn = BlockchainTxn.create({ tokenRedeem })
    const serializedTxn = BlockchainTxn.encode(txn).finish()

    expect(serializedTxn.toString('base64')).toBe(
      'ygJJCAISNAExM004ZFVieHltRTN4dGlBWHN6UmtHTW1lek1oQlM4TGk3d0VzTW9qTGRiNFNkeGM0d2MYZCIJsomesignaturKAIwAQ==',
    )
  })
})
