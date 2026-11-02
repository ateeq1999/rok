import type { HttpContext } from '@adonisjs/core/http'
import hash from '@adonisjs/core/services/hash'
import UserTransformer from '#transformers/user_transformer'
import PointsTransformer from '#transformers/helpers/points_transformer'
import { changePasswordValidator, updateProfileValidator } from '#validators/user'
import { success, unprocessable } from '#helpers/response'

export default class ProfileController {
  async show({ auth, response }: HttpContext) {
    const data = new UserTransformer(auth.getUserOrFail()).toObject()
    return success(response, data)
  }

  async update({ auth, request, response }: HttpContext) {
    const user = auth.getUserOrFail()
    const data = await request.validateUsing(updateProfileValidator)

    if (data.fullName !== undefined) user.fullName = data.fullName
    if (data.phone !== undefined) user.phone = data.phone

    await user.save()

    return success(response, UserTransformer.transform(user))
  }

  async points({ auth, response }: HttpContext) {
    const user = auth.getUserOrFail()
    return success(response, PointsTransformer.transform({ balance: user.points ?? 0, currency: 'JIONI' }))
  }

  async changePassword({ auth, request, response }: HttpContext) {
    const user = auth.getUserOrFail()
    const data = await request.validateUsing(changePasswordValidator)

    const isValid = await hash.verify(user.password, data.currentPassword)
    if (!isValid) {
      return unprocessable(response, 'Current password is incorrect')
    }

    user.password = await hash.make(data.password)
    await user.save()

    return success(response, { message: 'Password changed successfully' })
  }

  async attendance({ response }: HttpContext) {
    return success(response, { events: [], total: 0 })
  }
}
