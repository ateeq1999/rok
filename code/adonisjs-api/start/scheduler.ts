import CleanupExpiredOtpsJob from '#jobs/cleanup_expired_otps_job'
import CleanupExpiredOrdersJob from '#jobs/cleanup_expired_orders_job'

// Delete OTP records that expired >24h ago — keeps otp_codes table lean
await CleanupExpiredOtpsJob.schedule({}).cron('0 2 * * *')

// Cancel pending orders whose expiresAt has passed — frees reserved ticket capacity
await CleanupExpiredOrdersJob.schedule({}).cron('*/15 * * * *')