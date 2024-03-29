/// <reference types="vite/client" />
/// <reference types="@intlify/vite-plugin-vue-i18n/client" />

declare module '*.vue' {
  import type { DefineComponent } from 'vue';
  const component: DefineComponent<{}, {}, any>;
  export default component;
}

declare module '*.md' {
  import type { DefineComponent } from 'vue';
  const component: DefineComponent<{}, {}, any>;
  export default component;
}

/* ----------------------------------------------------------------
Vite environments
---------------------------------------------------------------- */
interface ImportMetaEnv {
  readonly VITE_SERVER_ADDRESS: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
