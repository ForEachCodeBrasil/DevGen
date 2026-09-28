import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/core'
import type { AppPreferences } from '../types'
import i18n from '../i18n'

export const usePreferencesStore = defineStore('preferences', {
    state: () => ({
        preferences: null as AppPreferences | null,
        loading: false
    }),
    actions: {
        async loadPreferences() {
            this.loading = true
            try {
                this.preferences = await invoke('get_preferences')
                if (this.preferences?.locale) {
                    // @ts-ignore
                    i18n.global.locale.value = this.preferences.locale
                }
            } catch (e) {
                console.error('Failed to load preferences', e)
            } finally {
                this.loading = false
            }
        },
        async savePreferences(newPrefs: AppPreferences) {
            try {
                await invoke('save_preferences', { prefs: newPrefs })
                this.preferences = newPrefs
                if (this.preferences?.locale) {
                    // @ts-ignore
                    i18n.global.locale.value = this.preferences.locale
                }
            } catch (e) {
                console.error('Failed to save preferences', e)
                throw e
            }
        }
    }
})
