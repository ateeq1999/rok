export interface SlugifyOptions {
  separator?: string
  maxLength?: number
  lower?: boolean
}

export function slugify(text: string, opts: SlugifyOptions = {}): string {
  const { separator = '-', maxLength = 100, lower = true } = opts

  let slug = text
    .normalize('NFKD')
    .replace(/[\u0300-\u036f]/g, '')
    .replace(/[^\w\s-]/g, '')
    .replace(/[\s_-]+/g, separator)
    .replace(new RegExp(`^${separator}|${separator}$`, 'g'), '')

  if (lower) slug = slug.toLowerCase()

  if (slug.length > maxLength) {
    slug = slug.slice(0, maxLength)
    const lastSep = slug.lastIndexOf(separator)
    if (lastSep > maxLength * 0.6) slug = slug.slice(0, lastSep)
  }

  return slug
}
