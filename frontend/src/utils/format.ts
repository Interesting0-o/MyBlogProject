export function formatDate(isoDate: string): string {
  return isoDate.replaceAll('-', '.')
}

export function getYear(isoDate: string): string {
  return isoDate.slice(0, 4)
}

export function formatMonthDay(isoDate: string): string {
  return isoDate.slice(5).replaceAll('-', '.')
}
