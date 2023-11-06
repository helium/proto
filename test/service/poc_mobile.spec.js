const proto = require('../../build')

const PocMobile = proto.helium.poc_mobile

describe('seniority_update', () => {
  it('can be constructed', () => {
    const now = new Date().getTime()
    const txn = PocMobile.seniority_update.create({
      cbsdId: 'some_id',
      newSeniorityTimestamp: now,
      reason:
        PocMobile.seniority_update_reason
          .seniority_update_reason_heartbeat_not_seen,
    })
    expect(txn.cbsdId).toBe('some_id')
    expect(txn.newSeniorityTimestamp).toBe(now)
    expect(txn.reason).toBe(
      PocMobile.seniority_update_reason
        .seniority_update_reason_heartbeat_not_seen,
    )
  })

  it('can be serialized', () => {
    const txn = PocMobile.seniority_update.create({
      cbsdId: 'some_id',
      newSeniorityTimestamp: 123456789,
      reason:
        PocMobile.seniority_update_reason
          .seniority_update_reason_heartbeat_not_seen,
    })
    const serializedTxn = PocMobile.seniority_update.encode(txn).finish()
    expect(serializedTxn.toString('base64')).toBe('Cgdzb21lX2lkGJWa7zogAA==')
  })
})
