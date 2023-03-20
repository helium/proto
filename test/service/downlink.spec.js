const proto = require('../../build')

const Downlink = proto.helium.downlink
const Region = proto.helium.region

describe('http_roaming_register_v1', () => {
  it('can be constructed', () => {
    const now = new Date().getTime()
    const txn = Downlink.http_roaming_register_v1.create({
      region: Region.US915,
      timestamp: now,
      signature: 'some signature',
    })

    expect(txn.region).toBe(Region.US915)
    expect(txn.timestamp).toBe(now)
    expect(txn.signature).toBe('some signature')
  })

  it('can be serialized', () => {
    const txn = Downlink.http_roaming_register_v1.create({
      region: Region.US915,
      timestamp: 123456789,
      signature: 'some signature',
    })

    const serializedTxn = Downlink.http_roaming_register_v1.encode(txn).finish()

    expect(serializedTxn.toString('base64')).toBe('CAAQlZrvOhoJsomesignatur')
  })
})
