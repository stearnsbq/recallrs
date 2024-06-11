import { writable } from 'svelte/store';
import baseTheme from '$lib/themes/base/theme.json'

console.log(baseTheme)

// import * as dome from '$app/paths'

// console.log(dome)

const theme = writable(baseTheme)

export default theme;