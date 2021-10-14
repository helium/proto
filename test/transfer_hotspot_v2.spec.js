const proto = require('../build')

describe('transfer_hotspot_v2', () => {
  it('can be constructed', () => {
    const Txn = proto.helium.blockchain_txn_transfer_hotspot_v2

    const txn = Txn.create({
      gateway: 'some gateway',
      owner: 'some owner',
      ownerSignature: 'some owner signature',
      newOwner: 'some new owner',
      fee: 10,
      nonce: 1,
    })

    expect(txn.gateway).toBe('some gateway')
    expect(txn.owner).toBe('some owner')
    expect(txn.ownerSignature).toBe('some owner signature')
    expect(txn.newOwner).toBe('some new owner')
    expect(txn.fee).toBe(10)
    expect(txn.nonce).toBe(1)
  })

  it('can be serialized', () => {
    const BlockchainTxn = proto.helium.blockchain_txn

    const Txn = proto.helium.blockchain_txn_transfer_hotspot_v2

    const transferHotspotV2 = Txn.create({
      gateway: 'some gateway',
      owner: 'some owner',
      ownerSignature: 'some owner signature',
      newOwner: 'some new owner',
      fee: 10,
      nonce: 1,
    })

    const txn = BlockchainTxn.create({ transferHotspotV2 })
    const serializedTxn = BlockchainTxn.encode(txn).finish()

    expect(serializedTxn.toString('base64')).toBe(
      'ogIwCgiyiZ6Bq17BrBIGsomeowneGg2yiZ6jCd6uyKCdq26tIgmyiZ6d7CjCd6soCjAB',
    )
  })
})
