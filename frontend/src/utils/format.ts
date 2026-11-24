export function formatDate(isoDate: string): string {
  return isoDate.replaceAll('-', '.')
}

export function formatIndex(index: number): string {
  return String(index + 1).padStart(2, '0')
}

export function getYear(isoDate: string): string {
  return isoDate.slice(0, 4)
}
