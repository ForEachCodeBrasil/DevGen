import { beforeEach, describe, expect, it, vi } from 'vitest'
import { createPinia, setActivePinia } from 'pinia'
import { invoke } from '@tauri-apps/api/core'
import { usePreferencesStore } from './preferences'
import type { AppPreferences } from '../types'
import i18n from '../i18n'

vi.mock('@tauri-apps/api/core', () => ({
    invoke: vi.fn()
}))

const invokeMock = vi.mocked(invoke)

describe('usePreferencesStore', () => {
    beforeEach(() => {
        setActivePinia(createPinia())
        invokeMock.mockReset()
        i18n.global.locale.value = 'pt-BR'
    })

    it('loads preferences and applies locale in i18n', async () => {
        const prefs: AppPreferences = {
            locale: 'en-US',
            quick_actions: ['quick.copy_cpf_masked'],
            history: [],
            generator_last_options: {},
            license_state: {
                tier: 'free',
                status: 'inactive',
                license_key: null,
                instance_id: null,
                customer_email: null,
                last_validated_at: null
            }
        }
        invokeMock.mockResolvedValueOnce(prefs)

        const store = usePreferencesStore()
        await store.loadPreferences()

        expect(invokeMock).toHaveBeenCalledWith('get_preferences')
        expect(store.preferences).toEqual(prefs)
        expect(i18n.global.locale.value).toBe('en-US')
        expect(store.loading).toBe(false)
    })

    it('saves preferences and syncs locale', async () => {
        const prefs: AppPreferences = {
            locale: 'pt-BR',
            quick_actions: ['quick.copy_password'],
            history: ['password: abc123'],
            generator_last_options: { password: { length: 16 } },
            license_state: {
                tier: 'pro',
                status: 'active',
                license_key: 'abc',
                instance_id: 'inst',
                customer_email: 'test@example.com',
                last_validated_at: '2025-01-01T00:00:00Z'
            }
        }
        invokeMock.mockResolvedValueOnce(undefined)

        const store = usePreferencesStore()
        await store.savePreferences(prefs)

        expect(invokeMock).toHaveBeenCalledWith('save_preferences', { prefs })
        expect(store.preferences).toEqual(prefs)
        expect(i18n.global.locale.value).toBe('pt-BR')
    })
})
