<script setup lang="ts">
import { onMounted, ref, watch } from 'vue'
import { useRouter } from 'vue-router'
import { usePreferencesStore } from '../stores/preferences'
import { useI18n } from 'vue-i18n'
import { ArrowLeft, Loader2 } from 'lucide-vue-next'
import { getErrorMessage } from '../utils/error'

const router = useRouter()
const store = usePreferencesStore()
const { t } = useI18n()
const locale = ref('')
const licenseKey = ref('')
const licenseEmail = ref('')
const licenseError = ref('')
const licenseMessage = ref('')

onMounted(async () => {
  if (!store.preferences) await store.loadPreferences()
  locale.value = store.preferences?.locale || 'pt-BR'
  await store.refreshLicenseState()
})

watch(locale, (newVal) => {
  if (store.preferences) {
    store.savePreferences({ ...store.preferences, locale: newVal })
  }
})

async function handleOpenCheckout() {
  licenseError.value = ''
  licenseMessage.value = ''
  try {
    await store.openCheckout()
  } catch (e: unknown) {
    licenseError.value = getErrorMessage(e)
  }
}

async function handleActivate() {
  licenseError.value = ''
  licenseMessage.value = ''
  if (!licenseKey.value.trim()) {
    licenseError.value = 'Informe sua chave de licença.'
    return
  }

  try {
    await store.activateLicense(licenseKey.value.trim(), licenseEmail.value.trim() || undefined)
    licenseMessage.value = 'Licença ativada com sucesso.'
  } catch (e: unknown) {
    licenseError.value = getErrorMessage(e)
  }
}

async function handleValidate() {
  licenseError.value = ''
  licenseMessage.value = ''
  try {
    await store.validateLicense()
    licenseMessage.value = 'Licença validada.'
  } catch (e: unknown) {
    licenseError.value = getErrorMessage(e)
  }
}

async function handleDeactivate() {
  licenseError.value = ''
  licenseMessage.value = ''
  try {
    await store.deactivateLicense()
    licenseMessage.value = 'Licença removida deste dispositivo.'
    licenseKey.value = ''
    licenseEmail.value = ''
  } catch (e: unknown) {
    licenseError.value = getErrorMessage(e)
  }
}
</script>


<template>
  <div class="flex flex-col h-screen overflow-hidden bg-deep-space font-mono text-gray-100">
    <!-- Header -->
    <div
      class="flex items-center gap-3 px-4 pt-5 pb-3 border-b border-border-dark bg-dark-surface/50 shrink-0"
      data-tauri-drag-region
    >
      <button
        @click="router.push('/')"
        class="p-1.5 -ml-1.5 rounded-sm text-gray-400 hover:text-neon-green hover:bg-white/5 transition-all"
      >
        <ArrowLeft class="w-4 h-4" />
      </button>
      <span class="text-sm font-bold tracking-tight text-gray-100 uppercase">{{ t('settings.title') }}</span>
    </div>

    <!-- Content -->
    <div class="flex-1 overflow-y-auto px-4 py-4 min-h-0 space-y-4">
      <!-- Language Section -->
      <div class="card space-y-3">
        <div class="flex items-center justify-between gap-4">
          <div>
            <p class="text-sm font-medium text-gray-200 uppercase tracking-wide">{{ t('settings.language') }}</p>
            <p class="text-xs text-gray-500 mt-0.5">Interface localization</p>
          </div>
          <div class="relative w-40">
            <select
              v-model="locale"
              class="appearance-none w-full bg-deep-space border border-gray-700 rounded-sm px-3 py-1.5 text-xs text-gray-200 focus:border-neon-green/50 focus:ring-1 focus:ring-neon-green/50 outline-none transition-all"
            >
              <option value="pt-BR">Português (BR)</option>
              <option value="en-US">English (US)</option>
            </select>
            <div class="pointer-events-none absolute inset-y-0 right-0 flex items-center pr-2 text-gray-500">
              <svg class="h-3 w-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                <path stroke-linecap="round" stroke-linejoin="round" d="M19 9l-7 7-7-7" />
              </svg>
            </div>
          </div>
        </div>
      </div>

      <!-- License Section -->
      <div class="card space-y-4">
        <div class="flex items-center justify-between gap-4 border-b border-gray-800 pb-3">
          <div>
            <p class="text-sm font-medium text-gray-200 uppercase tracking-wide">LICENSE_STATUS</p>
            <div class="flex items-center gap-2 mt-1">
               <div :class="[
                 'w-2 h-2 rounded-full animate-pulse',
                 store.licenseState.tier === 'pro' ? 'bg-neon-green' : 'bg-gray-500'
               ]"></div>
               <p class="text-xs text-gray-400 font-mono">
                 {{ store.licenseState.tier.toUpperCase() }} :: {{ store.licenseState.status }}
               </p>
            </div>
          </div>
          <button
            v-if="store.licenseState.tier !== 'pro'"
            @click="handleOpenCheckout"
            class="btn btn-primary px-3 py-1.5 text-xs shadow-neon"
          >
            UPGRADE_TO_PRO
          </button>
        </div>

        <div class="space-y-3">
          <div class="space-y-1">
            <label class="text-[10px] uppercase text-gray-500 font-bold tracking-wider">License Key</label>
            <input
              v-model="licenseKey"
              type="text"
              placeholder="XXXX-XXXX-XXXX-XXXX"
              class="input-field font-mono text-xs"
            />
          </div>
          <div class="space-y-1">
            <label class="text-[10px] uppercase text-gray-500 font-bold tracking-wider">Email (Optional)</label>
            <input
              v-model="licenseEmail"
              type="email"
              placeholder="developer@example.com"
              class="input-field font-mono text-xs"
            />
          </div>
        </div>

        <div class="flex gap-2 flex-wrap pt-2">
          <button
            @click="handleActivate"
            :disabled="store.licenseLoading"
            class="btn btn-secondary px-3 py-1.5 text-xs flex-1"
            type="button"
          >
            <span v-if="store.licenseLoading" class="inline-flex items-center gap-2">
              <Loader2 class="w-3 h-3 animate-spin" />
              PROCESSING...
            </span>
            <span v-else>ACTIVATE</span>
          </button>
          <button
            @click="handleValidate"
            :disabled="store.licenseLoading"
            class="btn btn-secondary px-3 py-1.5 text-xs flex-1"
            type="button"
          >
            VALIDATE
          </button>
        </div>
        
        <div v-if="store.licenseState.license_key" class="pt-2 border-t border-gray-800">
           <button
            @click="handleDeactivate"
            :disabled="store.licenseLoading"
            class="btn btn-danger w-full px-3 py-1.5 text-xs"
            type="button"
          >
            DEACTIVATE_DEVICE
          </button>
        </div>

        <div v-if="licenseMessage" class="p-2 border border-neon-green/20 bg-neon-green/5 text-neon-green text-xs font-mono">
          > {{ licenseMessage }}
        </div>
        <div v-if="licenseError" class="p-2 border border-error-red/20 bg-error-red/5 text-error-red text-xs font-mono">
          > ERROR: {{ licenseError }}
        </div>
      </div>

      <p class="text-[10px] text-gray-600 font-mono text-center uppercase tracking-widest pt-4">
        DevGen Sys v1.0.0
      </p>
    </div>
  </div>
</template>

<style scoped>
/* Scoped styles can be minimal now as we rely on global utility classes */
</style>
