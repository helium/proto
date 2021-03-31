const proto = require('../build')

const bob = Buffer.from(
  'ATEzTThkVWJ4eW1FM3h0aUFYc3pSa0dNbWV6TWhCUzhMaTd3RXNNb2pMZGI0U2R4YzR3Yw==',
  'base64',
)
const alice = Buffer.from(
  'ATE0OGQ4S1RSY0tBNUpLUGVrQmNLRmQ0S2Z2cHJ2RlJwakd0aXZodG1SbW5aOE1GWW5QMw==',
  'base64',
)

const makeAssert = () => {
    const AssertTxn = proto.helium.blockchain_txn_assert_location_v2

    return AssertTxn.create({
      gateway: bob,
      owner: alice,
      payer: bob,
      ownerSignature: 'some signature',
      payerSignature: 'some other signature',
      location: 'location',
      nonce: 1,
      gain: 2,
      elevation: 3,
      fee: 4,
      stakingFee: 5,
    })
}

describe('assert_location_v2', () => {
  it('can be constructed', () => {
    const assert = makeAssert()

    expect(assert.gateway).toBe(bob)
    expect(assert.owner).toBe(alice)
    expect(assert.payer).toBe(bob)
    expect(assert.ownerSignature).toBe('some signature')
    expect(assert.payerSignature).toBe('some other signature')
    expect(assert.location).toBe('location')
    expect(assert.nonce).toBe(1)
    expect(assert.gain).toBe(2)
    expect(assert.elevation).toBe(3)
    expect(assert.fee).toBe(4)
    expect(assert.stakingFee).toBe(5)
  })

  it('can be serialized', () => {
    const BlockchainTxn = proto.helium.blockchain_txn

    const assert = makeAssert()

    const txn = BlockchainTxn.create({ assertLocationV2: assert })
    const serializedTxn = BlockchainTxn.encode(txn).finish()

    expect(serializedTxn.toString('base64')).toBe(
      'mgLQAQo0ATEzTThkVWJ4eW1FM3h0aUFYc3pSa0dNbWV6TWhCUzhMaTd3RXNNb2pMZGI0U2R4YzR3YxI0ATE0OGQ4S1RSY0tBNUpLUGVrQmNLRmQ0S2Z2cHJ2RlJwakd0aXZodG1SbW5aOE1GWW5QMxo0ATEzTThkVWJ4eW1FM3h0aUFYc3pSa0dNbWV6TWhCUzhMaTd3RXNNb2pMZGI0U2R4YzR3YyIJsomesignaturKg2yiZ6i2F6uyKCdq26tMghsb2NhdGlvbjgBQAJIA1AFWAQ=',
    )
  })
})
