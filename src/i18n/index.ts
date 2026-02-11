import { createI18n } from 'vue-i18n'
import ptBR from './pt-BR.json'
import enUS from './en-US.json'

const i18n = createI18n({
    legacy: false,
    locale: 'pt-BR',
    fallbackLocale: 'en-US',
    messages: {
        'pt-BR': ptBR,
        'en-US': enUS
    }
})

export default i18n
