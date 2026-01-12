import {
  defineConfig,
  presetUno,
  presetTypography,
  presetIcons,
} from "unocss";

export default defineConfig({
  presets: [
    presetUno({
      dark: "media",
    }),
    presetTypography(),
    presetIcons(),
  ],
  theme: {
    colors: {
      accent: "#3b82f6",
      border: {
        light: "#e5e7eb",
        dark: "#27272a",
      },
    },
  },
  shortcuts: {
    "btn": "px-4 py-2 rounded-md transition-colors duration-200 cursor-pointer disabled:cursor-not-allowed disabled:opacity-50 text-sm font-medium",
    "btn-primary": "bg-accent text-white hover:bg-blue-600 active:bg-blue-700 dark:bg-accent dark:text-white dark:hover:bg-blue-600",
    "btn-outline": "border-1 bg-transparent text-neutral-700 border-neutral-300 hover:bg-neutral-100 hover:text-neutral-900 dark:text-neutral-200 dark:border-neutral-600 dark:hover:bg-neutral-800 dark:hover:text-white",
    "input-base": "bg-white dark:bg-neutral-900 text-neutral-900 dark:text-neutral-100 border border-border-light dark:border-border-dark rounded-md px-3 py-1.5 focus:outline-none focus:ring-2 focus:ring-accent",
  },
});
