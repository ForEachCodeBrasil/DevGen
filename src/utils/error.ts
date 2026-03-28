export function getErrorMessage(error: unknown, fallback = 'Ocorreu um erro inesperado.') {
    if (typeof error === 'string' && error.trim()) {
        return error
    }

    if (error instanceof Error && error.message) {
        return error.message
    }

    if (error && typeof error === 'object') {
        const maybeMessage = (error as { message?: unknown }).message
        if (typeof maybeMessage === 'string' && maybeMessage.trim()) {
            return maybeMessage
        }
    }

    return fallback
}
