const proto = require('../build')

const bob = Buffer.from(
  'ATEzTThkVWJ4eW1FM3h0aUFYc3pSa0dNbWV6TWhCUzhMaTd3RXNNb2pMZGI0U2R4YzR3Yw==',
  'base64',
)
const alice = Buffer.from(
  'ATE0OGQ4S1RSY0tBNUpLUGVrQmNLRmQ0S2Z2cHJ2RlJwakd0aXZodG1SbW5aOE1GWW5QMw==',
  'base64',
)

describe('payment_v2', () => {
  it('can be constructed', () => {
    const PaymentTxn = proto.helium.blockchain_txn_payment_v2
    const Payment = proto.helium.payment

    const payments = [Payment.create({
      payee: alice,
      amount: 10,
    })]

    const payment = PaymentTxn.create({
      payer: bob,
      payments,
      fee: 1,
      nonce: 2,
      signature: 'some signature',
    })

    expect(payment.payer).toBe(bob)
    expect(payment.payments[0].payee).toBe(alice)
    expect(payment.payments[0].amount).toBe(10)
    expect(payment.fee).toBe(1)
    expect(payment.nonce).toBe(2)
    expect(payment.signature).toBe('some signature')
  })

  it('can be serialized', () => {
    const BlockchainTxn = proto.helium.blockchain_txn

    const PaymentTxn = proto.helium.blockchain_txn_payment_v2
    const Payment = proto.helium.payment

    const payments = [Payment.create({
      payee: alice,
      amount: 10,
    })]

    const paymentV2 = PaymentTxn.create({
      payer: bob,
      payments,
      fee: 1,
      nonce: 2,
      signature: 'some signature',
    })

    const txn = BlockchainTxn.create({ paymentV2 })
    const serializedTxn = BlockchainTxn.encode(txn).finish()

    expect(serializedTxn.toString('base64')).toBe(
      'wgF/CjQBMTNNOGRVYnh5bUUzeHRpQVhzelJrR01tZXpNaEJTOExpN3dFc01vakxkYjRTZHhjNHdjEjgKNAExNDhkOEtUUmNLQTVKS1Bla0JjS0ZkNEtmdnBydkZScGpHdGl2aHRtUm1uWjhNRlluUDMQChgBIAIqCbKJnrIoJ2rbqw==',
    )
  })
})
