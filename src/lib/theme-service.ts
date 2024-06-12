import { writable } from 'svelte/store';
import baseTheme from '$lib/themes/base/theme.json'

console.log(baseTheme)

const theme = writable(baseTheme)




function registerThemes(){

}

export function setTheme(){

}

export default theme;