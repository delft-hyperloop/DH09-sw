import type { Config } from 'tailwindcss'
import { delft_hyperloop_dark } from './src/delft_hyperloop_dark'

export default {
	darkMode: 'class',
	content: ['./src/**/*.{html,js,svelte,ts}'],
	theme: {
		extend: {},
	},
	plugins: [

	],
} satisfies Config;
