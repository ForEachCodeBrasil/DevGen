import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/core'
import type { AppPreferences, LicenseState } from '../types'
import i18n from '../i18n'

export const usePreferencesStore = defineStore('preferences', {
    state: () => ({
        preferences: null as AppPreferences | null,
        loading: false,
        licenseLoading: false
    }),
    getters: {
        licenseState: (state): LicenseState => {
            return (
                state.preferences?.license_state ?? {
                    tier: 'free',
                    status: 'inactive',
                    license_key: null,
                    instance_id: null,
                    customer_email: null,
                    last_validated_at: null
                }
            )
        },
        isPro(): boolean {
            return this.licenseState.tier === 'pro'
        }
    },
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
        async refreshLicenseState() {
            this.licenseLoading = true
            try {
                const next = await invoke<LicenseState>('get_access_tier')
                if (this.preferences) {
                    this.preferences = {
                        ...this.preferences,
                        license_state: next
                    }
                }
                return next
            } finally {
                this.licenseLoading = false
            }
        },
        async openCheckout() {
            await invoke('open_checkout')
        },
        async activateLicense(key: string, email?: string) {
            this.licenseLoading = true
            try {
                const next = await invoke<LicenseState>('activate_license', {
                    req: { key, email: email || null }
                })
                if (this.preferences) {
                    this.preferences = {
                        ...this.preferences,
                        license_state: next
                    }
                }
                return next
            } finally {
                this.licenseLoading = false
            }
        },
        async validateLicense() {
            this.licenseLoading = true
            try {
                const next = await invoke<LicenseState>('validate_license')
                if (this.preferences) {
                    this.preferences = {
                        ...this.preferences,
                        license_state: next
                    }
                }
                return next
            } finally {
                this.licenseLoading = false
            }
        },
        async deactivateLicense() {
            this.licenseLoading = true
            try {
                await invoke('deactivate_license')
                if (this.preferences) {
                    this.preferences = {
                        ...this.preferences,
                        license_state: {
                            tier: 'free',
                            status: 'inactive',
                            license_key: null,
                            instance_id: null,
                            customer_email: null,
                            last_validated_at: null
                        }
                    }
                }
            } finally {
                this.licenseLoading = false
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
