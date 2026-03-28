export function normalizeLicenseKey(key: string) {
    return key
        .trim()
        .replace(/\s+/g, '')
        .replace(/-+/g, '-')
        .toUpperCase()
}
