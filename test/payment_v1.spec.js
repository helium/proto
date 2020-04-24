const proto = require('../build')

const bob = Buffer.from(
  'ATEzTThkVWJ4eW1FM3h0aUFYc3pSa0dNbWV6TWhCUzhMaTd3RXNNb2pMZGI0U2R4YzR3Yw==',
  'base64',
)
const alice = Buffer.from(
  'ATE0OGQ4S1RSY0tBNUpLUGVrQmNLRmQ0S2Z2cHJ2RlJwakd0aXZodG1SbW5aOE1GWW5QMw==',
  'base64',
)

describe('payment_v1', () => {
  it('can be constructed', () => {
    const Payment = proto.helium.blockchain_txn_payment_v1
    const payment = Payment.create({
      payer: bob,
      payee: alice,
      amount: 10,
      fee: 1,
      nonce: 2,
      signature: 'some signature',
    })
    expect(payment.payer).toBe(bob)
    expect(payment.payee).toBe(alice)
    expect(payment.amount).toBe(10)
    expect(payment.fee).toBe(1)
    expect(payment.nonce).toBe(2)
    expect(payment.signature).toBe('some signature')
  })

  it('can be serialized', () => {
    const Txn = proto.helium.blockchain_txn
    const Payment = proto.helium.blockchain_txn_payment_v1
    const payment = Payment.create({
      payer: bob,
      payee: alice,
      amount: 10,
      fee: 1,
      nonce: 2,
      signature: 'some signature',
    })
    const txn = Txn.create({ payment })
    const serializedTxn = Txn.encode(txn).finish()
    expect(serializedTxn.toString('base64')).toBe(
      'Qn0KNAExM004ZFVieHltRTN4dGlBWHN6UmtHTW1lek1oQlM4TGk3d0VzTW9qTGRiNFNkeGM0d2MSNAExNDhkOEtUUmNLQTVKS1Bla0JjS0ZkNEtmdnBydkZScGpHdGl2aHRtUm1uWjhNRlluUDMYCiABKAIyCbKJnrIoJ2rbqw==',
    )
  })
})
